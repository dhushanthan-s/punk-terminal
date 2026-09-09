//! wgpu + font atlas rendering for [`punk_terminal::TerminalGrid`].

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use bytemuck::{Pod, Zeroable};
use fontdue::Font;
use punk_terminal::TerminalGrid;
use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;
use winit::window::Window;

const DEFAULT_FONT_PX: f32 = 14.0;
const DEFAULT_FONT_FAMILY: &str = "JetBrainsMonoNerdFont-Regular";
const FALLBACK_CHAR: char = '?';

const ATLAS_W: u32 = 2048;
const ATLAS_H: u32 = 1024;
/// Transparent margin kept between packed glyphs so neighbours never bleed in.
const GUTTER: u32 = 2;
/// Side of the fully-opaque block reserved at atlas (0, 0) for solid fills.
const SOLID_PX: u32 = 2;
/// Stop flushing a full atlas after this many resets, so a pathological working
/// set degrades to `?` instead of rebuilding the atlas every single frame.
const MAX_ATLAS_RESETS: u32 = 4;
/// Cursor blink half-period. Matches xterm's default.
const BLINK_MS: u128 = 530;

/// Shelf packer for the glyph atlas. The cursor persists across frames so glyphs
/// can be added lazily as new codepoints appear.
struct ShelfPacker {
    x: u32,
    y: u32,
    row_h: u32,
}

impl ShelfPacker {
    /// Start packing past the reserved solid block so it can never be overwritten.
    fn reset() -> Self {
        Self {
            x: SOLID_PX + GUTTER,
            y: GUTTER,
            row_h: 0,
        }
    }

    /// Reserve a `w` x `h` rect, returning its top-left corner. `None` when full.
    fn alloc(&mut self, w: u32, h: u32) -> Option<(u32, u32)> {
        if w == 0 || h == 0 || w + GUTTER > ATLAS_W || h + GUTTER > ATLAS_H {
            return None;
        }
        if self.x + w + GUTTER > ATLAS_W {
            self.x = GUTTER;
            self.y += self.row_h + GUTTER;
            self.row_h = 0;
        }
        if self.y + h + GUTTER > ATLAS_H {
            return None;
        }
        let spot = (self.x, self.y);
        self.x += w + GUTTER;
        self.row_h = self.row_h.max(h);
        Some(spot)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct GpuVertex {
    pos: [f32; 2],
    uv: [f32; 2],
    fg: [f32; 3],
    bg: [f32; 3],
}

pub struct TerminalRenderer {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    atlas_texture: wgpu::Texture,
    #[allow(dead_code)]
    sampler: wgpu::Sampler,
    cell_w_px: f32,
    cell_h_px: f32,
    baseline_px: f32,
    /// Kept alive so glyphs can be rasterized on demand, not just at startup.
    font: Font,
    font_px: f32,
    packer: ShelfPacker,
    atlas_glyphs: HashMap<char, AtlasGlyph>,
    fallback_glyph: AtlasGlyph,
    /// UV of the reserved opaque texel; a degenerate rect, so it always samples 1.0.
    solid_uv: [f32; 4],
    /// A pack failed during the last frame; the atlas is flushed before the next one.
    atlas_full: bool,
    atlas_resets: u32,
    blink_epoch: std::time::Instant,
}

#[derive(Clone, Copy)]
struct AtlasGlyph {
    uv: [f32; 4],
    width: f32,
    height: f32,
    xmin: f32,
    ymin: f32,
    advance: f32,
}

impl TerminalRenderer {
    pub fn new(window: Arc<Window>, font_family: &str, font_px: f32) -> Result<Self, String> {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let surface = instance
            .create_surface(window)
            .map_err(|e| format!("create_surface: {e}"))?;

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .ok_or_else(|| "no wgpu adapter".to_string())?;

        let caps = surface.get_capabilities(&adapter);
        let format = caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(caps.formats[0]);

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
            },
            None,
        ))
        .map_err(|e| format!("request_device: {e}"))?;

