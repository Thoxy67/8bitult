# USB HID Keyboard Codes Documentation

## Important Note About Keyboard Layouts
This documentation is based on the US QWERTY keyboard layout. When using these keycodes with different keyboard layouts (like AZERTY, QWERTZ, etc.), you must refer to the physical position of the key on a US QWERTY keyboard, not the character it produces.

For example, if you want to bind the 'A' key on an AZERTY keyboard, you should use the keycode for 'Q' (0x14) because that's where the 'A' is physically located on a QWERTY layout.

![KB_United_States](https://upload.wikimedia.org/wikipedia/commons/d/da/KB_United_States.svg)

## Using Custom Keycodes
Keycodes can be specified in two ways in your configuration:
1. Using predefined key names: `"A"`, `"ENTER"`, `"LEFT_SHIFT"`, etc.
2. Using raw keycodes: `keycode(04)` for 'A', `keycode(28)` for 'ENTER', etc.

Example:
```toml
[bindings]
L1 = ["keycode(04)", "keycode(05)"]  # Maps to 'A' and 'B' keys (physical position on QWERTY)
R1 = ["A", "keycode(06)"]            # Maps to 'A' and 'C' keys (physical position on QWERTY)
```

## Complete Keycode Reference Table
Below is the complete reference table of all available keycodes. Remember that these codes correspond to physical key positions on a US QWERTY keyboard.

| Dec | Hex  | Constant Name | Description |
|-----|------|--------------|-------------|
| 0 | 0x00 | KEY_NULL | No key pressed |
| 1 | 0x01 | KEY_ERROR_ROLLOVER | Rollover error |
| 2 | 0x02 | KEY_POST_FAIL | POST failure |
| 3 | 0x03 | KEY_ERROR_UNDEFINED | Undefined error |
| 4 | 0x04 | KEY_A | Letter 'A' |
| 5 | 0x05 | KEY_B | Letter 'B' |
| 6 | 0x06 | KEY_C | Letter 'C' |
| 7 | 0x07 | KEY_D | Letter 'D' |
| 8 | 0x08 | KEY_E | Letter 'E' |
| 9 | 0x09 | KEY_F | Letter 'F' |
| 10 | 0x0A | KEY_G | Letter 'G' |
| 11 | 0x0B | KEY_H | Letter 'H' |
| 12 | 0x0C | KEY_I | Letter 'I' |
| 13 | 0x0D | KEY_J | Letter 'J' |
| 14 | 0x0E | KEY_K | Letter 'K' |
| 15 | 0x0F | KEY_L | Letter 'L' |
| 16 | 0x10 | KEY_M | Letter 'M' |
| 17 | 0x11 | KEY_N | Letter 'N' |
| 18 | 0x12 | KEY_O | Letter 'O' |
| 19 | 0x13 | KEY_P | Letter 'P' |
| 20 | 0x14 | KEY_Q | Letter 'Q' |
| 21 | 0x15 | KEY_R | Letter 'R' |
| 22 | 0x16 | KEY_S | Letter 'S' |
| 23 | 0x17 | KEY_T | Letter 'T' |
| 24 | 0x18 | KEY_U | Letter 'U' |
| 25 | 0x19 | KEY_V | Letter 'V' |
| 26 | 0x1A | KEY_W | Letter 'W' |
| 27 | 0x1B | KEY_X | Letter 'X' |
| 28 | 0x1C | KEY_Y | Letter 'Y' |
| 29 | 0x1D | KEY_Z | Letter 'Z' |
| 30 | 0x1E | KEY_1 | Number '1' |
| 31 | 0x1F | KEY_2 | Number '2' |
| 32 | 0x20 | KEY_3 | Number '3' |
| 33 | 0x21 | KEY_4 | Number '4' |
| 34 | 0x22 | KEY_5 | Number '5' |
| 35 | 0x23 | KEY_6 | Number '6' |
| 36 | 0x24 | KEY_7 | Number '7' |
| 37 | 0x25 | KEY_8 | Number '8' |
| 38 | 0x26 | KEY_9 | Number '9' |
| 39 | 0x27 | KEY_0 | Number '0' |
| 40 | 0x28 | KEY_ENTER | Enter/Return key |
| 41 | 0x29 | KEY_ESC | Escape key |
| 42 | 0x2A | KEY_BACK | Backspace |
| 43 | 0x2B | KEY_TAB | Tab key |
| 44 | 0x2C | KEY_SPACE | Spacebar |
| 45 | 0x2D | KEY_SUB | Minus (-) |
| 46 | 0x2E | KEY_EQUAL | Equal (=) |
| 47 | 0x2F | KEY_LEFTBRACKET | Left bracket ([) |
| 48 | 0x30 | KEY_RIGHTBRACKET | Right bracket (]) |
| 49 | 0x31 | KEY_VERTICALLINE | Vertical line (\\) |
| 51 | 0x33 | KEY_SEMICOLON | Semicolon (;) |
| 52 | 0x34 | KEY_QUOTE | Quote (') |
| 53 | 0x35 | KEY_WAVE | Wave/Tilde (~) |
| 54 | 0x36 | KEY_COMMA | Comma (,) |
| 55 | 0x37 | KEY_DOT | Period (.) |
| 56 | 0x38 | KEY_QUESTION | Question mark (/) |
| 57 | 0x39 | KEY_CAPS | Caps Lock |
| 58 | 0x3A | KEY_F1 | F1 |
| 59 | 0x3B | KEY_F2 | F2 |
| 60 | 0x3C | KEY_F3 | F3 |
| 61 | 0x3D | KEY_F4 | F4 |
| 62 | 0x3E | KEY_F5 | F5 |
| 63 | 0x3F | KEY_F6 | F6 |
| 64 | 0x40 | KEY_F7 | F7 |
| 65 | 0x41 | KEY_F8 | F8 |
| 66 | 0x42 | KEY_F9 | F9 |
| 67 | 0x43 | KEY_F10 | F10 |
| 68 | 0x44 | KEY_F11 | F11 |
| 69 | 0x45 | KEY_F12 | F12 |
| 70 | 0x46 | KEY_PRTSCR | Print Screen |
| 71 | 0x47 | KEY_SCOLLLOCK | Scroll Lock |
| 72 | 0x48 | KEY_PAUSE | Pause |
| 73 | 0x49 | KEY_INS | Insert |
| 74 | 0x4A | KEY_HOME | Home |
| 75 | 0x4B | KEY_PAGEUP | Page Up |
| 76 | 0x4C | KEY_DEL | Delete |
| 77 | 0x4D | KEY_END | End |
| 78 | 0x4E | KEY_PAGEDOWN | Page Down |
| 79 | 0x4F | KEY_RIGHT | Right Arrow |
| 80 | 0x50 | KEY_LEFT | Left Arrow |
| 81 | 0x51 | KEY_DOWN | Down Arrow |
| 82 | 0x52 | KEY_UP | Up Arrow |
| 83 | 0x53 | KEYBOARDPAD_NUMLOCK | Num Lock |
| 84 | 0x54 | KEYBOARDPAD_DIV | Keypad / |
| 85 | 0x55 | KEYBOARDPAD_MUL | Keypad * |
| 86 | 0x56 | KEYBOARDPAD_SUB | Keypad - |
| 87 | 0x57 | KEYBOARDPAD_ADD | Keypad + |
| 88 | 0x58 | KEYBOARDPAD_ENTER | Keypad Enter |
| 89 | 0x59 | KEYBOARDPAD_1 | Keypad 1 |
| 90 | 0x5A | KEYBOARDPAD_2 | Keypad 2 |
| 91 | 0x5B | KEYBOARDPAD_3 | Keypad 3 |
| 92 | 0x5C | KEYBOARDPAD_4 | Keypad 4 |
| 93 | 0x5D | KEYBOARDPAD_5 | Keypad 5 |
| 94 | 0x5E | KEYBOARDPAD_6 | Keypad 6 |
| 95 | 0x5F | KEYBOARDPAD_7 | Keypad 7 |
| 96 | 0x60 | KEYBOARDPAD_8 | Keypad 8 |
| 97 | 0x61 | KEYBOARDPAD_9 | Keypad 9 |
| 98 | 0x62 | KEYBOARDPAD_0 | Keypad 0 |
| 99 | 0x63 | KEYBOARDPAD_DOT | Keypad . |
| 100 | 0x64 | KEY_F13 | F13 |
| 101 | 0x65 | KEY_F14 | F14 |
| 102 | 0x66 | KEY_F15 | F15 |
| 103 | 0x67 | KEY_F16 | F16 |
| 104 | 0x68 | KEY_F17 | F17 |
| 105 | 0x69 | KEY_F18 | F18 |
| 106 | 0x6A | KEY_F19 | F19 |
| 107 | 0x6B | KEY_F20 | F20 |
| 108 | 0x6C | KEY_F21 | F21 |
| 109 | 0x6D | KEY_F22 | F22 |
| 110 | 0x6E | KEY_F23 | F23 |
| 111 | 0x6F | KEY_F24 | F24 |
| 118 | 0x76 | KEY_MENU | Menu |
| 119 | 0x77 | KEY_SELECT | Select |
| 120 | 0x78 | KEY_STOP | Stop |
| 121 | 0x79 | KEY_AGAIN | Again |
| 122 | 0x7A | KEY_UNDO | Undo |
| 123 | 0x7B | KEY_CUT | Cut |
| 124 | 0x7C | KEY_COPY | Copy |
| 125 | 0x7D | KEY_PASTE | Paste |
| 126 | 0x7E | KEY_FIND | Find |
| 224 | 0xE0 | KEY_LCTRL | Left Control |
| 225 | 0xE1 | KEY_LSHIFT | Left Shift |
| 226 | 0xE2 | KEY_LALT | Left Alt |
| 227 | 0xE3 | KEY_LWIN | Left Windows |
| 228 | 0xE4 | KEY_RCTRL | Right Control |
| 229 | 0xE5 | KEY_RSHIFT | Right Shift |
| 230 | 0xE6 | KEY_RALT | Right Alt |
| 231 | 0xE7 | KEY_MUTE | Mute |
| 232 | 0xE8 | KEY_VOLUMEUP | Volume Up |
| 233 | 0xE9 | KEY_VOLUMEDOWN | Volume Down |
| 234 | 0xEA | KEY_PLAYPAUSE | Play/Pause |
| 235 | 0xEB | KEY_MEDIASTOP | Media Stop |
| 236 | 0xEC | KEY_PREVIOUSSONG | Previous Track |
| 237 | 0xED | KEY_NEXTSONG | Next Track |
| 238 | 0xEE | KEY_EMAIL | Email |
| 239 | 0xEF | KEY_CALCULATOR | Calculator |
| 240 | 0xF0 | KEY_MYCOMPUTER | My Computer |
| 241 | 0xF1 | KEY_WWWSEARCH | WWW Search |
| 242 | 0xF2 | KEY_WWWHOME | WWW Home |
| 243 | 0xF3 | KEY_WWWBACK | WWW Back |
| 244 | 0xF4 | KEY_WWWFORWARD | WWW Forward |
| 245 | 0xF5 | KEY_WWWSTOP | WWW Stop |
| 246 | 0xF6 | KEY_WWWREFRESH | WWW Refresh |
| 247 | 0xF7 | KEY_WWWFAVORITES | WWW Favorites |
| 248 | 0xF8 | KEY_POWER | Power |
| 249 | 0xF9 | KEY_SLEEP | Sleep |
| 250 | 0xFA | KEY_WAKE | Wake |
| 251 | 0xFB | KEY_INTERNATIONAL1 | International 1 |
| 252 | 0xFC | KEY_INTERNATIONAL2 | International 2 |
| 253 | 0xFD | KEY_INTERNATIONAL3 | International 3 |
| 254 | 0xFE | KEY_INTERNATIONAL4 | International 4 |
| 255 | 0xFF | KEY_INTERNATIONAL5 | International 5 |

Notes:
- Codes not listed are reserved or unassigned
- All numeric values are represented in both decimal and hexadecimal for convenience
- International keys are typically used for language-specific functions