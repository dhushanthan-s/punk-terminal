//! Terminal screen grid driven by [`vte::Perform`].

use vte::{Params, Perform};

const TAB_STOP: u16 = 8;

/// Default foreground (light gray) and background (black), sRGB bytes.
pub const DEFAULT_FG: [u8; 3] = [204, 204, 204];
pub const DEFAULT_BG: [u8; 3] = [12, 12, 12];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    pub ch: char,
    pub fg: [u8; 3],
    pub bg: [u8; 3],
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: DEFAULT_FG,
            bg: DEFAULT_BG,
        }
    }
}

/// Scrollable terminal buffer with cursor and SGR state.
pub struct TerminalGrid {
    cols: u16,
    rows: u16,
    cells: Vec<Cell>,
    cursor_col: u16,
    cursor_row: u16,
    cursor_visible: bool,
    scroll_top: u16,
    scroll_bottom: u16,
    fg: [u8; 3],
    bg: [u8; 3],
    bold: bool,
    dirty: bool,
}

impl TerminalGrid {
    pub fn new(cols: u16, rows: u16) -> Self {
        let mut g = Self {
            cols,
            rows,
            cells: Vec::new(),
            cursor_col: 0,
            cursor_row: 0,
            cursor_visible: true,
            scroll_top: 0,
            scroll_bottom: rows.saturating_sub(1),
            fg: DEFAULT_FG,
            bg: DEFAULT_BG,
            bold: false,
            dirty: true,
        };
        g.resize_buffer();
        g
    }

    pub fn cols(&self) -> u16 {
        self.cols
    }

    pub fn rows(&self) -> u16 {
        self.rows
    }

    pub fn cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn cursor(&self) -> (u16, u16) {
        (self.cursor_col, self.cursor_row)
    }

