#![allow(unused_imports)]
#![allow(dead_code)]

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyCode {
    None = 0,                // 0x00
    ErrorRollOver = 1,       // 0x01
    PostFail = 2,            // 0x02
    ErrorUndefined = 3,      // 0x03
    A = 4,                   // 0x04
    B = 5,                   // 0x05
    C = 6,                   // 0x06
    D = 7,                   // 0x07
    E = 8,                   // 0x08
    F = 9,                   // 0x09
    G = 10,                  // 0x0A
    H = 11,                  // 0x0B
    I = 12,                  // 0x0C
    J = 13,                  // 0x0D
    K = 14,                  // 0x0E
    L = 15,                  // 0x0F
    M = 16,                  // 0x10
    N = 17,                  // 0x11
    O = 18,                  // 0x12
    P = 19,                  // 0x13
    Q = 20,                  // 0x14
    R = 21,                  // 0x15
    S = 22,                  // 0x16
    T = 23,                  // 0x17
    U = 24,                  // 0x18
    V = 25,                  // 0x19
    W = 26,                  // 0x1A
    X = 27,                  // 0x1B
    Y = 28,                  // 0x1C
    Z = 29,                  // 0x1D
    Key1 = 30,               // 0x1E
    Key2 = 31,               // 0x1F
    Key3 = 32,               // 0x20
    Key4 = 33,               // 0x21
    Key5 = 34,               // 0x22
    Key6 = 35,               // 0x23
    Key7 = 36,               // 0x24
    Key8 = 37,               // 0x25
    Key9 = 38,               // 0x26
    Key0 = 39,               // 0x27
    Enter = 40,              // 0x28
    Escape = 41,             // 0x29
    Backspace = 42,          // 0x2A
    Tab = 43,                // 0x2B
    Space = 44,              // 0x2C
    Minus = 45,              // 0x2D
    Equal = 46,              // 0x2E
    LeftBracket = 47,        // 0x2F
    RightBracket = 48,       // 0x30
    Backslash = 49,          // 0x31
    HashTilde = 50,          // 0x32
    Semicolon = 51,          // 0x33
    Apostrophe = 52,         // 0x34
    Grave = 53,              // 0x35
    Comma = 54,              // 0x36
    Dot = 55,                // 0x37
    Slash = 56,              // 0x38
    CapsLock = 57,           // 0x39
    F1 = 58,                 // 0x3A
    F2 = 59,                 // 0x3B
    F3 = 60,                 // 0x3C
    F4 = 61,                 // 0x3D
    F5 = 62,                 // 0x3E
    F6 = 63,                 // 0x3F
    F7 = 64,                 // 0x40
    F8 = 65,                 // 0x41
    F9 = 66,                 // 0x42
    F10 = 67,                // 0x43
    F11 = 68,                // 0x44
    F12 = 69,                // 0x45
    PrintScreen = 70,        // 0x46
    ScrollLock = 71,         // 0x47
    Pause = 72,              // 0x48
    Insert = 73,             // 0x49
    Home = 74,               // 0x4A
    PageUp = 75,             // 0x4B
    Delete = 76,             // 0x4C
    End = 77,                // 0x4D
    PageDown = 78,           // 0x4E
    Right = 79,              // 0x4F
    Left = 80,               // 0x50
    Down = 81,               // 0x51
    Up = 82,                 // 0x52
    NumLock = 83,            // 0x53
    KpSlash = 84,            // 0x54
    KpAsterisk = 85,         // 0x55
    KpMinus = 86,            // 0x56
    KpPlus = 87,             // 0x57
    KpEnter = 88,            // 0x58
    Kp1 = 89,                // 0x59
    Kp2 = 90,                // 0x5A
    Kp3 = 91,                // 0x5B
    Kp4 = 92,                // 0x5C
    Kp5 = 93,                // 0x5D
    Kp6 = 94,                // 0x5E
    Kp7 = 95,                // 0x5F
    Kp8 = 96,                // 0x60
    Kp9 = 97,                // 0x61
    Kp0 = 98,                // 0x62
    KpDot = 99,              // 0x63
    Key102nd = 100,          // 0x64
    Compose = 101,           // 0x65
    Power = 102,             // 0x66
    KpEqual = 103,           // 0x67
    F13 = 104,               // 0x68
    F14 = 105,               // 0x69
    F15 = 106,               // 0x6A
    F16 = 107,               // 0x6B
    F17 = 108,               // 0x6C
    F18 = 109,               // 0x6D
    F19 = 110,               // 0x6E
    F20 = 111,               // 0x6F
    F21 = 112,               // 0x70
    F22 = 113,               // 0x71
    F23 = 114,               // 0x72
    F24 = 115,               // 0x73
    Open = 116,              // 0x74
    Help = 117,              // 0x75
    Props = 118,             // 0x76
    Front = 119,             // 0x77
    Stop = 120,              // 0x78
    Again = 121,             // 0x79
    Undo = 122,              // 0x7A
    Cut = 123,               // 0x7B
    Copy = 124,              // 0x7C
    Paste = 125,             // 0x7D
    Find = 126,              // 0x7E
    Mute = 127,              // 0x7F
    VolumeUp = 128,          // 0x80
    VolumeDown = 129,        // 0x81
    LockingCapsLock = 130,   // 0x82
    LockingNumLock = 131,    // 0x83
    LockingScrollLock = 132, // 0x84
    KpComma = 133,           // 0x85
    KpEqualSign = 134,       // 0x86
    Ro = 135,                // 0x87
    KatakanaHiragana = 136,  // 0x88
    Yen = 137,               // 0x89
    Henkan = 138,            // 0x8A
    Muhenkan = 139,          // 0x8B
    KpJpComma = 140,         // 0x8C
    International7 = 141,    // 0x8D
    International8 = 142,    // 0x8E
    International9 = 143,    // 0x8F
    Hangeul = 144,           // 0x90
    Hanja = 145,             // 0x91
    Katakana = 146,          // 0x92
    Hiragana = 147,          // 0x93
    ZenkakuHankaku = 148,    // 0x94
    Lang6 = 149,             // 0x95
    Lang7 = 150,             // 0x96
    Lang8 = 151,             // 0x97
    Lang9 = 152,             // 0x98
    AlternateErase = 153,    // 0x99
    SysReqAttention = 154,   // 0x9A
    Cancel = 155,            // 0x9B
    Clear = 156,             // 0x9C
    Prior = 157,             // 0x9D
    Return = 158,            // 0x9E
    Separator = 159,         // 0x9F
    Out = 160,               // 0xA0
    Oper = 161,              // 0xA1
    ClearAgain = 162,        // 0xA2
    CrSelProps = 163,        // 0xA3
    ExSel = 164,             // 0xA4
    Reserved165 = 165,       // 0xA5
    Reserved166 = 166,       // 0xA6
    Reserved167 = 167,       // 0xA7
    Reserved168 = 168,       // 0xA8
    Reserved169 = 169,       // 0xA9
    Reserved170 = 170,       // 0xAA
    Reserved171 = 171,       // 0xAB
    Reserved172 = 172,       // 0xAC
    Reserved173 = 173,       // 0xAD
    Reserved174 = 174,       // 0xAE
    Reserved175 = 175,       // 0xAF
    Reserved176 = 176,       // 0xB0
    Reserved177 = 177,       // 0xB1
    Reserved178 = 178,       // 0xB2
    Reserved179 = 179,       // 0xB3
    Reserved180 = 180,       // 0xB4
    Reserved181 = 181,       // 0xB5
    KpLeftParen = 182,       // 0xB6
    KpRightParen = 183,      // 0xB7
    Reserved184 = 184,       // 0xB8
    Reserved185 = 185,       // 0xB9
    Reserved186 = 186,       // 0xBA
    Reserved187 = 187,       // 0xBB
    Reserved188 = 188,       // 0xBC
    Reserved189 = 189,       // 0xBD
    Reserved190 = 190,       // 0xBE
    Reserved191 = 191,       // 0xBF
    Reserved192 = 192,       // 0xC0
    Reserved193 = 193,       // 0xC1
    Reserved194 = 194,       // 0xC2
    Reserved195 = 195,       // 0xC3
    Reserved196 = 196,       // 0xC4
    Reserved197 = 197,       // 0xC5
    Reserved198 = 198,       // 0xC6
    Reserved199 = 199,       // 0xC7
    Reserved200 = 200,       // 0xC8
    Reserved201 = 201,       // 0xC9
    Reserved202 = 202,       // 0xCA
    Reserved203 = 203,       // 0xCB
    Reserved204 = 204,       // 0xCC
    Reserved205 = 205,       // 0xCD
    Reserved206 = 206,       // 0xCE
    Reserved207 = 207,       // 0xCF
    Reserved208 = 208,       // 0xD0
    Reserved209 = 209,       // 0xD1
    Reserved210 = 210,       // 0xD2
    Reserved211 = 211,       // 0xD3
    Reserved212 = 212,       // 0xD4
    Reserved213 = 213,       // 0xD5
    Reserved214 = 214,       // 0xD6
    Reserved215 = 215,       // 0xD7
    Reserved216 = 216,       // 0xD8
    Reserved217 = 217,       // 0xD9
    Reserved218 = 218,       // 0xDA
    Reserved219 = 219,       // 0xDB
    Reserved220 = 220,       // 0xDC
    Reserved221 = 221,       // 0xDD
    Reserved222 = 222,       // 0xDE
    Reserved223 = 223,       // 0xDF
    LeftCtrl = 224,          // 0xE0
    LeftShift = 225,         // 0xE1
    LeftAlt = 226,           // 0xE2
    LeftMeta = 227,          // 0xE3
    RightCtrl = 228,         // 0xE4
    RightShift = 229,        // 0xE5
    RightAlt = 230,          // 0xE6
    RightMeta = 231,         // 0xE7
    MediaPlayPause = 232,    // 0xE8
    MediaStopCD = 233,       // 0xE9
    MediaPreviousSong = 234, // 0xEA
    MediaNextSong = 235,     // 0xEB
    MediaEjectCD = 236,      // 0xEC
    MediaVolumeUp = 237,     // 0xED
    MediaVolumeDown = 238,   // 0xEE
    MediaMute = 239,         // 0xEF
    MediaWWW = 240,          // 0xF0
    MediaBack = 241,         // 0xF1
    MediaForward = 242,      // 0xF2
    MediaStop = 243,         // 0xF3
    MediaFind = 244,         // 0xF4
    MediaScrollUp = 245,     // 0xF5
    MediaScrollDown = 246,   // 0xF6
    MediaEdit = 247,         // 0xF7
    MediaSleep = 248,        // 0xF8
    MediaCoffee = 249,       // 0xF9
    MediaRefresh = 250,      // 0xFA
    MediaCalc = 251,         // 0xFB
    Reserved252 = 252,       // 0xFC
    Reserved253 = 253,       // 0xFD
    Reserved254 = 254,       // 0xFE
    Reserved255 = 255,       // 0xFF
}

impl fmt::Display for KeyCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl KeyCode {
    pub fn from_u8(value: u8) -> Option<Self> {
        use std::mem::transmute;
        Some(unsafe { transmute(value) })
    }

    pub fn to_u8(self) -> u8 {
        self as u8
    }

    pub fn name(&self) -> String {
        format!("{:?}", self)
    }
}
