# USB HID Keyboard Codes

Every key you can bind is a USB HID usage code. In a profile you refer to a key
either by its **name** (the Constant Name column below) or by its raw code with
`keycode(XX)`, where `XX` is the hex value.

- [Keyboard layouts](#keyboard-layouts)
- [How to write keys](#how-to-write-keys)
- [Common keys](#common-keys)
- Full reference: [Letters](#letters) · [Numbers](#numbers) ·
  [Whitespace and control](#whitespace-and-control) ·
  [Symbols](#symbols-and-punctuation) · [Function keys](#function-keys) ·
  [Navigation and editing](#navigation-and-editing) · [Keypad](#keypad) ·
  [Modifiers](#modifiers) · [Consumer and media](#consumer-and-media) ·
  [Application and system](#application-and-system) ·
  [International and language](#international-and-language) ·
  [Reserved](#reserved-and-error-codes)

## Keyboard layouts

The codes follow the **US QWERTY** layout. They map to a physical key position,
not to the character printed on your keys. On a non-US layout (AZERTY, QWERTZ,
and so on) you bind by where the key sits on a US QWERTY board.

For example, to hit the key that types `A` on AZERTY, bind `Q` (`0x14`), because
that key is in the QWERTY `Q` position.

![US QWERTY layout](https://upload.wikimedia.org/wikipedia/commons/d/da/KB_United_States.svg)

## How to write keys

Names are **case sensitive** and must match the Constant Name column exactly
(`Enter`, not `ENTER` or `enter`). Two forms are accepted:

1. By name: `"A"`, `"Enter"`, `"LeftShift"`
2. By raw hex code: `"keycode(04)"` for `A`, `"keycode(28)"` for `Enter`

```toml
[bindings]
L1 = ["keycode(04)", "keycode(05)"]  # A and B by code (QWERTY position)
R1 = ["A", "keycode(06)"]            # A and C, mixing a name and a code
```

`None` (`0x00`) means "no key". An empty list (`HOME = []`) leaves a button
unbound, which is the same thing.

## Common keys

The keys you are most likely to bind. The full tables further down list
everything.

| Keys | Names |
|------|-------|
| Letters | `A` to `Z` |
| Numbers (top row) | `Key1` to `Key9`, `Key0` |
| Whitespace | `Enter`, `Escape`, `Backspace`, `Tab`, `Space` |
| Arrows | `Up`, `Down`, `Left`, `Right` |
| Editing | `Insert`, `Delete`, `Home`, `End`, `PageUp`, `PageDown` |
| Modifiers | `LeftCtrl`, `LeftShift`, `LeftAlt`, `LeftMeta` (and `Right*`) |

## Letters

| Dec | Hex | Name | | Dec | Hex | Name |
|-----|------|------|---|-----|------|------|
| 4 | 0x04 | A | | 17 | 0x11 | N |
| 5 | 0x05 | B | | 18 | 0x12 | O |
| 6 | 0x06 | C | | 19 | 0x13 | P |
| 7 | 0x07 | D | | 20 | 0x14 | Q |
| 8 | 0x08 | E | | 21 | 0x15 | R |
| 9 | 0x09 | F | | 22 | 0x16 | S |
| 10 | 0x0A | G | | 23 | 0x17 | T |
| 11 | 0x0B | H | | 24 | 0x18 | U |
| 12 | 0x0C | I | | 25 | 0x19 | V |
| 13 | 0x0D | J | | 26 | 0x1A | W |
| 14 | 0x0E | K | | 27 | 0x1B | X |
| 15 | 0x0F | L | | 28 | 0x1C | Y |
| 16 | 0x10 | M | | 29 | 0x1D | Z |

## Numbers

Top-row number keys. The keypad digits are in [Keypad](#keypad).

| Dec | Hex | Name | Key |
|-----|------|------|-----|
| 30 | 0x1E | Key1 | 1 |
| 31 | 0x1F | Key2 | 2 |
| 32 | 0x20 | Key3 | 3 |
| 33 | 0x21 | Key4 | 4 |
| 34 | 0x22 | Key5 | 5 |
| 35 | 0x23 | Key6 | 6 |
| 36 | 0x24 | Key7 | 7 |
| 37 | 0x25 | Key8 | 8 |
| 38 | 0x26 | Key9 | 9 |
| 39 | 0x27 | Key0 | 0 |

## Whitespace and control

| Dec | Hex | Name | Key |
|-----|------|------|-----|
| 40 | 0x28 | Enter | Enter / Return |
| 41 | 0x29 | Escape | Escape |
| 42 | 0x2A | Backspace | Backspace |
| 43 | 0x2B | Tab | Tab |
| 44 | 0x2C | Space | Spacebar |

## Symbols and punctuation

| Dec | Hex | Name | Key |
|-----|------|------|-----|
| 45 | 0x2D | Minus | `-` |
| 46 | 0x2E | Equal | `=` |
| 47 | 0x2F | LeftBracket | `[` |
| 48 | 0x30 | RightBracket | `]` |
| 49 | 0x31 | Backslash | `\` |
| 50 | 0x32 | HashTilde | non-US `#` / `~` |
| 51 | 0x33 | Semicolon | `;` |
| 52 | 0x34 | Apostrophe | `'` |
| 53 | 0x35 | Grave | `` ` `` / `~` |
| 54 | 0x36 | Comma | `,` |
| 55 | 0x37 | Dot | `.` |
| 56 | 0x38 | Slash | `/` |
| 100 | 0x64 | Key102nd | non-US `\` / `\|` |

## Function keys

| Dec | Hex | Name | | Dec | Hex | Name |
|-----|------|------|---|-----|------|------|
| 58 | 0x3A | F1 | | 104 | 0x68 | F13 |
| 59 | 0x3B | F2 | | 105 | 0x69 | F14 |
| 60 | 0x3C | F3 | | 106 | 0x6A | F15 |
| 61 | 0x3D | F4 | | 107 | 0x6B | F16 |
| 62 | 0x3E | F5 | | 108 | 0x6C | F17 |
| 63 | 0x3F | F6 | | 109 | 0x6D | F18 |
| 64 | 0x40 | F7 | | 110 | 0x6E | F19 |
| 65 | 0x41 | F8 | | 111 | 0x6F | F20 |
| 66 | 0x42 | F9 | | 112 | 0x70 | F21 |
| 67 | 0x43 | F10 | | 113 | 0x71 | F22 |
| 68 | 0x44 | F11 | | 114 | 0x72 | F23 |
| 69 | 0x45 | F12 | | 115 | 0x73 | F24 |

## Navigation and editing

| Dec | Hex | Name | Key |
|-----|------|------|-----|
| 57 | 0x39 | CapsLock | Caps Lock |
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

## Keypad

| Dec | Hex | Name | Key |
|-----|------|------|-----|
| 83 | 0x53 | NumLock | Num Lock |
| 84 | 0x54 | KpSlash | `/` |
| 85 | 0x55 | KpAsterisk | `*` |
| 86 | 0x56 | KpMinus | `-` |
| 87 | 0x57 | KpPlus | `+` |
| 88 | 0x58 | KpEnter | Enter |
| 89 | 0x59 | Kp1 | 1 / End |
| 90 | 0x5A | Kp2 | 2 / Down |
| 91 | 0x5B | Kp3 | 3 / PageDn |
| 92 | 0x5C | Kp4 | 4 / Left |
| 93 | 0x5D | Kp5 | 5 |
| 94 | 0x5E | Kp6 | 6 / Right |
| 95 | 0x5F | Kp7 | 7 / Home |
| 96 | 0x60 | Kp8 | 8 / Up |
| 97 | 0x61 | Kp9 | 9 / PageUp |
| 98 | 0x62 | Kp0 | 0 / Insert |
| 99 | 0x63 | KpDot | `.` / Delete |
| 103 | 0x67 | KpEqual | `=` |
| 133 | 0x85 | KpComma | `,` |
| 134 | 0x86 | KpEqualSign | `=` (AS/400) |
| 182 | 0xB6 | KpLeftParen | `(` |
| 183 | 0xB7 | KpRightParen | `)` |

## Modifiers

| Dec | Hex | Name | Key |
|-----|------|------|-----|
| 224 | 0xE0 | LeftCtrl | Left Control |
| 225 | 0xE1 | LeftShift | Left Shift |
| 226 | 0xE2 | LeftAlt | Left Alt |
| 227 | 0xE3 | LeftMeta | Left GUI (Super / Win) |
| 228 | 0xE4 | RightCtrl | Right Control |
| 229 | 0xE5 | RightShift | Right Shift |
| 230 | 0xE6 | RightAlt | Right Alt |
| 231 | 0xE7 | RightMeta | Right GUI (Super / Win) |

## Consumer and media

| Dec | Hex | Name | Key |
|-----|------|------|-----|
| 102 | 0x66 | Power | Power |
| 127 | 0x7F | Mute | Mute |
| 128 | 0x80 | VolumeUp | Volume Up |
| 129 | 0x81 | VolumeDown | Volume Down |
| 232 | 0xE8 | MediaPlayPause | Play / Pause |
| 233 | 0xE9 | MediaStopCD | Stop CD |
| 234 | 0xEA | MediaPreviousSong | Previous Track |
| 235 | 0xEB | MediaNextSong | Next Track |
| 236 | 0xEC | MediaEjectCD | Eject CD |
| 237 | 0xED | MediaVolumeUp | Volume Up |
| 238 | 0xEE | MediaVolumeDown | Volume Down |
| 239 | 0xEF | MediaMute | Mute |
| 240 | 0xF0 | MediaWWW | Web |
| 241 | 0xF1 | MediaBack | Back |
| 242 | 0xF2 | MediaForward | Forward |
| 243 | 0xF3 | MediaStop | Stop |
| 244 | 0xF4 | MediaFind | Find |
| 245 | 0xF5 | MediaScrollUp | Scroll Up |
| 246 | 0xF6 | MediaScrollDown | Scroll Down |
| 247 | 0xF7 | MediaEdit | Edit |
| 248 | 0xF8 | MediaSleep | Sleep |
| 249 | 0xF9 | MediaCoffee | Lock / Screensaver |
| 250 | 0xFA | MediaRefresh | Refresh |
| 251 | 0xFB | MediaCalc | Calculator |

## Application and system

Less common keys for application control and editing. Many are legacy and not
honored by every OS.

| Dec | Hex | Name | Key |
|-----|------|------|-----|
| 101 | 0x65 | Compose | Application / Menu |
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
| 130 | 0x82 | LockingCapsLock | Locking Caps Lock |
| 131 | 0x83 | LockingNumLock | Locking Num Lock |
| 132 | 0x84 | LockingScrollLock | Locking Scroll Lock |
| 153 | 0x99 | AlternateErase | Alternate Erase |
| 154 | 0x9A | SysReqAttention | SysReq / Attention |
| 155 | 0x9B | Cancel | Cancel |
| 156 | 0x9C | Clear | Clear |
| 157 | 0x9D | Prior | Prior |
| 158 | 0x9E | Return | Return |
| 159 | 0x9F | Separator | Separator |
| 160 | 0xA0 | Out | Out |
| 161 | 0xA1 | Oper | Oper |
| 162 | 0xA2 | ClearAgain | Clear / Again |
| 163 | 0xA3 | CrSelProps | CrSel / Props |
| 164 | 0xA4 | ExSel | ExSel |

## International and language

Used for language-specific input, mostly on JP and KR keyboards.

| Dec | Hex | Name | Key |
|-----|------|------|-----|
| 135 | 0x87 | Ro | International1 (Ro) |
| 136 | 0x88 | KatakanaHiragana | International2 |
| 137 | 0x89 | Yen | International3 (Yen) |
| 138 | 0x8A | Henkan | International4 |
| 139 | 0x8B | Muhenkan | International5 |
| 140 | 0x8C | KpJpComma | International6 (JP comma) |
| 141 | 0x8D | International7 | International7 |
| 142 | 0x8E | International8 | International8 |
| 143 | 0x8F | International9 | International9 |
| 144 | 0x90 | Hangeul | Language1 (Hangeul) |
| 145 | 0x91 | Hanja | Language2 (Hanja) |
| 146 | 0x92 | Katakana | Language3 |
| 147 | 0x93 | Hiragana | Language4 |
| 148 | 0x94 | ZenkakuHankaku | Language5 |
| 149 | 0x95 | Lang6 | Language6 |
| 150 | 0x96 | Lang7 | Language7 |
| 151 | 0x97 | Lang8 | Language8 |
| 152 | 0x98 | Lang9 | Language9 |

## Reserved and error codes

These exist as names for completeness but are not useful as bindings.

| Dec | Hex | Name | Meaning |
|-----|------|------|---------|
| 0 | 0x00 | None | No key (unbound) |
| 1 | 0x01 | ErrorRollOver | Roll-over error |
| 2 | 0x02 | PostFail | POST failed |
| 3 | 0x03 | ErrorUndefined | Undefined error |
| 165-181 | 0xA5-0xB5 | Reserved165-181 | Reserved |
| 184-223 | 0xB8-0xDF | Reserved184-223 | Reserved |
| 252-255 | 0xFC-0xFF | Reserved252-255 | Reserved |

Each reserved value still has a name of the form `Reserved<N>` (for example
`Reserved200`) and can be sent with `keycode(XX)` if you need to.