    /// Whether the cursor should be painted. Toggled by DECTCEM (`CSI ?25h` / `CSI ?25l`).
    pub fn cursor_visible(&self) -> bool {
        self.cursor_visible
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    /// Resize the grid; clears content. Call after PTY resize.
    pub fn resize(&mut self, cols: u16, rows: u16) {
        self.cols = cols;
        self.rows = rows;
        self.cursor_col = self.cursor_col.min(cols.saturating_sub(1));
        self.cursor_row = self.cursor_row.min(rows.saturating_sub(1));
        self.scroll_top = 0;
        self.scroll_bottom = rows.saturating_sub(1);
        self.resize_buffer();
        self.dirty = true;
    }

    fn resize_buffer(&mut self) {
        let n = (self.cols as usize) * (self.rows as usize);
        self.cells.clear();
        self.cells.resize(n, Cell::default());
    }

    fn idx(&self, col: u16, row: u16) -> usize {
        row as usize * self.cols as usize + col as usize
    }

    fn putc(&mut self, c: char) {
        if self.cursor_row >= self.rows || self.cursor_col >= self.cols {
            return;
        }
        let i = self.idx(self.cursor_col, self.cursor_row);
        self.cells[i] = Cell {
            ch: c,
            fg: self.fg,
            bg: self.bg,
        };
        self.dirty = true;
        self.cursor_col += 1;
        if self.cursor_col >= self.cols {
            self.linefeed();
        }
    }

    fn linefeed(&mut self) {
        self.cursor_col = 0;
        if self.cursor_row < self.scroll_bottom {
            self.cursor_row += 1;
            return;
        }
        self.scroll_up_one();
    }

    fn scroll_up_one(&mut self) {
        let top = self.scroll_top as usize;
        let bot = self.scroll_bottom as usize;
        let w = self.cols as usize;
        if bot <= top {
            return;
        }
        for r in top..bot {
            for c in 0..w {
                let dst = r * w + c;
                let src = (r + 1) * w + c;
                self.cells[dst] = self.cells[src];
            }
        }
        let last = bot * w;
        for c in 0..w {
            self.cells[last + c] = Cell::default();
        }
        self.dirty = true;
    }

    fn carriage_return(&mut self) {
        self.cursor_col = 0;
        self.dirty = true;
    }

    /// BS (0x08) moves the cursor one column left and nothing else.
    ///
    /// It must not erase: applications rub a character out by writing `\b \b` or
    /// `\b` + `CSI K`, and readline emits a bare `\b` purely to reposition. Clearing
    /// the cell here deletes text the application still expects to be on screen.
    fn backspace(&mut self) {
        if self.cursor_col > 0 {
            self.cursor_col -= 1;
            self.dirty = true;
        }
    }

    /// DEC private mode set/reset (`CSI ? Pn h` / `CSI ? Pn l`).
    ///
    /// Only DECTCEM (25, cursor visibility) is implemented. Alt-screen (1049),
    /// bracketed paste (2004) and app-cursor-keys (1) are still ignored.
    fn set_private_mode(&mut self, params: &Params, enable: bool) {
        for group in params.iter() {
            if group.first().copied() == Some(25) {
                self.cursor_visible = enable;
                self.dirty = true;
            }
        }
    }

    fn tab(&mut self) {
        let next = ((self.cursor_col / TAB_STOP) + 1) * TAB_STOP;
        self.cursor_col = next.min(self.cols.saturating_sub(1));
        self.dirty = true;
    }

    fn erase_line(&mut self, mode: u16) {
        let row = self.cursor_row as usize;
        let w = self.cols as usize;
        let base = row * w;
        let blank = Cell::default();
        match mode {
            1 => {
                for c in 0..=self.cursor_col as usize {
                    self.cells[base + c] = blank;
                }
            }
            2 => {
                for c in 0..w {
                    self.cells[base + c] = blank;
                }
            }
            _ => {
                for c in self.cursor_col as usize..w {
                    self.cells[base + c] = blank;
                }
            }
        }
        self.dirty = true;
    }

    fn erase_display(&mut self, mode: u16) {
        let w = self.cols as usize;
        let h = self.rows as usize;
        let cell = Cell::default();
        match mode {
            1 => {
                for r in 0..=self.cursor_row as usize {
                    for c in 0..w {
                        self.cells[r * w + c] = cell;
                    }
                }
            }
            2 => {
                for v in self.cells.iter_mut() {
                    *v = cell;
                }
            }
            _ => {
                for r in self.cursor_row as usize..h {
                    for c in 0..w {
                        self.cells[r * w + c] = cell;
                    }
                }
            }
        }
        self.dirty = true;
    }

    fn apply_sgr(&mut self, params: &Params) {
        let nums: Vec<u16> = params.iter().flat_map(|s| s.iter().copied()).collect();
        if nums.is_empty() {
            self.fg = DEFAULT_FG;
            self.bg = DEFAULT_BG;
            self.bold = false;
            return;
        }
        let mut i = 0usize;
        while i < nums.len() {
            let n = nums[i];
            match n {
                0 => {
                    self.fg = DEFAULT_FG;
                    self.bg = DEFAULT_BG;
                    self.bold = false;
                }
                1 => self.bold = true,
                22 => self.bold = false,
                30..=37 => self.fg = ansi_fg(n),
                39 => self.fg = DEFAULT_FG,
                40..=47 => self.bg = ansi_bg(n),
                49 => self.bg = DEFAULT_BG,
                38 => {
                    if i + 2 < nums.len() && nums[i + 1] == 5 {
                        self.fg = xterm256(nums[i + 2] as u8);
                        i += 2;
                    } else if i + 4 < nums.len() && nums[i + 1] == 2 {
                        self.fg = [nums[i + 2] as u8, nums[i + 3] as u8, nums[i + 4] as u8];
                        i += 4;
                    }
                }
                48 => {
                    if i + 2 < nums.len() && nums[i + 1] == 5 {
                        self.bg = xterm256(nums[i + 2] as u8);
                        i += 2;
                    } else if i + 4 < nums.len() && nums[i + 1] == 2 {
                        self.bg = [nums[i + 2] as u8, nums[i + 3] as u8, nums[i + 4] as u8];
                        i += 4;
                    }
                }
                _ => {}
            }
            i += 1;
        }
    }
}

fn ansi_fg(n: u16) -> [u8; 3] {
    match n {
        30 => [0, 0, 0],
        31 => [205, 49, 49],
        32 => [13, 188, 121],
        33 => [229, 193, 0],
        34 => [36, 114, 200],
        35 => [188, 84, 188],
        36 => [17, 168, 205],
        37 => [229, 229, 229],
        _ => DEFAULT_FG,
    }
}

fn ansi_bg(n: u16) -> [u8; 3] {
    match n {
        40 => [0, 0, 0],
        41 => [80, 0, 0],
        42 => [0, 80, 0],
        43 => [80, 80, 0],
        44 => [0, 0, 80],
        45 => [80, 0, 80],
        46 => [0, 80, 80],
        47 => [80, 80, 80],
        _ => DEFAULT_BG,
    }
}

fn xterm256(i: u8) -> [u8; 3] {
    if i < 8 {
        let v: u8 = if i == 0 { 0 } else { 205 };
        match i {
            0 => [0, 0, 0],
            1 => [v, 0, 0],
            2 => [0, v, 0],
            3 => [v, v, 0],
            4 => [0, 0, v],
            5 => [v, 0, v],
            6 => [0, v, v],
            _ => [v, v, v],
        }
    } else if i < 16 {
        let v = 255u8;
        match i {
            8 => [85, 85, 85],
            9 => [v, 0, 0],
            10 => [0, v, 0],
            11 => [v, v, 0],
            12 => [0, 0, v],
            13 => [v, 0, v],
            14 => [0, v, v],
            15 => [v, v, v],
            _ => DEFAULT_FG,
        }
    } else if i < 232 {
        let i = i - 16;
        let r = i / 36;
        let g = (i % 36) / 6;
        let b = i % 6;
        let step = |x: u8| -> u8 { if x == 0 { 0 } else { 55 + (x - 1) * 40 } };
        [step(r), step(g), step(b)]
    } else {
        let g = i - 232;
        let v = 8 + g * 10;
        [v, v, v]
    }
}

fn first_param(p: &[u16], default: u16) -> u16 {
    match p.first().copied() {
        None => default,
        Some(0) => default,
        Some(n) => n,
    }
}

fn csi_count(p: &[u16]) -> u16 {
    match p.first().copied() {
        None => 1,
        Some(0) => 1,
        Some(n) => n,
    }
}

impl Perform for TerminalGrid {
    fn print(&mut self, c: char) {
        // vte routes only 0x00-0x1F and 0x80-0x9F to `execute`, so DEL arrives here.
        // It has no meaning on output - drop it rather than printing a blank cell.
        if c == '\u{7f}' {
            return;
        }
        self.putc(c);
    }

