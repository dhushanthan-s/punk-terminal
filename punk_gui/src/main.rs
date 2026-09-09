mod render;

use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use punk_terminal::{PtySession, TerminalGrid};
use punk_utils::config::load_gui_config;
use render::TerminalRenderer;
use vte::Parser;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{KeyCode, ModifiersState, NamedKey, PhysicalKey};
use winit::window::{Window, WindowId};

const SESSION_MARKER_KEY: &str = "PUNK_SESSION";
const SESSION_MARKER_VALUE: &str = "1";

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    let inside_punk_session = env::var(SESSION_MARKER_KEY)
        .map(|v| v == SESSION_MARKER_VALUE)
        .unwrap_or(false);

    match parse_command_mode(&args, inside_punk_session) {
        CommandMode::LaunchGui => {}
        CommandMode::Help => {
            print_help();
            return Ok(());
        }
        CommandMode::Version => {
            println!("{}", env!("CARGO_PKG_VERSION"));
            return Ok(());
        }
        CommandMode::InsideSession(args) => {
            run_inside_session_command(&args)?;
            return Ok(());
        }
        CommandMode::DeniedOutside(args) => {
            eprintln!(
                "The `punk {}` command is only available inside a punk-terminal session.",
                args.join(" ")
            );
            eprintln!("Run `punk` to launch punk-terminal.");
            return Err("outside-session command denied".to_string());
        }
    }

    let mut elb = EventLoop::<Vec<u8>>::with_user_event();
    let event_loop = elb.build().map_err(|e| format!("event loop: {e}"))?;
    let proxy = event_loop.create_proxy();
    let gui_cfg = load_gui_config();
    let mut app = App {
        proxy,
        window_id: None,
        window: None,
        renderer: None,
        grid: None,
        parser: Parser::new(),
        pty: None,
        mods: ModifiersState::default(),
        gui_font_family: gui_cfg.font_family,
        gui_font_size: gui_cfg.font_size,
    };
    event_loop
        .run_app(&mut app)
        .map_err(|e| format!("run_app: {e}"))
}

#[derive(Debug, PartialEq, Eq)]
enum CommandMode {
    LaunchGui,
    Help,
    Version,
    InsideSession(Vec<String>),
    DeniedOutside(Vec<String>),
}

fn parse_command_mode(args: &[String], inside_punk_session: bool) -> CommandMode {
    if args.is_empty() {
        return CommandMode::LaunchGui;
    }

    if args.len() == 1 {
        match args[0].as_str() {
            "--help" | "-h" => return CommandMode::Help,
            "--version" | "-V" => return CommandMode::Version,
            _ => {}
        }
    }

    if inside_punk_session {
        CommandMode::InsideSession(args.to_vec())
    } else {
        CommandMode::DeniedOutside(args.to_vec())
    }
}

fn print_help() {
    println!(
        "\
punk - launch Punk Terminal

USAGE:
    punk
    punk --help
    punk --version

NOTES:
    - Running `punk` launches the Punk Terminal GUI.
    - Non-launch command functionality is available only inside Punk Terminal sessions.
"
    );
}

fn run_inside_session_command(args: &[String]) -> Result<(), String> {
    // Placeholder entrypoint for inside-session command functionality.
    // The command gate allows this path only when PUNK_SESSION=1.
    eprintln!("inside-session command mode: {}", args.join(" "));
    Ok(())
}

struct App {
    proxy: winit::event_loop::EventLoopProxy<Vec<u8>>,
    window_id: Option<WindowId>,
    window: Option<Arc<Window>>,
    renderer: Option<TerminalRenderer>,
    grid: Option<TerminalGrid>,
    parser: Parser,
    pty: Option<Arc<Mutex<PtySession>>>,
    mods: ModifiersState,
    gui_font_family: String,
    gui_font_size: f32,
}

impl App {
    fn grid_dims_for_window(r: &TerminalRenderer, window: &Window) -> (u16, u16) {
        const MAX_COLS: u32 = 512;
        const MAX_ROWS: u32 = 256;
        let sz = window.inner_size();
        let (cw, ch) = r.cell_pixel_size();
        let cols = ((sz.width as f32 / cw).floor() as u32).clamp(1, MAX_COLS) as u16;
        let rows = ((sz.height as f32 / ch).floor() as u32).clamp(1, MAX_ROWS) as u16;
        (cols, rows)
    }

