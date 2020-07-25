use num_derive::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Symbolic names for the keycodes used by Gms2. These are a cutdown version of the Microsoft
/// Virtual keycodes from `winuser.h`, and with Vk_Any and Vk_None (custom Gms2) added.
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
