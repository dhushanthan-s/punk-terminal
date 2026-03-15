//! Key parser: raw bytes -> KeyEvent. Uses only std. Supports control, CSI (with params), SS3, and UTF-8.

use super::Key;
use super::Key::*;
use super::KeyEvent;
use super::Modifiers;

/// Result of parsing the input buffer: either a key event plus bytes consumed, or need more input.
#[derive(Debug)]
pub enum ParseResult<'a> {
    Consumed(KeyEvent<'a>, usize),
    NeedMore,
}

/// Expected UTF-8 sequence length from the first byte (1-4), or None if invalid lead byte.
#[inline]
fn utf8_expected_len(first: u8) -> Option<usize> {
    match first {
        0x00..=0x7F => Some(1),
        0xC2..=0xDF => Some(2),
        0xE0..=0xEF => Some(3),
        0xF0..=0xF4 => Some(4),
        _ => None,
    }
}

/// Map CSI modifier parameter (Pm) to Modifiers.
///
/// xterm encodes `Pm = 1 + modifier_bits` where bits are: Shift=1, Alt=2, Ctrl=4, Meta/Super=8.
/// Examples: Pm=1 → none, Pm=2 → Shift, Pm=3 → Alt, Pm=5 → Ctrl, Pm=6 → Shift+Ctrl.
fn csi_modifier_to_modifiers(pm: u8) -> Modifiers {
    let bits = pm.saturating_sub(1);
    Modifiers {
        shift: bits & 1 != 0,
        alt: bits & 2 != 0,
        ctrl: bits & 4 != 0,
        super_: bits & 8 != 0,
/// Map CSI modifier parameter (Pm) to Modifiers. xterm-style: 1=Shift, 2=Alt, 3=Alt+Shift, 4=Ctrl, 5=Ctrl+Shift, 6=Alt+Ctrl, 7=all, 8=Super.
    }
}