    fn setup_grid_pty(&mut self, window: &Window) -> Result<(), String> {
        let r = self
            .renderer
            .as_mut()
            .ok_or_else(|| "no renderer".to_string())?;
        let (cols, rows) = Self::grid_dims_for_window(r, window);
        match self.grid.as_mut() {
            Some(g) => g.resize(cols, rows),
            None => {
                self.grid = Some(TerminalGrid::new(cols, rows));
            }
        }
        let pty = PtySession::spawn_shell(cols, rows).map_err(|e| format!("pty: {e}"))?;
        let pty = Arc::new(Mutex::new(pty));
        let pty_read = Arc::clone(&pty);
        let proxy = self.proxy.clone();
        thread::spawn(move || {
            let mut buf = [0u8; 16384];
            loop {
                let n = match pty_read.lock() {
                    Ok(mut g) => g.try_read(&mut buf),
                    Err(_) => break,
                };
                match n {
                    Ok(0) => thread::sleep(Duration::from_millis(2)),
                    Ok(n) => {
                        let _ = proxy.send_event(buf[..n].to_vec());
                    }
                    Err(_) => break,
                }
            }
        });
        self.pty = Some(pty);
        Ok(())
    }

    fn resize_terminal(&mut self, window: &Window) {
        let Some(r) = self.renderer.as_mut() else {
            return;
        };
        r.resize(window.inner_size());
        let (cols, rows) = Self::grid_dims_for_window(r, window);
        if let Some(g) = self.grid.as_mut() {
            g.resize(cols, rows);
        }
        if let Some(p) = self.pty.as_ref() {
            if let Ok(mut s) = p.lock() {
                let _ = s.resize(cols, rows);
            }
        }
    }

    fn write_pty(&self, bytes: &[u8]) {
        let Some(p) = self.pty.as_ref() else {
            return;
        };
        if let Ok(mut s) = p.lock() {
            let _ = s.write_all(bytes);
        }
    }
}

