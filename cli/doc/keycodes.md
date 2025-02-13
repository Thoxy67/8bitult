# USB HID Keyboard Codes Documentation

## Important Note About Keyboard Layouts
This documentation is based on the US QWERTY keyboard layout. When using these keycodes with different keyboard layouts (like AZERTY, QWERTZ, etc.), you must refer to the physical position of the key on a US QWERTY keyboard, not the character it produces.

## Complete Keycode Reference Table
Below is the complete reference table of all available keycodes. Remember that these codes correspond to physical key positions on a US QWERTY keyboard.

| Dec | Hex  | Constant Name | Description |
|-----|------|--------------|-------------|
| 0 | 0x00 | None | No key pressed |
| 1 | 0x01 | ErrorRollOver | Keyboard Error Roll Over |
| 2 | 0x02 | PostFail | POST Failed |
| 3 | 0x03 | ErrorUndefined | Error Undefined |
| 4 | 0x04 | A | Letter 'A' |
| 5 | 0x05 | B | Letter 'B' |
| 6 | 0x06 | C | Letter 'C' |
| 7 | 0x07 | D | Letter 'D' |
| 8 | 0x08 | E | Letter 'E' |
| 9 | 0x09 | F | Letter 'F' |
| 10 | 0x0A | G | Letter 'G' |
| 11 | 0x0B | H | Letter 'H' |
| 12 | 0x0C | I | Letter 'I' |
| 13 | 0x0D | J | Letter 'J' |
| 14 | 0x0E | K | Letter 'K' |
| 15 | 0x0F | L | Letter 'L' |
| 16 | 0x10 | M | Letter 'M' |
| 17 | 0x11 | N | Letter 'N' |
| 18 | 0x12 | O | Letter 'O' |
| 19 | 0x13 | P | Letter 'P' |
| 20 | 0x14 | Q | Letter 'Q' |
| 21 | 0x15 | R | Letter 'R' |
| 22 | 0x16 | S | Letter 'S' |
| 23 | 0x17 | T | Letter 'T' |
| 24 | 0x18 | U | Letter 'U' |
| 25 | 0x19 | V | Letter 'V' |
| 26 | 0x1A | W | Letter 'W' |
| 27 | 0x1B | X | Letter 'X' |
| 28 | 0x1C | Y | Letter 'Y' |
| 29 | 0x1D | Z | Letter 'Z' |
| 30 | 0x1E | Key1 | Number '1' |
| 31 | 0x1F | Key2 | Number '2' |
| 32 | 0x20 | Key3 | Number '3' |
| 33 | 0x21 | Key4 | Number '4' |
| 34 | 0x22 | Key5 | Number '5' |
| 35 | 0x23 | Key6 | Number '6' |
| 36 | 0x24 | Key7 | Number '7' |
| 37 | 0x25 | Key8 | Number '8' |
| 38 | 0x26 | Key9 | Number '9' |
| 39 | 0x27 | Key0 | Number '0' |
| 40 | 0x28 | Enter | Enter/Return key |
| 41 | 0x29 | Escape | Escape key |
| 42 | 0x2A | Backspace | Backspace |
| 43 | 0x2B | Tab | Tab key |
| 44 | 0x2C | Space | Spacebar |
| 45 | 0x2D | Minus | Minus (-) |
| 46 | 0x2E | Equal | Equal (=) |
| 47 | 0x2F | LeftBracket | Left bracket ([) |
| 48 | 0x30 | RightBracket | Right bracket (]) |
| 49 | 0x31 | Backslash | Backslash (\\) |
| 50 | 0x32 | HashTilde | Non-US # and ~ |
| 51 | 0x33 | Semicolon | Semicolon (;) |
| 52 | 0x34 | Apostrophe | Quote (') |
| 53 | 0x35 | Grave | Grave accent and tilde |
| 54 | 0x36 | Comma | Comma (,) |
| 55 | 0x37 | Dot | Period (.) |
| 56 | 0x38 | Slash | Forward Slash (/) |
| 57 | 0x39 | CapsLock | Caps Lock |
| 58 | 0x3A | F1 | F1 |
| 59 | 0x3B | F2 | F2 |
| 60 | 0x3C | F3 | F3 |
| 61 | 0x3D | F4 | F4 |
| 62 | 0x3E | F5 | F5 |
| 63 | 0x3F | F6 | F6 |
| 64 | 0x40 | F7 | F7 |
| 65 | 0x41 | F8 | F8 |
| 66 | 0x42 | F9 | F9 |
| 67 | 0x43 | F10 | F10 |
| 68 | 0x44 | F11 | F11 |
| 69 | 0x45 | F12 | F12 |
| 70 | 0x46 | PrintScreen | Print Screen |
| 71 | 0x47 | ScrollLock | Scroll Lock |
| 72 | 0x48 | Pause | Pause |
| 73 | 0x49 | Insert | Insert |
| 74 | 0x4A | Home | Home |
| 75 | 0x4B | PageUp | Page Up |
| 76 | 0x4C | Delete | Delete Forward |
| 77 | 0x4D | End | End |
| 78 | 0x4E | PageDown | Page Down |
| 79 | 0x4F | Right | Right Arrow |
| 80 | 0x50 | Left | Left Arrow |
| 81 | 0x51 | Down | Down Arrow |
| 82 | 0x52 | Up | Up Arrow |
| 83 | 0x53 | NumLock | Num Lock |
| 84 | 0x54 | KpSlash | Keypad / |
| 85 | 0x55 | KpAsterisk | Keypad * |
| 86 | 0x56 | KpMinus | Keypad - |
| 87 | 0x57 | KpPlus | Keypad + |
| 88 | 0x58 | KpEnter | Keypad Enter |
| 89 | 0x59 | Kp1 | Keypad 1 and End |
| 90 | 0x5A | Kp2 | Keypad 2 and Down Arrow |
| 91 | 0x5B | Kp3 | Keypad 3 and PageDn |
| 92 | 0x5C | Kp4 | Keypad 4 and Left Arrow |
| 93 | 0x5D | Kp5 | Keypad 5 |
| 94 | 0x5E | Kp6 | Keypad 6 and Right Arrow |
| 95 | 0x5F | Kp7 | Keypad 7 and Home |
| 96 | 0x60 | Kp8 | Keypad 8 and Up Arrow |
| 97 | 0x61 | Kp9 | Keypad 9 and Page Up |
| 98 | 0x62 | Kp0 | Keypad 0 and Insert |
| 99 | 0x63 | KpDot | Keypad . and Delete |
| 100 | 0x64 | Key102nd | Non-US \\ and \| |
| 101 | 0x65 | Compose | Application |
| 102 | 0x66 | Power | Power |
| 103 | 0x67 | KpEqual | Keypad = |
| 104 | 0x68 | F13 | F13 |
| 105 | 0x69 | F14 | F14 |
| 106 | 0x6A | F15 | F15 |
| 107 | 0x6B | F16 | F16 |
| 108 | 0x6C | F17 | F17 |
| 109 | 0x6D | F18 | F18 |
| 110 | 0x6E | F19 | F19 |
| 111 | 0x6F | F20 | F20 |
| 112 | 0x70 | F21 | F21 |
| 113 | 0x71 | F22 | F22 |
| 114 | 0x72 | F23 | F23 |
| 115 | 0x73 | F24 | F24 |
| 116 | 0x74 | Open | Execute |
| 117 | 0x75 | Help | Help |
| 118 | 0x76 | Props | Menu |
| 119 | 0x77 | Front | Select |
| 120 | 0x78 | Stop | Stop |
| 121 | 0x79 | Again | Again |
| 122 | 0x7A | Undo | Undo |
| 123 | 0x7B | Cut | Cut |
| 124 | 0x7C | Copy | Copy |
| 125 | 0x7D | Paste | Paste |
| 126 | 0x7E | Find | Find |
| 127 | 0x7F | Mute | Mute |
| 128 | 0x80 | VolumeUp | Volume Up |
| 129 | 0x81 | VolumeDown | Volume Down |
| 130 | 0x82 | LockingCapsLock | Locking Caps Lock |
| 131 | 0x83 | LockingNumLock | Locking Num Lock |
| 132 | 0x84 | LockingScrollLock | Locking Scroll Lock |
| 133 | 0x85 | KpComma | Keypad Comma |
| 134 | 0x86 | KpEqualSign | Keypad Equal Sign |
| 135 | 0x87 | Ro | International1 (Ro) |
| 136 | 0x88 | KatakanaHiragana | International2 (Katakana/Hiragana) |
| 137 | 0x89 | Yen | International3 (Yen) |
| 138 | 0x8A | Henkan | International4 (Henkan) |
| 139 | 0x8B | Muhenkan | International5 (Muhenkan) |
| 140 | 0x8C | KpJpComma | International6 (JP Comma) |
| 141 | 0x8D | International7 | International7 |
| 142 | 0x8E | International8 | International8 |
| 143 | 0x8F | International9 | International9 |
| 144 | 0x90 | Hangeul | Language1 (Hangeul) |
| 145 | 0x91 | Hanja | Language2 (Hanja) |
| 146 | 0x92 | Katakana | Language3 (Katakana) |
| 147 | 0x93 | Hiragana | Language4 (Hiragana) |
| 148 | 0x94 | ZenkakuHankaku | Language5 (Zenkaku/Hankaku) |
| 149 | 0x95 | Lang6 | Language6 |
| 150 | 0x96 | Lang7 | Language7 |
| 151 | 0x97 | Lang8 | Language8 |
| 152 | 0x98 | Lang9 | Language9 |
| 153 | 0x99 | AlternateErase | Alternate Erase |
| 154 | 0x9A | SysReqAttention | SysReq/Attention |
| 155 | 0x9B | Cancel | Cancel |
| 156 | 0x9C | Clear | Clear |
| 157 | 0x9D | Prior | Prior |
| 158 | 0x9E | Return | Return |
| 159 | 0x9F | Separator | Separator |
| 160 | 0xA0 | Out | Out |
| 161 | 0xA1 | Oper | Oper |
| 162 | 0xA2 | ClearAgain | Clear/Again |
| 163 | 0xA3 | CrSelProps | CrSel/Props |
| 164 | 0xA4 | ExSel | ExSel |
| 165-181 | 0xA5-0xB5 | Reserved165-181 | Reserved |
| 182 | 0xB6 | KpLeftParen | Keypad ( |
| 183 | 0xB7 | KpRightParen | Keypad ) |
| 184-223 | 0xB8-0xDF | Reserved184-223 | Reserved |
| 224 | 0xE0 | LeftCtrl | Left Control |
| 225 | 0xE1 | LeftShift | Left Shift |
| 226 | 0xE2 | LeftAlt | Left Alt |
| 227 | 0xE3 | LeftMeta | Left GUI (Windows) |
| 228 | 0xE4 | RightCtrl | Right Control |
| 229 | 0xE5 | RightShift | Right Shift |
| 230 | 0xE6 | RightAlt | Right Alt |
| 231 | 0xE7 | RightMeta | Right GUI (Windows) |
| 232 | 0xE8 | MediaPlayPause | Media Play/Pause |
| 233 | 0xE9 | MediaStopCD | Media Stop CD |
| 234 | 0xEA | MediaPreviousSong | Media Previous Track |
| 235 | 0xEB | MediaNextSong | Media Next Track |
| 236 | 0xEC | MediaEjectCD | Media Eject CD |
| 237 | 0xED | MediaVolumeUp | Media Volume Up |
| 238 | 0xEE | MediaVolumeDown | Media Volume Down |
| 239 | 0xEF | MediaMute | Media Mute |
| 240 | 0xF0 | MediaWWW | Media WWW |
| 241 | 0xF1 | MediaBack | Media Back |
| 242 | 0xF2 | MediaForward | Media Forward |
| 243 | 0xF3 | MediaStop | Media Stop |
| 244 | 0xF4 | MediaFind | Media Find |
| 245 | 0xF5 | MediaScrollUp | Media Scroll Up |
| 246 | 0xF6 | MediaScrollDown | Media Scroll Down |
| 247 | 0xF7 | MediaEdit | Media Edit |
| 248 | 0xF8 | MediaSleep | Media Sleep |
| 249 | 0xF9 | MediaCoffee | Media Coffee |
| 250 | 0xFA | MediaRefresh | Media Refresh |
| 251 | 0xFB | MediaCalc | Media Calculator |
| 252 | 0xFC | Reserved252 | Reserved |
| 253 | 0xFD | Reserved253 | Reserved |
| 254 | 0xFE | Reserved254 | Reserved |
| 255 | 0xFF | Reserved255 | Reserved |