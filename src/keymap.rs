use rmk::types::action::KeyAction;
use rmk::types::keycode::HidKeyCode;
use rmk::{a, k, layer, mo};

pub(crate) const COL: usize = 17;
pub(crate) const ROW: usize = 6;
pub(crate) const NUM_LAYER: usize = 2;

pub struct KeyMapping {
    pub key_code: u8,
    pub row: u8,
    pub col: u8,
}

pub const KEY_MAPPING: &[KeyMapping] = &[
    KeyMapping {
        key_code: HidKeyCode::Escape as u8,
        row: 0,
        col: 0,
    },
    KeyMapping {
        key_code: HidKeyCode::F1 as u8,
        row: 0,
        col: 1,
    },
    KeyMapping {
        key_code: HidKeyCode::F2 as u8,
        row: 0,
        col: 2,
    },
    KeyMapping {
        key_code: HidKeyCode::F3 as u8,
        row: 0,
        col: 3,
    },
    KeyMapping {
        key_code: HidKeyCode::F4 as u8,
        row: 0,
        col: 4,
    },
    KeyMapping {
        key_code: HidKeyCode::F5 as u8,
        row: 0,
        col: 5,
    },
    KeyMapping {
        key_code: HidKeyCode::F6 as u8,
        row: 0,
        col: 6,
    },
    KeyMapping {
        key_code: HidKeyCode::F7 as u8,
        row: 0,
        col: 7,
    },
    KeyMapping {
        key_code: HidKeyCode::F8 as u8,
        row: 0,
        col: 8,
    },
    KeyMapping {
        key_code: HidKeyCode::F9 as u8,
        row: 0,
        col: 9,
    },
    KeyMapping {
        key_code: HidKeyCode::F10 as u8,
        row: 0,
        col: 10,
    },
    KeyMapping {
        key_code: HidKeyCode::F11 as u8,
        row: 0,
        col: 11,
    },
    KeyMapping {
        key_code: HidKeyCode::F12 as u8,
        row: 0,
        col: 12,
    },
    KeyMapping {
        key_code: HidKeyCode::PrintScreen as u8,
        row: 0,
        col: 13,
    },
    KeyMapping {
        key_code: HidKeyCode::ScrollLock as u8,
        row: 0,
        col: 14,
    },
    KeyMapping {
        key_code: HidKeyCode::Pause as u8,
        row: 0,
        col: 15,
    },
    KeyMapping {
        key_code: HidKeyCode::Grave as u8,
        row: 1,
        col: 0,
    },
    KeyMapping {
        key_code: HidKeyCode::Kc1 as u8,
        row: 1,
        col: 1,
    },
    KeyMapping {
        key_code: HidKeyCode::Kc2 as u8,
        row: 1,
        col: 2,
    },
    KeyMapping {
        key_code: HidKeyCode::Kc3 as u8,
        row: 1,
        col: 3,
    },
    KeyMapping {
        key_code: HidKeyCode::Kc4 as u8,
        row: 1,
        col: 4,
    },
    KeyMapping {
        key_code: HidKeyCode::Kc5 as u8,
        row: 1,
        col: 5,
    },
    KeyMapping {
        key_code: HidKeyCode::Kc6 as u8,
        row: 1,
        col: 6,
    },
    KeyMapping {
        key_code: HidKeyCode::Kc7 as u8,
        row: 1,
        col: 7,
    },
    KeyMapping {
        key_code: HidKeyCode::Kc8 as u8,
        row: 1,
        col: 8,
    },
    KeyMapping {
        key_code: HidKeyCode::Kc9 as u8,
        row: 1,
        col: 9,
    },
    KeyMapping {
        key_code: HidKeyCode::Kc0 as u8,
        row: 1,
        col: 10,
    },
    KeyMapping {
        key_code: HidKeyCode::Minus as u8,
        row: 1,
        col: 11,
    },
    KeyMapping {
        key_code: HidKeyCode::Equal as u8,
        row: 1,
        col: 12,
    },
    KeyMapping {
        key_code: HidKeyCode::Backspace as u8,
        row: 1,
        col: 13,
    },
    KeyMapping {
        key_code: HidKeyCode::Insert as u8,
        row: 1,
        col: 14,
    },
    KeyMapping {
        key_code: HidKeyCode::Home as u8,
        row: 1,
        col: 15,
    },
    KeyMapping {
        key_code: HidKeyCode::PageUp as u8,
        row: 1,
        col: 16,
    },
    KeyMapping {
        key_code: HidKeyCode::Tab as u8,
        row: 2,
        col: 0,
    },
    KeyMapping {
        key_code: HidKeyCode::Q as u8,
        row: 2,
        col: 1,
    },
    KeyMapping {
        key_code: HidKeyCode::W as u8,
        row: 2,
        col: 2,
    },
    KeyMapping {
        key_code: HidKeyCode::E as u8,
        row: 2,
        col: 3,
    },
    KeyMapping {
        key_code: HidKeyCode::R as u8,
        row: 2,
        col: 4,
    },
    KeyMapping {
        key_code: HidKeyCode::T as u8,
        row: 2,
        col: 5,
    },
    KeyMapping {
        key_code: HidKeyCode::Y as u8,
        row: 2,
        col: 6,
    },
    KeyMapping {
        key_code: HidKeyCode::U as u8,
        row: 2,
        col: 7,
    },
    KeyMapping {
        key_code: HidKeyCode::I as u8,
        row: 2,
        col: 8,
    },
    KeyMapping {
        key_code: HidKeyCode::O as u8,
        row: 2,
        col: 9,
    },
    KeyMapping {
        key_code: HidKeyCode::P as u8,
        row: 2,
        col: 10,
    },
    KeyMapping {
        key_code: HidKeyCode::LeftBracket as u8,
        row: 2,
        col: 11,
    },
    KeyMapping {
        key_code: HidKeyCode::RightBracket as u8,
        row: 2,
        col: 12,
    },
    KeyMapping {
        key_code: HidKeyCode::Backslash as u8,
        row: 2,
        col: 13,
    },
    KeyMapping {
        key_code: HidKeyCode::Delete as u8,
        row: 2,
        col: 14,
    },
    KeyMapping {
        key_code: HidKeyCode::End as u8,
        row: 2,
        col: 15,
    },
    KeyMapping {
        key_code: HidKeyCode::PageDown as u8,
        row: 2,
        col: 16,
    },
    KeyMapping {
        key_code: HidKeyCode::LCtrl as u8,
        row: 3,
        col: 0,
    },
    KeyMapping {
        key_code: HidKeyCode::A as u8,
        row: 3,
        col: 1,
    },
    KeyMapping {
        key_code: HidKeyCode::S as u8,
        row: 3,
        col: 2,
    },
    KeyMapping {
        key_code: HidKeyCode::D as u8,
        row: 3,
        col: 3,
    },
    KeyMapping {
        key_code: HidKeyCode::F as u8,
        row: 3,
        col: 4,
    },
    KeyMapping {
        key_code: HidKeyCode::G as u8,
        row: 3,
        col: 5,
    },
    KeyMapping {
        key_code: HidKeyCode::H as u8,
        row: 3,
        col: 6,
    },
    KeyMapping {
        key_code: HidKeyCode::J as u8,
        row: 3,
        col: 7,
    },
    KeyMapping {
        key_code: HidKeyCode::K as u8,
        row: 3,
        col: 8,
    },
    KeyMapping {
        key_code: HidKeyCode::L as u8,
        row: 3,
        col: 9,
    },
    KeyMapping {
        key_code: HidKeyCode::Semicolon as u8,
        row: 3,
        col: 10,
    },
    KeyMapping {
        key_code: HidKeyCode::Quote as u8,
        row: 3,
        col: 11,
    },
    KeyMapping {
        key_code: HidKeyCode::NonusHash as u8,
        row: 3,
        col: 12,
    },
    KeyMapping {
        key_code: HidKeyCode::Enter as u8,
        row: 3,
        col: 13,
    },
    KeyMapping {
        key_code: HidKeyCode::LShift as u8,
        row: 4,
        col: 0,
    },
    KeyMapping {
        key_code: HidKeyCode::Backslash as u8,
        row: 4,
        col: 1,
    },
    KeyMapping {
        key_code: HidKeyCode::Z as u8,
        row: 4,
        col: 2,
    },
    KeyMapping {
        key_code: HidKeyCode::X as u8,
        row: 4,
        col: 3,
    },
    KeyMapping {
        key_code: HidKeyCode::C as u8,
        row: 4,
        col: 4,
    },
    KeyMapping {
        key_code: HidKeyCode::V as u8,
        row: 4,
        col: 5,
    },
    KeyMapping {
        key_code: HidKeyCode::B as u8,
        row: 4,
        col: 6,
    },
    KeyMapping {
        key_code: HidKeyCode::N as u8,
        row: 4,
        col: 7,
    },
    KeyMapping {
        key_code: HidKeyCode::M as u8,
        row: 4,
        col: 8,
    },
    KeyMapping {
        key_code: HidKeyCode::Comma as u8,
        row: 4,
        col: 9,
    },
    KeyMapping {
        key_code: HidKeyCode::Dot as u8,
        row: 4,
        col: 10,
    },
    KeyMapping {
        key_code: HidKeyCode::Slash as u8,
        row: 4,
        col: 11,
    },
    KeyMapping {
        key_code: HidKeyCode::RShift as u8,
        row: 4,
        col: 12,
    },
    KeyMapping {
        key_code: HidKeyCode::Up as u8,
        row: 4,
        col: 15,
    },
    KeyMapping {
        key_code: HidKeyCode::LGui as u8,
        row: 5,
        col: 0,
    },
    KeyMapping {
        key_code: HidKeyCode::LAlt as u8,
        row: 5,
        col: 1,
    },
    KeyMapping {
        key_code: HidKeyCode::Space as u8,
        row: 5,
        col: 5,
    },
    KeyMapping {
        key_code: HidKeyCode::RAlt as u8,
        row: 5,
        col: 9,
    },
    KeyMapping {
        key_code: HidKeyCode::RCtrl as u8,
        row: 5,
        col: 12,
    },
    KeyMapping {
        key_code: HidKeyCode::Right as u8,
        row: 5,
        col: 13,
    },
    KeyMapping {
        key_code: HidKeyCode::Down as u8,
        row: 5,
        col: 14,
    },
    KeyMapping {
        key_code: HidKeyCode::Left as u8,
        row: 5,
        col: 15,
    },
];