impl ApplicationHandler<Vec<u8>> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let win = match event_loop.create_window(
            Window::default_attributes()
                .with_title("punk")
                .with_inner_size(winit::dpi::LogicalSize::new(960.0, 540.0)),
        ) {
            Ok(w) => Arc::new(w),
            Err(e) => {
                eprintln!("window: {e}");
                event_loop.exit();
                return;
            }
        };
        self.window_id = Some(win.id());
        let mut renderer = match TerminalRenderer::new(
            Arc::clone(&win),
            &self.gui_font_family,
            self.gui_font_size,
        ) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("gpu: {e}");
                event_loop.exit();
                return;
            }
        };
        renderer.resize(win.inner_size());
        self.renderer = Some(renderer);
        self.window = Some(win.clone());
        if let Err(e) = self.setup_grid_pty(&win) {
            eprintln!("pty: {e}");
            event_loop.exit();
            return;
        }
        win.request_redraw();
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: Vec<u8>) {
        if let Some(grid) = self.grid.as_mut() {
            self.parser.advance(grid, &event);
            grid.mark_dirty();
        }
        if let Some(w) = self.window.as_ref() {
            w.request_redraw();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if self.window_id != Some(window_id) {
            return;
        }
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::ModifiersChanged(m) => {
                self.mods = m.state();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed {
                    // Keep the cursor solid while typing instead of blinking out mid-keystroke.
                    if let Some(r) = self.renderer.as_mut() {
                        r.reset_cursor_blink();
                    }
                    if let Some(bytes) = terminal_key_bytes(&event, self.mods) {
                        self.write_pty(&bytes);
                    }
                }
            }
            WindowEvent::Resized(sz) => {
                if sz.width > 0 && sz.height > 0 {
                    if let Some(w) = self.window.clone() {
                        self.resize_terminal(&w);
                        w.request_redraw();
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                let Some(w) = self.window.as_ref() else {
                    return;
                };
                let Some(r) = self.renderer.as_mut() else {
                    return;
                };
                let Some(grid) = self.grid.as_ref() else {
                    return;
                };
                let sz = w.inner_size();
                if let Err(e) = r.draw_grid(grid, sz.width, sz.height) {
                    eprintln!("draw: {e}");
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(w) = self.window.as_ref() {
            w.request_redraw();
        }
    }
}

fn terminal_key_bytes(key: &KeyEvent, mods: ModifiersState) -> Option<Vec<u8>> {
    if key.state != ElementState::Pressed {
        return None;
    }
    if mods.control_key() {
        if let Some(t) = key.text.as_ref() {
            for c in t.chars() {
                if let Some(b) = ctrl_byte(c) {
                    return Some(vec![b]);
                }
            }
        }
        if let PhysicalKey::Code(code) = key.physical_key {
            return ctrl_from_code(code);
        }
    }
    if let Some(t) = key.text.as_ref() {
        return Some(t.as_bytes().to_vec());
    }
    use PhysicalKey::Code;
    match key.physical_key {
        Code(KeyCode::Enter) => Some(b"\r".to_vec()),
        Code(KeyCode::Tab) => Some(b"\t".to_vec()),
        Code(KeyCode::Backspace) => Some(vec![0x7f]),
        Code(KeyCode::ArrowUp) => Some(b"\x1b[A".to_vec()),
        Code(KeyCode::ArrowDown) => Some(b"\x1b[B".to_vec()),
        Code(KeyCode::ArrowRight) => Some(b"\x1b[C".to_vec()),
        Code(KeyCode::ArrowLeft) => Some(b"\x1b[D".to_vec()),
        _ => match &key.logical_key {
            winit::keyboard::Key::Named(NamedKey::Enter) => Some(b"\r".to_vec()),
            winit::keyboard::Key::Named(NamedKey::Tab) => Some(b"\t".to_vec()),
            winit::keyboard::Key::Named(NamedKey::Backspace) => Some(vec![0x7f]),
            winit::keyboard::Key::Named(NamedKey::ArrowUp) => Some(b"\x1b[A".to_vec()),
            winit::keyboard::Key::Named(NamedKey::ArrowDown) => Some(b"\x1b[B".to_vec()),
            winit::keyboard::Key::Named(NamedKey::ArrowRight) => Some(b"\x1b[C".to_vec()),
            winit::keyboard::Key::Named(NamedKey::ArrowLeft) => Some(b"\x1b[D".to_vec()),
            _ => None,
        },
    }
}

fn ctrl_byte(c: char) -> Option<u8> {
    let lower = c.to_ascii_lowercase();
    if lower.is_ascii_lowercase() {
        Some(lower as u8 - b'a' + 1)
    } else {
        None
    }
}

fn ctrl_from_code(code: KeyCode) -> Option<Vec<u8>> {
    let b = match code {
        KeyCode::KeyA => 1,
        KeyCode::KeyB => 2,
        KeyCode::KeyC => 3,
        KeyCode::KeyD => 4,
        KeyCode::KeyE => 5,
        KeyCode::KeyF => 6,
        KeyCode::KeyG => 7,
        KeyCode::KeyH => 8,
        KeyCode::KeyI => 9,
        KeyCode::KeyJ => 10,
        KeyCode::KeyK => 11,
        KeyCode::KeyL => 12,
        KeyCode::KeyM => 13,
        KeyCode::KeyN => 14,
        KeyCode::KeyO => 15,
        KeyCode::KeyP => 16,
        KeyCode::KeyQ => 17,
        KeyCode::KeyR => 18,
        KeyCode::KeyS => 19,
        KeyCode::KeyT => 20,
        KeyCode::KeyU => 21,
        KeyCode::KeyV => 22,
        KeyCode::KeyW => 23,
        KeyCode::KeyX => 24,
        KeyCode::KeyY => 25,
        KeyCode::KeyZ => 26,
        KeyCode::BracketLeft => 27,
        KeyCode::Backslash => 28,
        KeyCode::BracketRight => 29,
        _ => return None,
    };
    Some(vec![b])
}

#[cfg(test)]
mod tests {
    use super::{CommandMode, parse_command_mode};

    fn v(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn launch_mode_when_no_args() {
        assert_eq!(parse_command_mode(&v(&[]), false), CommandMode::LaunchGui);
    }

    #[test]
    fn help_and_version_allowed_outside() {
        assert_eq!(
            parse_command_mode(&v(&["--help"]), false),
            CommandMode::Help
        );
        assert_eq!(
            parse_command_mode(&v(&["--version"]), false),
            CommandMode::Version
        );
    }

    #[test]
    fn outside_denies_other_commands() {
        assert_eq!(
            parse_command_mode(&v(&["status"]), false),
            CommandMode::DeniedOutside(v(&["status"]))
        );
    }

    #[test]
    fn inside_allows_other_commands() {
        assert_eq!(
            parse_command_mode(&v(&["status"]), true),
            CommandMode::InsideSession(v(&["status"]))
        );
    }
}
