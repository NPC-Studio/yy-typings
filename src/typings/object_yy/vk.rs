use num_derive::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};
use smart_default::SmartDefault;

/// Symbolic names for the keycodes used by Gms2. These are a cutdown version of
/// the Microsoft Virtual keycodes from `winuser.h`, and with Vk_Any and Vk_None
/// (custom Gms2) added.
#[derive(
    Debug,
    Hash,
    Ord,
    PartialOrd,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Deserialize_repr,
    Serialize_repr,
    FromPrimitive,
)]
#[repr(u32)]
pub enum VirtualKeyCode {
    NoKey = 0,
    AnyKey = 1,
    Backspace = 0x08,
    Tab,
    Return = 0x0D,
    Shift = 0x10,
    Control,
    Alt,
    Pause,
    Escape = 0x1B,
    Space = 0x20,
    PageUp,
    PageDown,
    End,
    Home,
    Left,
    Up,
    Right,
    Down,
    Insert = 0x2D,
    Delete,
    Zero = 0x30,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A = 0x41,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    Numpad0 = 0x60,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadMultiply,
    NumpadAdd,
    NumpadSubtract = 0x6D,
    NumpadDeciminal,
    NumpadDivide,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
}

impl AsRef<str> for VirtualKeyCode {
    fn as_ref(&self) -> &str {
        match self {
            VirtualKeyCode::NoKey => "NoKey",
            VirtualKeyCode::AnyKey => "AnyKey",
            VirtualKeyCode::Backspace => "Backspace",
            VirtualKeyCode::Tab => "Tab",
            VirtualKeyCode::Return => "Return",
            VirtualKeyCode::Shift => "Shift",
            VirtualKeyCode::Control => "Control",
            VirtualKeyCode::Alt => "Alt",
            VirtualKeyCode::Pause => "Pause",
            VirtualKeyCode::Escape => "Escape",
            VirtualKeyCode::Space => "Space",
            VirtualKeyCode::PageUp => "PageUp",
            VirtualKeyCode::PageDown => "PageDown",
            VirtualKeyCode::End => "End",
            VirtualKeyCode::Home => "Home",
            VirtualKeyCode::Left => "Left",
            VirtualKeyCode::Up => "Up",
            VirtualKeyCode::Right => "Right",
            VirtualKeyCode::Down => "Down",
            VirtualKeyCode::Insert => "Insert",
            VirtualKeyCode::Delete => "Delete",
            VirtualKeyCode::Zero => "Zero",
            VirtualKeyCode::One => "One",
            VirtualKeyCode::Two => "Two",
            VirtualKeyCode::Three => "Three",
            VirtualKeyCode::Four => "Four",
            VirtualKeyCode::Five => "Five",
            VirtualKeyCode::Six => "Six",
            VirtualKeyCode::Seven => "Seven",
            VirtualKeyCode::Eight => "Eight",
            VirtualKeyCode::Nine => "Nine",
            VirtualKeyCode::A => "A",
            VirtualKeyCode::B => "B",
            VirtualKeyCode::C => "C",
            VirtualKeyCode::D => "D",
            VirtualKeyCode::E => "E",
            VirtualKeyCode::F => "F",
            VirtualKeyCode::G => "G",
            VirtualKeyCode::H => "H",
            VirtualKeyCode::I => "I",
            VirtualKeyCode::J => "J",
            VirtualKeyCode::K => "K",
            VirtualKeyCode::L => "L",
            VirtualKeyCode::M => "M",
            VirtualKeyCode::N => "N",
            VirtualKeyCode::O => "O",
            VirtualKeyCode::P => "P",
            VirtualKeyCode::Q => "Q",
            VirtualKeyCode::R => "R",
            VirtualKeyCode::S => "S",
            VirtualKeyCode::T => "T",
            VirtualKeyCode::U => "U",
            VirtualKeyCode::V => "V",
            VirtualKeyCode::W => "W",
            VirtualKeyCode::X => "X",
            VirtualKeyCode::Y => "Y",
            VirtualKeyCode::Z => "Z",
            VirtualKeyCode::Numpad0 => "Numpad0",
            VirtualKeyCode::Numpad1 => "Numpad1",
            VirtualKeyCode::Numpad2 => "Numpad2",
            VirtualKeyCode::Numpad3 => "Numpad3",
            VirtualKeyCode::Numpad4 => "Numpad4",
            VirtualKeyCode::Numpad5 => "Numpad5",
            VirtualKeyCode::Numpad6 => "Numpad6",
            VirtualKeyCode::Numpad7 => "Numpad7",
            VirtualKeyCode::Numpad8 => "Numpad8",
            VirtualKeyCode::Numpad9 => "Numpad9",
            VirtualKeyCode::NumpadMultiply => "NumpadMultiply",
            VirtualKeyCode::NumpadAdd => "NumpadAdd",
            VirtualKeyCode::NumpadSubtract => "NumpadSubtract",
            VirtualKeyCode::NumpadDeciminal => "NumpadDeciminal",
            VirtualKeyCode::NumpadDivide => "NumpadDivide",
            VirtualKeyCode::F1 => "F1",
            VirtualKeyCode::F2 => "F2",
            VirtualKeyCode::F3 => "F3",
            VirtualKeyCode::F4 => "F4",
            VirtualKeyCode::F5 => "F5",
            VirtualKeyCode::F6 => "F6",
            VirtualKeyCode::F7 => "F7",
            VirtualKeyCode::F8 => "F8",
            VirtualKeyCode::F9 => "F9",
            VirtualKeyCode::F10 => "F10",
            VirtualKeyCode::F11 => "F11",
            VirtualKeyCode::F12 => "F12",
        }
    }
}

/// Symbolic names for the mouse buttons used by Gms2.
#[derive(
    Debug,
    Hash,
    Ord,
    PartialOrd,
    PartialEq,
    Eq,
    Clone,
    Copy,
    SmartDefault,
    Deserialize_repr,
    Serialize_repr,
    FromPrimitive,
)]
#[repr(u8)]
pub enum MouseButtonCode {
    #[default]
    Left,
    Right,
    Middle,
}

impl MouseButtonCode {
    #[cfg(test)]
    pub(crate) const COUNT: usize = 3;
}

/// Symbolic names for the Gestures used in Gms2
#[derive(
    Debug,
    Hash,
    Ord,
    PartialOrd,
    PartialEq,
    Eq,
    Clone,
    Copy,
    SmartDefault,
    Deserialize_repr,
    Serialize_repr,
    FromPrimitive,
)]
#[repr(u8)]
pub enum Gesture {
    #[default]
    Tap,
    DoubleTap,
    DragStart,
    Dragging,
    DragEnd,
    Flick,
    PinchStart,
    PinchIn,
    PinchOut,
    PinchEnd,
    RotateStart,
    Rotating,
    RotateEnd,
}

impl Gesture {
    #[cfg(test)]
    pub(crate) const COUNT: usize = 12;
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn sanity_checks() {
        assert_eq!(VirtualKeyCode::Space as usize, 32);
        assert_eq!(VirtualKeyCode::Seven as usize, 55);
        assert_eq!(VirtualKeyCode::Right as usize, 39);
        assert_eq!(VirtualKeyCode::NumpadAdd as usize, 107);
        assert_eq!(VirtualKeyCode::Delete as usize, 46);
        assert_eq!(VirtualKeyCode::Backspace as usize, 8);
        assert_eq!(VirtualKeyCode::P as usize, 80);
    }
}