    fn execute(&mut self, byte: u8) {
        match byte {
            0x07 => {}
            0x08 => self.backspace(),
            0x09 => self.tab(),
            0x0a..=0x0c => self.linefeed(),
            0x0d => self.carriage_return(),
            _ => {}
        }
    }

    fn csi_dispatch(&mut self, params: &Params, intermediates: &[u8], _ignore: bool, action: char) {
        // `?`-prefixed sequences are DEC private modes, not the standard CSI actions below.
        if intermediates == b"?" {
            match action {
                'h' => self.set_private_mode(params, true),
                'l' => self.set_private_mode(params, false),
                _ => {}
            }
            return;
        }

        match action {
            'A' => {
                let n = params.iter().next().map(csi_count).unwrap_or(1);
                self.cursor_row = self.cursor_row.saturating_sub(n);
                self.dirty = true;
            }
            'B' => {
                let n = params.iter().next().map(csi_count).unwrap_or(1);
                self.cursor_row = (self.cursor_row + n).min(self.rows.saturating_sub(1));
                self.dirty = true;
            }
            'C' => {
                let n = params.iter().next().map(csi_count).unwrap_or(1);
                self.cursor_col = (self.cursor_col + n).min(self.cols.saturating_sub(1));
                self.dirty = true;
            }
            'D' => {
                let n = params.iter().next().map(csi_count).unwrap_or(1);
                self.cursor_col = self.cursor_col.saturating_sub(n);
                self.dirty = true;
            }
            'G' | '`' => {
                let n = params.iter().next().map(csi_count).unwrap_or(1);
                self.cursor_col = (n.saturating_sub(1)).min(self.cols.saturating_sub(1));
                self.dirty = true;
            }
            'H' | 'f' => {
                let mut it = params.iter();
                let row = first_param(it.next().unwrap_or(&[]), 1).saturating_sub(1);
                let col = first_param(it.next().unwrap_or(&[]), 1).saturating_sub(1);
                self.cursor_row = row.min(self.rows.saturating_sub(1));
                self.cursor_col = col.min(self.cols.saturating_sub(1));
                self.dirty = true;
            }
            'J' => {
                let n = params
                    .iter()
                    .next()
                    .and_then(|g| g.first().copied())
                    .unwrap_or(0);
                self.erase_display(n);
            }
            'K' => {
                let n = params
                    .iter()
                    .next()
                    .and_then(|g| g.first().copied())
                    .unwrap_or(0);
                self.erase_line(n);
            }
            'm' => self.apply_sgr(params),
            'r' => {
                let mut it = params.iter();
                let top = first_param(it.next().unwrap_or(&[]), 1).saturating_sub(1);
                let bot = first_param(it.next().unwrap_or(&[]), self.rows).saturating_sub(1);
                self.scroll_top = top.min(self.rows.saturating_sub(1));
                self.scroll_bottom = bot.max(self.scroll_top).min(self.rows.saturating_sub(1));
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vte::Parser;

    #[test]
    fn prints_text() {
        let mut g = TerminalGrid::new(8, 4);
        let mut p = Parser::new();
        p.advance(&mut g, b"hi");
        assert_eq!(g.cells()[0].ch, 'h');
        assert_eq!(g.cells()[1].ch, 'i');
    }

    #[test]
    fn newline_moves_cursor() {
        let mut g = TerminalGrid::new(8, 4);
        let mut p = Parser::new();
        p.advance(&mut g, b"a\nb");
        assert_eq!(g.cells()[0].ch, 'a');
        assert_eq!(g.cells()[8].ch, 'b');
    }

    #[test]
    fn cursor_visible_defaults_true() {
        let g = TerminalGrid::new(8, 4);
        assert!(g.cursor_visible());
    }

    #[test]
    fn dectcem_hides_and_shows_cursor() {
        let mut g = TerminalGrid::new(8, 4);
        let mut p = Parser::new();
        p.advance(&mut g, b"\x1b[?25l");
        assert!(!g.cursor_visible());
        p.advance(&mut g, b"\x1b[?25h");
        assert!(g.cursor_visible());
    }

    #[test]
    fn unrelated_private_mode_does_not_toggle_cursor() {
        let mut g = TerminalGrid::new(8, 4);
        let mut p = Parser::new();
        p.advance(&mut g, b"\x1b[?25l");
        p.advance(&mut g, b"\x1b[?1049h");
        assert!(
            !g.cursor_visible(),
            "alt-screen must not re-show the cursor"
        );
    }

    #[test]
    fn private_mode_h_is_not_treated_as_a_standard_csi() {
        // `CSI ?25h` must not fall through to any `h`-adjacent standard action,
        // and must leave the cursor position alone.
        let mut g = TerminalGrid::new(8, 4);
        let mut p = Parser::new();
        p.advance(&mut g, b"\x1b[3;5H\x1b[?25l");
        assert_eq!(g.cursor(), (4, 2));
    }

    #[test]
    fn backspace_moves_cursor_without_erasing() {
        // Readline emits a bare \b just to reposition; it must not destroy the cell.
        let mut g = TerminalGrid::new(8, 4);
        let mut p = Parser::new();
        p.advance(&mut g, b"abc\x08\x08");
        assert_eq!(g.cursor(), (1, 0));
        assert_eq!(g.cells()[0].ch, 'a');
        assert_eq!(g.cells()[1].ch, 'b');
        assert_eq!(
            g.cells()[2].ch,
            'c',
            "backspace must not erase what it passes over"
        );
    }

    #[test]
    fn rubout_sequence_still_erases() {
        // The standard way to actually delete: backspace, overwrite, backspace.
        let mut g = TerminalGrid::new(8, 4);
        let mut p = Parser::new();
        p.advance(&mut g, b"ab\x08 \x08");
        assert_eq!(g.cells()[0].ch, 'a');
        assert_eq!(g.cells()[1].ch, ' ');
        assert_eq!(g.cursor(), (1, 0));
    }

    #[test]
    fn backspace_stops_at_column_zero() {
        let mut g = TerminalGrid::new(8, 4);
        let mut p = Parser::new();
        p.advance(&mut g, b"a\x08\x08\x08");
        assert_eq!(g.cursor(), (0, 0));
        assert_eq!(g.cells()[0].ch, 'a');
    }

    #[test]
    fn del_on_output_is_ignored() {
        let mut g = TerminalGrid::new(8, 4);
        let mut p = Parser::new();
        p.advance(&mut g, b"ab\x7f");
        assert_eq!(g.cursor(), (2, 0));
        assert_eq!(g.cells()[1].ch, 'b');
    }

    #[test]
    fn cursor_left_then_overwrite_replaces_in_place() {
        // Moving left over text and typing must replace, not blank-then-write.
        let mut g = TerminalGrid::new(8, 4);
        let mut p = Parser::new();
        p.advance(&mut g, b"abc\x1b[2DX");
        assert_eq!(g.cells()[0].ch, 'a');
        assert_eq!(g.cells()[1].ch, 'X');
        assert_eq!(g.cells()[2].ch, 'c');
    }

    #[test]
    fn cursor_move_marks_dirty() {
        let mut g = TerminalGrid::new(8, 4);
        let mut p = Parser::new();
        g.clear_dirty();
        p.advance(&mut g, b"\x1b[2C");
        assert!(g.is_dirty());
        assert_eq!(g.cursor(), (2, 0));
    }
}
