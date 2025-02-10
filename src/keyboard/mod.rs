#![allow(unused_imports)]
#![allow(dead_code)]

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyCode {
    // System codes (0-3)
    Null = 0,
    ErrorRollover = 1,
    PostFail = 2,
    ErrorUndefined = 3,

    // Alphabetic keys (4-29)
    A = 4,
    B = 5,
    C = 6,
    D = 7,
    E = 8,
    F = 9,
    G = 10,
    H = 11,
    I = 12,
    J = 13,
    K = 14,
    L = 15,
    M = 16,
    N = 17,
    O = 18,
    P = 19,
    Q = 20,
    R = 21,
    S = 22,
    T = 23,
    U = 24,
    V = 25,
    W = 26,
    X = 27,
    Y = 28,
    Z = 29,

    // Numeric keys (30-39)
    Key1 = 30,
    Key2 = 31,
    Key3 = 32,
    Key4 = 33,
    Key5 = 34,
    Key6 = 35,
    Key7 = 36,
    Key8 = 37,
    Key9 = 38,
    Key0 = 39,

    // Function keys (58-69, 100-111)
    F1 = 58,
    F2 = 59,
    F3 = 60,
    F4 = 61,
    F5 = 62,
    F6 = 63,
    F7 = 64,
    F8 = 65,
    F9 = 66,
    F10 = 67,
    F11 = 68,
    F12 = 69,
    F13 = 100,
    F14 = 101,
    F15 = 102,
    F16 = 103,
    F17 = 104,
    F18 = 105,
    F19 = 106,
    F20 = 107,
    F21 = 108,
    F22 = 109,
    F23 = 110,
    F24 = 111,

    // Navigation keys
    Up = 82,
    Down = 81,
    Left = 80,
    Right = 79,
    Home = 74,
    End = 77,
    PageUp = 75,
    PageDown = 78,
    Insert = 73,
    Delete = 76,

    // Control keys
    Escape = 41,
    Tab = 43,
    CapsLock = 57,
    Enter = 40,
    Space = 44,
    Backspace = 42,

    // Modifier keys
    LeftShift = 225,
    RightShift = 229,
    LeftCtrl = 224,
    RightCtrl = 228,
    LeftAlt = 226,
    RightAlt = 230,
    LeftWin = 227,

    // Special keys
    PrintScreen = 70,
    ScrollLock = 71,
    Pause = 72,

    // Punctuation keys
    Comma = 54,
    Dot = 55,
    Semicolon = 51,
    Quote = 52,
    LeftBracket = 47,
    RightBracket = 48,
    VerticalLine = 49,
    Wave = 53,
    Equal = 46,
    Minus = 45,
    Question = 56,

    // Numpad keys
    NumLock = 83,
    NumpadDivide = 84,
    NumpadMultiply = 85,
    NumpadSubtract = 86,
    NumpadAdd = 87,
    NumpadEnter = 88,
    Numpad1 = 89,
    Numpad2 = 90,
    Numpad3 = 91,
    Numpad4 = 92,
    Numpad5 = 93,
    Numpad6 = 94,
    Numpad7 = 95,
    Numpad8 = 96,
    Numpad9 = 97,
    Numpad0 = 98,
    NumpadDot = 99,

    // Media keys
    Mute = 231,
    VolumeUp = 232,
    VolumeDown = 233,
    PlayPause = 234,
    MediaStop = 235,
    PreviousSong = 236,
    NextSong = 237,
    Email = 238,
    Calculator = 239,
    MyComputer = 240,
    WwwSearch = 241,
    WwwHome = 242,
    WwwBack = 243,
    WwwForward = 244,
    WwwStop = 245,
    WwwRefresh = 246,
    WwwFavorites = 247,

    // System control keys
    Power = 248,
    Sleep = 249,
    Wake = 250,

    // International keys
    International1 = 251,
    International2 = 252,
    International3 = 253,
    International4 = 254,
    International5 = 255,
}

impl fmt::Display for KeyCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl KeyCode {
    pub fn from_u8(value: u8) -> Option<Self> {
        use std::mem::transmute;
        // Safety: KeyCode enum values exactly match their u8 representations
        match value {
            0..=3
            | 4..=29
            | 30..=39
            | 58..=69
            | 73..=83
            | 84..=99
            | 100..=111
            | 224..=230
            | 231..=247
            | 248..=250
            | 251..=255 => Some(unsafe { transmute(value) }),
            _ => None,
        }
    }

    pub fn to_u8(self) -> u8 {
        self as u8
    }

    pub fn name(&self) -> String {
        self.to_string()
    }
}