        let mut config = surface
            .get_default_config(&adapter, size.width.max(1), size.height.max(1))
            .ok_or_else(|| "no surface config".to_string())?;
        config.format = format;
        surface.configure(&device, &config);

        let chosen_px = sanitize_font_size(font_px);
        let font = load_font(font_family)?;
        let line = font
            .horizontal_line_metrics(chosen_px)
            .ok_or_else(|| "no line metrics".to_string())?;
        let (cell_w_px, cell_h_px) = terminal_cell_pixel_size(&font, chosen_px, line.new_line_size);
        let baseline_px = line.ascent.ceil();

        let atlas_tex = create_atlas_texture(&device);
        let atlas_view = atlas_tex.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            ..Default::default()
        });

        let bind_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&atlas_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_layout],
            push_constant_ranges: &[],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("terminal.wgsl").into()),
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<GpuVertex>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Float32x3, 3 => Float32x3],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let mut this = Self {
            surface,
            device,
            queue,
            config,
            pipeline,
            bind_group,
            atlas_texture: atlas_tex,
            sampler,
            cell_w_px,
            cell_h_px,
            baseline_px,
            font,
            font_px: chosen_px,
            packer: ShelfPacker::reset(),
            atlas_glyphs: HashMap::new(),
            fallback_glyph: BLANK_GLYPH,
            solid_uv: [0.0; 4],
            atlas_full: false,
            atlas_resets: 0,
            blink_epoch: std::time::Instant::now(),
        };
        // Fills the atlas, sets `solid_uv`, and replaces the placeholder fallback.
        this.rebuild_atlas();
        Ok(this)
    }

    /// Clear the atlas and re-seed it: solid texel, then printable ASCII as a warm
    /// cache so the first frame does not stall rasterizing the whole prompt.
    fn rebuild_atlas(&mut self) {
        clear_atlas(&self.queue, &self.atlas_texture);
        self.solid_uv = write_solid_texel(&self.queue, &self.atlas_texture);
        self.packer = ShelfPacker::reset();
        self.atlas_glyphs.clear();
        self.fallback_glyph = BLANK_GLYPH;

        for code in 32u8..=126u8 {
            self.ensure_glyph(code as char);
        }
        self.fallback_glyph = self
            .atlas_glyphs
            .get(&FALLBACK_CHAR)
            .copied()
            .or_else(|| self.atlas_glyphs.get(&' ').copied())
            .unwrap_or(BLANK_GLYPH);
    }

    /// Rasterize, pack and upload `ch`, caching the result. Never fails: when the
    /// atlas is full the fallback glyph is returned and a flush is queued for the
    /// next frame.
    fn ensure_glyph(&mut self, ch: char) -> AtlasGlyph {
        // A codepoint the face lacks would rasterize as .notdef. Cache the fallback
        // under it so we do not re-probe the same char on every frame.
        if !self.font.has_glyph(ch) {
            let g = self.fallback_glyph;
            self.atlas_glyphs.insert(ch, g);
            return g;
        }

        let (metrics, bitmap) =
            rasterize_fitted(&self.font, ch, self.font_px, self.cell_w_px, self.cell_h_px);
        let bw = metrics.width as u32;
        let bh = metrics.height as u32;

        let uv = if bw > 0 && bh > 0 {
            // Three disjoint field borrows: `&self.queue`, `&self.atlas_texture` and
            // `&mut self.packer`. This only compiles because `alloc` is a method on
            // ShelfPacker and the upload is a free function - routing either through
            // a `&mut self` method would borrow all of `self`.
            match self.packer.alloc(bw, bh) {
                Some((x, y)) => {
                    upload_glyph(&self.queue, &self.atlas_texture, x, y, bw, bh, &bitmap);
                    uv_rect(x, y, bw, bh)
                }
                None => {
                    // Do NOT reset here: UVs already baked into this frame's vertex
                    // buffer would go stale and the frame would render scrambled.
                    self.atlas_full = true;
                    return self.fallback_glyph;
                }
            }
        } else {
            // Blank glyph (space and friends): no atlas space, never drawn.
            self.solid_uv
        };

        let glyph = AtlasGlyph {
            uv,
            width: metrics.width as f32,
            height: metrics.height as f32,
            xmin: metrics.xmin as f32,
            ymin: metrics.ymin as f32,
            advance: metrics.advance_width.max(1.0),
        };
        self.atlas_glyphs.insert(ch, glyph);
        glyph
    }

    /// Restart the blink phase so the cursor is solid immediately after a keystroke.
    pub fn reset_cursor_blink(&mut self) {
        self.blink_epoch = std::time::Instant::now();
    }

    pub fn cell_pixel_size(&self) -> (f32, f32) {
        (self.cell_w_px, self.cell_h_px)
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.config.width = size.width;
            self.config.height = size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    /// Pass the window inner size in physical pixels so cell quads match the grid used for the PTY
    /// (avoids micro‑columns when the swapchain size lags the window).
    pub fn draw_grid(
        &mut self,
        grid: &TerminalGrid,
        width: u32,
        height: u32,
    ) -> Result<(), String> {
        let w = width.max(1) as f32;
        let h = height.max(1) as f32;
        if width != self.config.width || height != self.config.height {
            self.config.width = width.max(1);
            self.config.height = height.max(1);
            self.surface.configure(&self.device, &self.config);
        }

        let cols = grid.cols() as usize;
        let rows = grid.rows() as usize;
        let cw = self.cell_w_px.max(1.0).round();
        let ch = self.cell_h_px.max(1.0).round();
        let content_w = cw * cols as f32;
        let content_h = ch * rows as f32;
        let origin_x = ((w - content_w) * 0.5).max(0.0).floor();
        let origin_y = ((h - content_h) * 0.5).max(0.0).floor();

        // A pack failed last frame. Flush now, between frames, so no UV baked into a
        // vertex buffer is invalidated mid-build.
        if self.atlas_full {
            self.atlas_full = false;
            if self.atlas_resets < MAX_ATLAS_RESETS {
                self.atlas_resets += 1;
                self.rebuild_atlas();
                if self.atlas_resets == MAX_ATLAS_RESETS {
                    eprintln!(
                        "glyph atlas exhausted repeatedly; uncached characters will render as '{FALLBACK_CHAR}'"
                    );
                }
            }
        }

        let (cur_col, cur_row) = grid.cursor();
        let blink_on = (self.blink_epoch.elapsed().as_millis() / BLINK_MS) % 2 == 0;
        let draw_cursor = grid.cursor_visible() && blink_on;
        let solid_uv = self.solid_uv;

        let mut verts: Vec<GpuVertex> = Vec::with_capacity(cols * rows * 6);
        let mut text_verts: Vec<GpuVertex> = Vec::with_capacity(cols * rows * 6);
        let cells = grid.cells();

        for row in 0..rows {
            for col in 0..cols {
                let cell = &cells[row * cols + col];
                let c = cell.ch;
                // Copy out of the map first: holding the `get` borrow across the
                // `ensure_glyph` call in the None arm would not compile.
                let cached = self.atlas_glyphs.get(&c).copied();
                let glyph = match cached {
                    Some(g) => g,
                    None => self.ensure_glyph(c),
                };
                // Inverse video paints the block cursor using the geometry that is
                // already being emitted for this cell - no extra quad, no shader change.
                let is_cursor = draw_cursor && row == cur_row as usize && col == cur_col as usize;
                let (fg, bg) = if is_cursor {
                    (rgb01(&cell.bg), rgb01(&cell.fg))
                } else {
                    (rgb01(&cell.fg), rgb01(&cell.bg))
                };
                let x0 = origin_x + col as f32 * cw;
                let y0 = origin_y + row as f32 * ch;
                let x1 = x0 + cw;
                let y1 = y0 + ch;
                let ndc = |px: f32, py: f32| [px / w * 2.0 - 1.0, 1.0 - py / h * 2.0];
                let p00 = ndc(x0, y0);
                let p10 = ndc(x1, y0);
                let p01 = ndc(x0, y1);
                let p11 = ndc(x1, y1);
                // Background pass: fill cell exactly.
                quad(&mut verts, p00, p10, p01, p11, solid_uv, bg, bg);

                if c == ' ' || glyph.width <= 0.0 || glyph.height <= 0.0 {
                    continue;
                }
                let glyph_x = (x0 + ((cw - glyph.advance).max(0.0) * 0.5) + glyph.xmin).round();
                // fontdue::Metrics::ymin is the distance from baseline to glyph bottom.
                // Convert to top-left in pixel space: baseline - (height + ymin).
                let glyph_top = (y0 + self.baseline_px - glyph.height - glyph.ymin).round();
                let gx0 = glyph_x.max(x0);
                let gy0 = glyph_top.max(y0);
                let gx1 = (glyph_x + glyph.width).min(x1);
                let gy1 = (glyph_top + glyph.height).min(y1);
                if gx1 <= gx0 || gy1 <= gy0 {
                    continue;
                }
                let uv = clip_uv_to_quad(
                    glyph.uv,
                    glyph_x,
                    glyph_top,
                    glyph.width,
                    glyph.height,
                    gx0,
                    gy0,
                    gx1,
                    gy1,
                );
                let gp00 = ndc(gx0, gy0);
                let gp10 = ndc(gx1, gy0);
                let gp01 = ndc(gx0, gy1);
                let gp11 = ndc(gx1, gy1);
                quad(&mut text_verts, gp00, gp10, gp01, gp11, uv, fg, bg);
            }
        }
        verts.extend(text_verts);

        let vb = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&verts),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let frame = match self.surface.get_current_texture() {
            Ok(f) => f,
            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                self.surface.configure(&self.device, &self.config);
                self.surface
                    .get_current_texture()
                    .map_err(|e| format!("surface texture: {e}"))?
            }
            Err(e) => return Err(format!("surface texture: {e}")),
        };
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.bind_group, &[]);
            pass.set_vertex_buffer(0, vb.slice(..));
            pass.draw(0..verts.len() as u32, 0..1);
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();
        Ok(())
    }
}