/// Parse a single key from the front of `buf`. Returns KeyEvent and bytes consumed, or NeedMore.
pub fn parse_key(buf: &[u8]) -> ParseResult<'static> {
    if buf.is_empty() {
        return ParseResult::NeedMore;
    }

    let b0 = buf[0];

    // Single-byte: control and printable ASCII
    if b0 <= 0x7F {
        return match b0 {
            0 => ParseResult::Consumed(KeyEvent::new(Modifiers::ctrl(), Letter('a')), 1),
            1 => ParseResult::Consumed(KeyEvent::new(Modifiers::ctrl(), Letter('b')), 1),
            2 => ParseResult::Consumed(KeyEvent::new(Modifiers::ctrl(), Letter('c')), 1),
            3 => ParseResult::Consumed(KeyEvent::new(Modifiers::ctrl(), Letter('d')), 1),
            4 => ParseResult::Consumed(KeyEvent::new(Modifiers::ctrl(), Letter('e')), 1),
            5 => ParseResult::Consumed(KeyEvent::new(Modifiers::ctrl(), Letter('f')), 1),
            6 => ParseResult::Consumed(KeyEvent::new(Modifiers::ctrl(), Letter('g')), 1),
            7 => ParseResult::Consumed(KeyEvent::new(Modifiers::ctrl(), Letter('h')), 1),
            8 => ParseResult::Consumed(KeyEvent::new(Modifiers::new(), Backspace), 1),
            9 => ParseResult::Consumed(KeyEvent::new(Modifiers::new(), Tab), 1),
            10 => ParseResult::Consumed(KeyEvent::new(Modifiers::new(), Enter), 1),
            11 => ParseResult::Consumed(KeyEvent::new(Modifiers::ctrl(), Letter('k')), 1),
            12 => ParseResult::Consumed(KeyEvent::new(Modifiers::ctrl(), Letter('l')), 1),
            13 => ParseResult::Consumed(KeyEvent::new(Modifiers::new(), Enter), 1),
            14 => ParseResult::Consumed(KeyEvent::new(Modifiers::ctrl(), Letter('n')), 1),
            15 => ParseResult::Consumed(KeyEvent::new(Modifiers::ctrl(), Letter('o')), 1),
            16 => ParseResult::Consumed(KeyEvent::new(Modifiers::ctrl(), Letter('p')), 1),
            17 => ParseResult::Consumed(KeyEvent::new(Modifiers::ctrl(), Letter('q')), 1),
            18 => ParseResult::Consumed(KeyEvent::new(Modifiers::ctrl(), Letter('r')), 1),
            19 => ParseResult::Consumed(KeyEvent::new(Modifiers::ctrl(), Letter('s')), 1),
            20 => ParseResult::Consumed(KeyEvent::new(Modifiers::ctrl(), Letter('t')), 1),
            21 => ParseResult::Consumed(KeyEvent::new(Modifiers::ctrl(), Letter('u')), 1),
            22 => ParseResult::Consumed(KeyEvent::new(Modifiers::ctrl(), Letter('v')), 1),
            23 => ParseResult::Consumed(KeyEvent::new(Modifiers::ctrl(), Letter('w')), 1),
            24 => ParseResult::Consumed(KeyEvent::new(Modifiers::ctrl(), Letter('x')), 1),
            25 => ParseResult::Consumed(KeyEvent::new(Modifiers::ctrl(), Letter('y')), 1),
            27 => parse_esc(buf),
            32..=126 => {
                ParseResult::Consumed(KeyEvent::new(Modifiers::new(), Letter(b0 as char)), 1)
            }
            127 => ParseResult::Consumed(KeyEvent::new(Modifiers::new(), Delete), 1),
            _ => ParseResult::Consumed(KeyEvent::new(Modifiers::new(), Unknown), 1),
        };
    }

    // 0x80..=0xBF continuation; 0xF5+ invalid
    if (0x80..=0xBF).contains(&b0) {
        return ParseResult::Consumed(KeyEvent::new(Modifiers::new(), Unknown), 1);
    }
    if b0 >= 0xF5 {
        return ParseResult::Consumed(KeyEvent::new(Modifiers::new(), Unknown), 1);
    }

    // Multi-byte UTF-8
    let expected = match utf8_expected_len(b0) {
        Some(n) => n,
        None => return ParseResult::Consumed(KeyEvent::new(Modifiers::new(), Unknown), 1),
    };
    if buf.len() < expected {
        return ParseResult::NeedMore;
    }
    match std::str::from_utf8(&buf[..expected]) {
        Ok(s) => {
            if let Some(c) = s.chars().next() {
                ParseResult::Consumed(KeyEvent::new(Modifiers::new(), Letter(c)), expected)
            } else {
                ParseResult::Consumed(KeyEvent::new(Modifiers::new(), Unknown), 1)
            }
        }
        Err(_) => ParseResult::Consumed(KeyEvent::new(Modifiers::new(), Unknown), 1),
    }
}

/// Parse after ESC: SS3 (ESC O X) for F1-F4, or CSI (ESC [ ...) for arrows/F-keys/Home/End with optional modifier.
fn parse_esc(buf: &[u8]) -> ParseResult<'static> {
    if buf.len() < 2 {
        return ParseResult::NeedMore;
    }
    if buf[1] == b'O' {
        return parse_ss3(buf);
    }
    if buf[1] != b'[' {
        return ParseResult::Consumed(KeyEvent::new(Modifiers::new(), Esc), 1);
    }
    parse_csi(buf)
}

/// SS3: ESC O P/Q/R/S = F1/F2/F3/F4.
fn parse_ss3(buf: &[u8]) -> ParseResult<'static> {
    if buf.len() < 3 {
        return ParseResult::NeedMore;
    }
    let key = match buf[2] {
        b'P' => Key::F(1),
        b'Q' => Key::F(2),
        b'R' => Key::F(3),
        b'S' => Key::F(4),
        _ => return ParseResult::Consumed(KeyEvent::new(Modifiers::new(), Esc), 1),
    };
    ParseResult::Consumed(KeyEvent::new(Modifiers::new(), key), 3)
}