pub fn key_code_to_pos(key_code: u8) -> Option<(u8, u8)> {
    KEY_MAPPING
        .iter()
        .find(|m| m.key_code == key_code)
        .map(|m| (m.row, m.col))
}

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        layer!([
            [k!(Escape),k!(F1),k!(F2),k!(F3),k!(F4),k!(F5),k!(F6),k!(F7),k!(F8),k!(F9),k!(F10),k!(F11),k!(F12),k!(PrintScreen),k!(ScrollLock),k!(Pause),a!(No)],
            [k!(Grave),k!(Kc1),k!(Kc2),k!(Kc3),k!(Kc4),k!(Kc5),k!(Kc6),k!(Kc7),k!(Kc8),k!(Kc9),k!(Kc0),k!(Minus),k!(Equal),k!(Backspace),k!(Insert),k!(Home),k!(PageUp)],
            [k!(Tab),k!(Q),k!(W),k!(E),k!(R),k!(T),k!(Y),k!(U),k!(I),k!(O),k!(P),k!(LeftBracket),k!(RightBracket),k!(Backslash),k!(Delete),k!(End),k!(PageDown)],
            [k!(LCtrl),k!(A),k!(S),k!(D),k!(F),k!(G),k!(H),k!(J),k!(K),k!(L),k!(Semicolon),k!(Quote),k!(NonusHash),k!(Enter),a!(No),a!(No),a!(No)],
            [k!(LShift),k!(Backslash),k!(Z),k!(X),k!(C),k!(V),k!(B),k!(N),k!(M),k!(Comma),k!(Dot),k!(Slash),k!(RShift),a!(No),a!(No),k!(Up),a!(No)],
            [k!(LGui),k!(LAlt),a!(No),a!(No),a!(No),k!(Space),a!(No),a!(No),a!(No),k!(RAlt),mo!(1),k!(RGui),k!(RCtrl),k!(Left),k!(Down),k!(Right),a!(No)]
        ]),
        layer!([
            [k!(Escape),k!(F1),k!(F2),k!(F3),k!(F4),k!(F5),k!(F6),k!(F7),k!(F8),k!(F9),k!(F10),k!(F11),k!(F12),k!(PrintScreen),k!(ScrollLock),k!(Pause),a!(No)],
            [k!(Grave),k!(Kc1),k!(Kc2),k!(Kc3),k!(Kc4),k!(Kc5),k!(Kc6),k!(Kc7),k!(Kc8),k!(Kc9),k!(Kc0),k!(Minus),k!(Equal),k!(Backspace),k!(Insert),k!(Home),k!(PageUp)],
            [k!(Tab),k!(Q),k!(W),k!(E),k!(R),k!(T),k!(Y),k!(U),k!(I),k!(O),k!(P),k!(LeftBracket),k!(RightBracket),k!(Backslash),k!(Delete),k!(End),k!(PageDown)],
            [k!(LCtrl),k!(A),k!(S),k!(D),k!(F),k!(G),k!(H),k!(J),k!(K),k!(L),k!(Semicolon),k!(Quote),k!(NonusHash),k!(Enter),a!(No),a!(No),a!(No)],
            [k!(LShift),k!(Backslash),k!(Z),k!(X),k!(C),k!(V),k!(B),k!(N),k!(M),k!(Comma),k!(Dot),k!(Slash),k!(RShift),a!(No),a!(No),k!(Up),a!(No)],
            [k!(LGui),k!(LAlt),a!(No),a!(No),a!(No),k!(Space),a!(No),a!(No),a!(No),k!(RAlt),mo!(1),k!(RGui),k!(RCtrl),k!(Left),k!(Down),k!(Right),a!(No)]
        ]),
    ]
}