fn sanitize_font_size(size: f32) -> f32 {
    if size.is_finite() && size > 0.0 {
        size.clamp(6.0, 128.0)
    } else {
        DEFAULT_FONT_PX
    }
}

fn resolve_font_file(font_family: &str) -> Option<PathBuf> {
    let trimmed = font_family.trim();
    if trimmed.is_empty() {
        return None;
    }

    let direct_path = Path::new(trimmed);
    if direct_path.is_file() {
        return Some(direct_path.to_path_buf());
    }

    // Dev-only: present under `cargo run`, never in an installed binary.
    let bundled = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fonts")
        .join(format!("{trimmed}.ttf"));
    if bundled.is_file() {
        return Some(bundled);
    }

    for dir in system_font_dirs() {
        if let Some(hit) = find_font_in_dir(&dir, trimmed, 1) {
            return Some(hit);
        }
    }

    None
}

/// Standard per-platform font directories, most specific (user) first.
fn system_font_dirs() -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = Vec::new();
    let home = std::env::var_os("HOME").map(PathBuf::from);

    if cfg!(target_os = "macos") {
        if let Some(h) = home.as_ref() {
            dirs.push(h.join("Library/Fonts"));
        }
        dirs.push(PathBuf::from("/Library/Fonts"));
        dirs.push(PathBuf::from("/System/Library/Fonts"));
    } else if cfg!(target_os = "windows") {
        if let Some(local) = std::env::var_os("LOCALAPPDATA") {
            dirs.push(PathBuf::from(local).join("Microsoft/Windows/Fonts"));
        }
        if let Some(win) = std::env::var_os("SystemRoot") {
            dirs.push(PathBuf::from(win).join("Fonts"));
        }
    } else {
        if let Some(h) = home.as_ref() {
            dirs.push(h.join(".local/share/fonts"));
            dirs.push(h.join(".fonts"));
        }
        dirs.push(PathBuf::from("/usr/local/share/fonts"));
        dirs.push(PathBuf::from("/usr/share/fonts"));
    }
    dirs
}