/// CSI: ESC [ [parameters] final. Parameters are digits and ; (e.g. 11, 11;5, 1;5). Final is A-D (arrows) or ~ (F-keys, Home, End).
fn parse_csi(buf: &[u8]) -> ParseResult<'static> {
    let body = &buf[2..];
    let mut i = 0;
    let mut ps: u16 = 0;
    let mut pm: u8 = 0;
    let mut has_ps = false;
    let mut has_pm = false;

    while i < body.len() {
        let b = body[i];
        if b.is_ascii_digit() {
            let mut n: u16 = 0;
            while i < body.len() && body[i].is_ascii_digit() {
                n = n.saturating_mul(10).saturating_add((body[i] - b'0') as u16);
                i += 1;
            }
            if !has_ps {
                ps = n;
                has_ps = true;
            } else if !has_pm {
                pm = n.min(255) as u8;
                has_pm = true;
            }
            continue;
        }
        if b == b';' {
            i += 1;
            continue;
        }
        if b == b'A' || b == b'B' || b == b'C' || b == b'D' {
            let key = match b {
                b'A' => Arrow("up"),
                b'B' => Arrow("down"),
                b'C' => Arrow("right"),
                b'D' => Arrow("left"),
                _ => unreachable!(),
            };
            let mods = if has_pm {
                csi_modifier_to_modifiers(pm)
            } else {
                Modifiers::new()
            };
            return ParseResult::Consumed(KeyEvent::new(mods, key), 2 + i + 1);
        }
        if b == b'~' {
            let key = csi_tilde_key(ps);
            let mods = if has_pm {
                csi_modifier_to_modifiers(pm)
            } else {
                Modifiers::new()
            };
            return ParseResult::Consumed(KeyEvent::new(mods, key), 2 + i + 1);
        }
        i += 1;
    }
    ParseResult::NeedMore
}

/// Map CSI Ps for ~-terminated sequences to Key. F1=11, F2=12, F5=15, F6=17, F7=18, F8=19, F9=20, F10=21, F11=23, F12=24; Home=1, End=4.
fn csi_tilde_key(ps: u16) -> Key<'static> {
    match ps {
        1 => Home,
        4 => End,
        11 => F(1),
        12 => F(2),
        15 => F(5),
        17 => F(6),
        18 => F(7),
        19 => F(8),
        20 => F(9),
        21 => F(10),
        23 => F(11),
        24 => F(12),
        _ => Unknown,
    }
}

#[cfg(test)]
mod csi_modifier_tests {
    use super::super::Modifiers;
    use super::csi_modifier_to_modifiers;

    #[test]
    fn xterm_pm_is_one_plus_bitmask() {
        assert_eq!(csi_modifier_to_modifiers(1), Modifiers::new());
        assert_eq!(csi_modifier_to_modifiers(2), Modifiers::shift());
        assert_eq!(csi_modifier_to_modifiers(3), Modifiers::alt());
        assert_eq!(
            csi_modifier_to_modifiers(4),
            Modifiers {
                shift: true,
                alt: true,
                ..Modifiers::new()
            }
        );
        assert_eq!(csi_modifier_to_modifiers(5), Modifiers::ctrl());
        assert_eq!(
            csi_modifier_to_modifiers(6),
            Modifiers {
                ctrl: true,
                shift: true,
                ..Modifiers::new()
            }
        );
        assert_eq!(
            csi_modifier_to_modifiers(7),
            Modifiers {
                ctrl: true,
                alt: true,
                ..Modifiers::new()
            }
        );
        assert_eq!(
            csi_modifier_to_modifiers(8),
            Modifiers {
                ctrl: true,
                shift: true,
                alt: true,
                ..Modifiers::new()
            }
        );
        assert_eq!(csi_modifier_to_modifiers(9), Modifiers::super_());
        assert_eq!(
            csi_modifier_to_modifiers(10),
            Modifiers {
                super_: true,
                shift: true,
                ..Modifiers::new()
            }
        );
    }

    #[test]
    fn pm_zero_saturates_to_no_modifiers() {
        assert_eq!(csi_modifier_to_modifiers(0), Modifiers::new());
    }
}