/// Look for `{name}.{ttf,otf,ttc}` in `dir`, recursing `depth` more levels.
/// Linux font dirs nest by foundry, so one level of recursion is needed there.
fn find_font_in_dir(dir: &Path, name: &str, depth: usize) -> Option<PathBuf> {
    for ext in ["ttf", "otf", "ttc"] {
        let candidate = dir.join(format!("{name}.{ext}"));
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    if depth == 0 {
        return None;
    }
    let entries = fs::read_dir(dir).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if let Some(hit) = find_font_in_dir(&path, name, depth - 1) {
                return Some(hit);
            }
        }
    }
    None
}

/// Bundled JetBrains Mono Nerd Font, used when `font_family` cannot be resolved.
const EMBEDDED_FONT: &[u8] = include_bytes!("../fonts/JetBrainsMonoNerdFont-Regular.ttf");

fn load_font(font_family: &str) -> Result<Font, String> {
    if let Some(path) = resolve_font_file(font_family) {
        if let Ok(bytes) = fs::read(&path) {
            if let Ok(font) = Font::from_bytes(bytes, fontdue::FontSettings::default()) {
                return Ok(font);
            }
        }
    }

    Font::from_bytes(EMBEDDED_FONT, fontdue::FontSettings::default())
        .map_err(|_| format!("failed to load fallback font ({}).", DEFAULT_FONT_FAMILY))
}

/// One terminal cell in pixels: at least the glyph box and advance, so column count stays sane.
fn terminal_cell_pixel_size(font: &Font, px: f32, new_line: f32) -> (f32, f32) {
    let (mm, _) = font.rasterize('M', px);
    let m = font.metrics('M', px);
    let adv = m.advance_width;
    if !adv.is_finite() || adv <= 0.0 {
        return (8.0_f32, new_line.max(12.0).max(1.0));
    }
    let w_from_glyph = mm.width as f32 + 2.0;
    let cell_w = adv.max(w_from_glyph).ceil().clamp(6.0, 256.0);
    let h_from_glyph = mm.height as f32 + 2.0;
    let cell_h = new_line.max(h_from_glyph).ceil().clamp(6.0, 256.0);
    (cell_w, cell_h)
}

fn rgb01(c: &[u8; 3]) -> [f32; 3] {
    [
        c[0] as f32 / 255.0,
        c[1] as f32 / 255.0,
        c[2] as f32 / 255.0,
    ]
}

fn quad(
    verts: &mut Vec<GpuVertex>,
    p00: [f32; 2],
    p10: [f32; 2],
    p01: [f32; 2],
    p11: [f32; 2],
    uv: [f32; 4],
    fg: [f32; 3],
    bg: [f32; 3],
) {
    let u0 = uv[0];
    let v0 = uv[1];
    let u1 = uv[2];
    let v1 = uv[3];
    verts.push(GpuVertex {
        pos: p00,
        uv: [u0, v0],
        fg,
        bg,
    });
    verts.push(GpuVertex {
        pos: p10,
        uv: [u1, v0],
        fg,
        bg,
    });
    verts.push(GpuVertex {
        pos: p01,
        uv: [u0, v1],
        fg,
        bg,
    });
    verts.push(GpuVertex {
        pos: p10,
        uv: [u1, v0],
        fg,
        bg,
    });
    verts.push(GpuVertex {
        pos: p11,
        uv: [u1, v1],
        fg,
        bg,
    });
    verts.push(GpuVertex {
        pos: p01,
        uv: [u0, v1],
        fg,
        bg,
    });
}

fn clip_uv_to_quad(
    uv: [f32; 4],
    glyph_x: f32,
    glyph_y: f32,
    glyph_w: f32,
    glyph_h: f32,
    qx0: f32,
    qy0: f32,
    qx1: f32,
    qy1: f32,
) -> [f32; 4] {
    let [u0, v0, u1, v1] = uv;
    let du = (u1 - u0).max(0.0);
    let dv = (v1 - v0).max(0.0);
    let tx0 = ((qx0 - glyph_x) / glyph_w).clamp(0.0, 1.0);
    let tx1 = ((qx1 - glyph_x) / glyph_w).clamp(0.0, 1.0);
    let ty0 = ((qy0 - glyph_y) / glyph_h).clamp(0.0, 1.0);
    let ty1 = ((qy1 - glyph_y) / glyph_h).clamp(0.0, 1.0);
    [u0 + du * tx0, v0 + dv * ty0, u0 + du * tx1, v0 + dv * ty1]
}

/// A cached entry for a glyph that occupies no atlas space and is never drawn.
const BLANK_GLYPH: AtlasGlyph = AtlasGlyph {
    uv: [0.0, 0.0, 0.0, 0.0],
    width: 0.0,
    height: 0.0,
    xmin: 0.0,
    ymin: 0.0,
    advance: 1.0,
};

fn create_atlas_texture(device: &wgpu::Device) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("glyph atlas"),
        size: wgpu::Extent3d {
            width: ATLAS_W,
            height: ATLAS_H,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::R8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    })
}

/// Upload one tightly packed R8 coverage bitmap at `(x, y)`.
///
/// `Queue::write_texture` re-stages internally, so unlike `copy_buffer_to_texture`
/// it imposes no 256-byte `bytes_per_row` alignment. For `R8Unorm` the stride in
/// bytes equals the width in texels, which is exactly fontdue's bitmap layout.
fn upload_glyph(
    queue: &wgpu::Queue,
    texture: &wgpu::Texture,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    bitmap: &[u8],
) {
    if w == 0 || h == 0 || bitmap.len() < (w as usize) * (h as usize) {
        return;
    }
    queue.write_texture(
        wgpu::ImageCopyTexture {
            texture,
            mip_level: 0,
            origin: wgpu::Origin3d { x, y, z: 0 },
            aspect: wgpu::TextureAspect::All,
        },
        bitmap,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(w),
            rows_per_image: Some(h),
        },
        wgpu::Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        },
    );
}

/// Zero the whole atlas in one upload. The staging buffer is dropped straight
/// after, so no CPU-side mirror of the atlas is kept resident.
fn clear_atlas(queue: &wgpu::Queue, texture: &wgpu::Texture) {
    let zeros = vec![0u8; (ATLAS_W as usize) * (ATLAS_H as usize)];
    upload_glyph(queue, texture, 0, 0, ATLAS_W, ATLAS_H, &zeros);
}

/// Reserve the opaque block at the atlas origin and return the UV that samples it.
fn write_solid_texel(queue: &wgpu::Queue, texture: &wgpu::Texture) -> [f32; 4] {
    let solid = [0xFFu8; (SOLID_PX * SOLID_PX) as usize];
    upload_glyph(queue, texture, 0, 0, SOLID_PX, SOLID_PX, &solid);
    // Degenerate rect at the block centre: every vertex samples the same interior
    // texel, so coverage is exactly 1.0 and `mix(bg, fg, cov)` yields pure fg.
    let u = (SOLID_PX as f32 * 0.5) / ATLAS_W as f32;
    let v = (SOLID_PX as f32 * 0.5) / ATLAS_H as f32;
    [u, v, u, v]
}

fn uv_rect(x: u32, y: u32, w: u32, h: u32) -> [f32; 4] {
    let inset_u = 0.5 / ATLAS_W as f32;
    let inset_v = 0.5 / ATLAS_H as f32;
    let u0 = x as f32 / ATLAS_W as f32 + inset_u;
    let v0 = y as f32 / ATLAS_H as f32 + inset_v;
    let u1 = (x + w) as f32 / ATLAS_W as f32 - inset_u;
    let v1 = (y + h) as f32 / ATLAS_H as f32 - inset_v;
    [u0, v0, u1.max(u0), v1.max(v0)]
}

/// Rasterize `ch` small enough to fit inside one cell.
///
/// Nerd Font icons in the non-`Mono` variants routinely have ~2x the advance of a
/// text glyph. Rather than truncating them at the cell edge, re-rasterize at a
/// smaller size: fontdue recomputes coverage from the outline, which stays crisp
/// under the atlas's `Nearest` sampler where a resampled bitmap would not.
///
/// The scale is uniform about the baseline origin, so `height`, `ymin`, `xmin` and
/// `advance` all shrink together and the caller's baseline placement still holds.
fn rasterize_fitted(
    font: &Font,
    ch: char,
    px: f32,
    cell_w: f32,
    cell_h: f32,
) -> (fontdue::Metrics, Vec<u8>) {
    let (mut metrics, mut bitmap) = font.rasterize(ch, px);
    let mut try_px = px;

    // Metrics are not linear in px (hinting, rounding), so converge over a few
    // passes instead of assuming one correction lands. Bounded by the iteration
    // count and the 4px floor; `scale` is capped below 1.0 so px strictly shrinks.
    for _ in 0..3 {
        let w = metrics.width as f32;
        let h = metrics.height as f32;
        if w <= cell_w && h <= cell_h {
            break;
        }
        let scale = (cell_w / w.max(1.0)).min(cell_h / h.max(1.0)).min(0.999);
        try_px = (try_px * scale).max(4.0);
        let (m2, b2) = font.rasterize(ch, try_px);
        metrics = m2;
        bitmap = b2;
        if try_px <= 4.0 {
            break;
        }
    }
    (metrics, bitmap)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn embedded() -> Font {
        Font::from_bytes(EMBEDDED_FONT, fontdue::FontSettings::default()).expect("bundled font")
    }

    /// The regression this whole change exists for: these codepoints used to render
    /// as `?` because the atlas only covered ASCII 32..=126.
    #[test]
    fn bundled_font_covers_nerd_and_box_drawing() {
        let font = embedded();
        for (ch, what) in [
            ('\u{e0b0}', "powerline separator"),
            ('\u{e0b2}', "powerline separator (reverse)"),
            ('\u{f015}', "nerd home icon"),
            ('\u{f07b}', "nerd folder icon"),
            ('\u{2500}', "box drawing"),
            ('\u{2588}', "full block"),
        ] {
            assert!(
                font.has_glyph(ch),
                "bundled font is missing {what} U+{:04X}",
                ch as u32
            );
        }
    }

    /// Known gap, asserted so it is recorded rather than assumed working: the
    /// bundled JetBrains Mono Nerd Font has no Braille block at all, so spinners
    /// and `btop`-style meters still fall back to `?`. Closing this needs a real
    /// multi-face fallback chain, which the renderer does not have - it loads
    /// exactly one `Font`. Flip this test when that lands.
    #[test]
    fn bundled_font_has_no_braille() {
        let font = embedded();
        assert!(!font.has_glyph('\u{2800}'));
        assert!(!font.has_glyph('\u{28ff}'));
    }

    #[test]
    fn rasterize_fitted_shrinks_oversized_glyphs_into_the_cell() {
        let font = embedded();
        let px = 34.0;
        let line = font.horizontal_line_metrics(px).expect("line metrics");
        let (cell_w, cell_h) = terminal_cell_pixel_size(&font, px, line.new_line_size);
        let (cell_w, cell_h) = (cell_w.round(), cell_h.round());

        // Nerd Font icons in the non-"Mono" variant are routinely ~2 cells wide.
        for ch in ['\u{e0b0}', '\u{f015}', '\u{f07b}', '\u{f121}'] {
            let (m, bmp) = rasterize_fitted(&font, ch, px, cell_w, cell_h);
            assert_eq!(
                bmp.len(),
                m.width * m.height,
                "bitmap must stay tightly packed"
            );
            assert!(
                m.width as f32 <= cell_w && m.height as f32 <= cell_h,
                "U+{:04X} rasterized {}x{}, does not fit cell {cell_w}x{cell_h}",
                ch as u32,
                m.width,
                m.height,
            );
        }
    }

    #[test]
    fn rasterize_fitted_leaves_ascii_untouched() {
        let font = embedded();
        let px = 34.0;
        let line = font.horizontal_line_metrics(px).expect("line metrics");
        let (cell_w, cell_h) = terminal_cell_pixel_size(&font, px, line.new_line_size);

        // ASCII already fits, so this must be the no-op fast path - identical to a
        // plain rasterize, which is what keeps text pixel-for-pixel as it was.
        for ch in ['M', 'g', '@', '.'] {
            let (fitted, _) = rasterize_fitted(&font, ch, px, cell_w, cell_h);
            let (plain, _) = font.rasterize(ch, px);
            assert_eq!(fitted.width, plain.width, "{ch} width changed");
            assert_eq!(fitted.height, plain.height, "{ch} height changed");
        }
    }

    #[test]
    fn packer_never_hands_out_the_reserved_solid_block() {
        let mut p = ShelfPacker::reset();
        for _ in 0..500 {
            let Some((x, y)) = p.alloc(9, 18) else { break };
            assert!(
                x >= SOLID_PX + GUTTER || y >= SOLID_PX + GUTTER,
                "alloc at ({x}, {y}) overlaps the solid texel"
            );
        }
    }

    #[test]
    fn packer_wraps_shelves_and_reports_full() {
        let mut p = ShelfPacker::reset();
        let mut last_y = 0;
        let mut wrapped = false;
        // Fill the atlas completely; alloc must return None rather than run off the end.
        while let Some((_, y)) = p.alloc(64, 64) {
            if y > last_y {
                wrapped = true;
            }
            last_y = y;
            assert!(y + 64 <= ATLAS_H, "packed past the bottom edge");
        }
        assert!(wrapped, "packer should have moved to a new shelf");
    }

    #[test]
    fn packer_rejects_degenerate_and_oversized_rects() {
        let mut p = ShelfPacker::reset();
        assert!(p.alloc(0, 10).is_none());
        assert!(p.alloc(10, 0).is_none());
        assert!(p.alloc(ATLAS_W, 10).is_none());
        assert!(p.alloc(10, ATLAS_H).is_none());
    }

    #[test]
    fn solid_uv_is_a_degenerate_rect_inside_the_reserved_block() {
        // write_solid_texel needs a GPU queue, so recompute the UV it returns.
        let u = (SOLID_PX as f32 * 0.5) / ATLAS_W as f32;
        let v = (SOLID_PX as f32 * 0.5) / ATLAS_H as f32;
        let uv = [u, v, u, v];
        assert_eq!(uv[0], uv[2], "u must be degenerate so coverage is constant");
        assert_eq!(uv[1], uv[3], "v must be degenerate so coverage is constant");
        assert!(u * ATLAS_W as f32 <= SOLID_PX as f32);
        assert!(v * ATLAS_H as f32 <= SOLID_PX as f32);
    }

    #[test]
    fn uv_rect_stays_inside_its_allocation() {
        let uv = uv_rect(100, 200, 10, 20);
        assert!(uv[0] >= 100.0 / ATLAS_W as f32);
        assert!(uv[2] <= 110.0 / ATLAS_W as f32);
        assert!(uv[1] >= 200.0 / ATLAS_H as f32);
        assert!(uv[3] <= 220.0 / ATLAS_H as f32);
        assert!(uv[2] >= uv[0] && uv[3] >= uv[1]);
    }

    #[test]
    fn resolve_font_file_rejects_blank_and_unknown_names() {
        assert!(resolve_font_file("").is_none());
        assert!(resolve_font_file("   ").is_none());
        assert!(resolve_font_file("NoSuchFontFamily-Nonexistent").is_none());
    }

    #[test]
    fn load_font_falls_back_to_bundled_face() {
        // An unresolvable family must still yield a usable font, not an error.
        let font = load_font("NoSuchFontFamily-Nonexistent").expect("fallback font");
        assert!(font.has_glyph('M'));
        assert!(font.has_glyph('\u{f015}'));
    }
}
