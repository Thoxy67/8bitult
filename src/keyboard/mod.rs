#![allow(unused_imports)]
#![allow(dead_code)]

pub const KEY_0: u8 = 39;
pub const KEY_1: u8 = 30;
pub const KEY_2: u8 = 31;
pub const KEY_3: u8 = 32;
pub const KEY_4: u8 = 33;
pub const KEY_5: u8 = 34;
pub const KEY_6: u8 = 35;
pub const KEY_7: u8 = 36;
pub const KEY_8: u8 = 37;
pub const KEY_9: u8 = 38;
pub const KEY_A: u8 = 4;
pub const KEY_B: u8 = 5;
pub const KEY_BACK: u8 = 42;
pub const KEY_C: u8 = 6;
pub const KEY_CAPS: u8 = 57;
pub const KEY_COMMA: u8 = 54;
pub const KEY_D: u8 = 7;
pub const KEY_DEL: u8 = 76;
pub const KEY_DOT: u8 = 55;
pub const KEY_DOWN: u8 = 81;
pub const KEY_E: u8 = 8;
pub const KEY_END: u8 = 77;
pub const KEY_ENTER: u8 = 40;
pub const KEY_EQUAL: u8 = 46;
pub const KEY_ESC: u8 = 41;
pub const KEY_F: u8 = 9;
pub const KEY_F1: u8 = 58;
pub const KEY_F10: u8 = 67;
pub const KEY_F11: u8 = 68;
pub const KEY_F12: u8 = 69;
pub const KEY_F2: u8 = 59;
pub const KEY_F3: u8 = 60;
pub const KEY_F4: u8 = 61;
pub const KEY_F5: u8 = 62;
pub const KEY_F6: u8 = 63;
pub const KEY_F7: u8 = 64;
pub const KEY_F8: u8 = 65;
pub const KEY_F9: u8 = 66;
pub const KEY_G: u8 = 10;
pub const KEY_H: u8 = 11;
pub const KEY_HOME: u8 = 74;
pub const KEY_I: u8 = 12;
pub const KEY_INS: u8 = 73;
pub const KEY_J: u8 = 13;
pub const KEY_K: u8 = 14;
pub const KEY_L: u8 = 15;
pub const KEY_LALT: u8 = 226;
pub const KEY_LCTRL: u8 = 224;
pub const KEY_LEFT: u8 = 80;
pub const KEY_LEFTBRACKET: u8 = 47;
pub const KEY_LSHIFT: u8 = 225;
pub const KEY_LWIN: u8 = 227;
pub const KEY_M: u8 = 16;
pub const KEY_N: u8 = 17;
pub const KEY_NULL: u8 = 0;
pub const KEY_O: u8 = 18;
pub const KEY_P: u8 = 19;
pub const KEY_PAGEDOWN: u8 = 78;
pub const KEY_PAGEUP: u8 = 75;
pub const KEY_PAUSE: u8 = 72;
pub const KEY_PRTSCR: u8 = 70;
pub const KEY_Q: u8 = 20;
pub const KEY_QUESTION: u8 = 56;
pub const KEY_QUOTE: u8 = 52;
pub const KEY_R: u8 = 21;
pub const KEY_RALT: u8 = 230;
pub const KEY_RCTRL: u8 = 228;
pub const KEY_RIGHT: u8 = 79;
pub const KEY_RIGHTBRACKET: u8 = 48;
pub const KEY_RSHIFT: u8 = 229;
pub const KEY_S: u8 = 22;
pub const KEY_SCOLLLOCK: u8 = 71;
pub const KEY_SEMICOLON: u8 = 51;
pub const KEY_SPACE: u8 = 44;
pub const KEY_SUB: u8 = 45;
pub const KEY_T: u8 = 23;
pub const KEY_TAB: u8 = 43;
pub const KEY_U: u8 = 24;
pub const KEY_UP: u8 = 82;
pub const KEY_V: u8 = 25;
pub const KEY_VERTICALLINE: u8 = 49;
pub const KEY_W: u8 = 26;
pub const KEY_WAVE: u8 = 53;
pub const KEY_X: u8 = 27;
pub const KEY_Y: u8 = 28;
pub const KEY_Z: u8 = 29;
pub const KEYBOARDPAD_0: u8 = 98;
pub const KEYBOARDPAD_1: u8 = 89;
pub const KEYBOARDPAD_2: u8 = 90;
pub const KEYBOARDPAD_3: u8 = 91;
pub const KEYBOARDPAD_4: u8 = 92;
pub const KEYBOARDPAD_5: u8 = 93;
pub const KEYBOARDPAD_6: u8 = 94;
pub const KEYBOARDPAD_7: u8 = 95;
pub const KEYBOARDPAD_8: u8 = 96;
pub const KEYBOARDPAD_9: u8 = 97;
pub const KEYBOARDPAD_ADD: u8 = 87;
pub const KEYBOARDPAD_DIV: u8 = 84;
pub const KEYBOARDPAD_DOT: u8 = 99;
pub const KEYBOARDPAD_ENTER: u8 = 88;
pub const KEYBOARDPAD_MUL: u8 = 85;
pub const KEYBOARDPAD_NUMLOCK: u8 = 83;
pub const KEYBOARDPAD_SUB: u8 = 86;

pub fn get_key_name(key_value: u8) -> String {
    match key_value {
        KEY_NULL => "NULL".to_string(),
        // Touches alphabétiques
        KEY_A => "A".to_string(),
        KEY_B => "B".to_string(),
        KEY_C => "C".to_string(),
        KEY_D => "D".to_string(),
        KEY_E => "E".to_string(),
        KEY_F => "F".to_string(),
        KEY_G => "G".to_string(),
        KEY_H => "H".to_string(),
        KEY_I => "I".to_string(),
        KEY_J => "J".to_string(),
        KEY_K => "K".to_string(),
        KEY_L => "L".to_string(),
        KEY_M => "M".to_string(),
        KEY_N => "N".to_string(),
        KEY_O => "O".to_string(),
        KEY_P => "P".to_string(),
        KEY_Q => "Q".to_string(),
        KEY_R => "R".to_string(),
        KEY_S => "S".to_string(),
        KEY_T => "T".to_string(),
        KEY_U => "U".to_string(),
        KEY_V => "V".to_string(),
        KEY_W => "W".to_string(),
        KEY_X => "X".to_string(),
        KEY_Y => "Y".to_string(),
        KEY_Z => "Z".to_string(),

        // Touches numériques
        KEY_0 => "0".to_string(),
        KEY_1 => "1".to_string(),
        KEY_2 => "2".to_string(),
        KEY_3 => "3".to_string(),
        KEY_4 => "4".to_string(),
        KEY_5 => "5".to_string(),
        KEY_6 => "6".to_string(),
        KEY_7 => "7".to_string(),
        KEY_8 => "8".to_string(),
        KEY_9 => "9".to_string(),

        // Touches de fonction
        KEY_F1 => "F1".to_string(),
        KEY_F2 => "F2".to_string(),
        KEY_F3 => "F3".to_string(),
        KEY_F4 => "F4".to_string(),
        KEY_F5 => "F5".to_string(),
        KEY_F6 => "F6".to_string(),
        KEY_F7 => "F7".to_string(),
        KEY_F8 => "F8".to_string(),
        KEY_F9 => "F9".to_string(),
        KEY_F10 => "F10".to_string(),
        KEY_F11 => "F11".to_string(),
        KEY_F12 => "F12".to_string(),

        // Touches de navigation
        KEY_UP => "UP".to_string(),
        KEY_DOWN => "DOWN".to_string(),
        KEY_LEFT => "LEFT".to_string(),
        KEY_RIGHT => "RIGHT".to_string(),
        KEY_HOME => "HOME".to_string(),
        KEY_END => "END".to_string(),
        KEY_PAGEUP => "PAGE_UP".to_string(),
        KEY_PAGEDOWN => "PAGE_DOWN".to_string(),
        KEY_INS => "INSERT".to_string(),
        KEY_DEL => "DELETE".to_string(),

        // Touches de contrôle
        KEY_ESC => "ESC".to_string(),
        KEY_TAB => "TAB".to_string(),
        KEY_CAPS => "CAPS_LOCK".to_string(),
        KEY_ENTER => "ENTER".to_string(),
        KEY_SPACE => "SPACE".to_string(),
        KEY_BACK => "BACKSPACE".to_string(),

        // Touches modificatrices
        KEY_LSHIFT => "LEFT_SHIFT".to_string(),
        KEY_RSHIFT => "RIGHT_SHIFT".to_string(),
        KEY_LCTRL => "LEFT_CTRL".to_string(),
        KEY_RCTRL => "RIGHT_CTRL".to_string(),
        KEY_LALT => "LEFT_ALT".to_string(),
        KEY_RALT => "RIGHT_ALT".to_string(),
        KEY_LWIN => "LEFT_WIN".to_string(),

        // Touches spéciales
        KEY_PRTSCR => "PRINT_SCREEN".to_string(),
        KEY_SCOLLLOCK => "SCROLL_LOCK".to_string(),
        KEY_PAUSE => "PAUSE".to_string(),

        // Touches de ponctuation
        KEY_COMMA => "COMMA".to_string(),
        KEY_DOT => "DOT".to_string(),
        KEY_SEMICOLON => "SEMICOLON".to_string(),
        KEY_QUOTE => "QUOTE".to_string(),
        KEY_LEFTBRACKET => "LEFT_BRACKET".to_string(),
        KEY_RIGHTBRACKET => "RIGHT_BRACKET".to_string(),
        KEY_VERTICALLINE => "VERTICAL_LINE".to_string(),
        KEY_WAVE => "TILDE".to_string(),
        KEY_EQUAL => "EQUAL".to_string(),
        KEY_SUB => "MINUS".to_string(),
        KEY_QUESTION => "QUESTION".to_string(),

        // Pavé numérique
        KEYBOARDPAD_0 => "NUMPAD_0".to_string(),
        KEYBOARDPAD_1 => "NUMPAD_1".to_string(),
        KEYBOARDPAD_2 => "NUMPAD_2".to_string(),
        KEYBOARDPAD_3 => "NUMPAD_3".to_string(),
        KEYBOARDPAD_4 => "NUMPAD_4".to_string(),
        KEYBOARDPAD_5 => "NUMPAD_5".to_string(),
        KEYBOARDPAD_6 => "NUMPAD_6".to_string(),
        KEYBOARDPAD_7 => "NUMPAD_7".to_string(),
        KEYBOARDPAD_8 => "NUMPAD_8".to_string(),
        KEYBOARDPAD_9 => "NUMPAD_9".to_string(),
        KEYBOARDPAD_ADD => "NUMPAD_PLUS".to_string(),
        KEYBOARDPAD_SUB => "NUMPAD_MINUS".to_string(),
        KEYBOARDPAD_MUL => "NUMPAD_MULTIPLY".to_string(),
        KEYBOARDPAD_DIV => "NUMPAD_DIVIDE".to_string(),
        KEYBOARDPAD_DOT => "NUMPAD_DOT".to_string(),
        KEYBOARDPAD_ENTER => "NUMPAD_ENTER".to_string(),
        KEYBOARDPAD_NUMLOCK => "NUM_LOCK".to_string(),

        // Valeur inconnue
        _ => format!("Unknown(0x{:02x})", key_value),
    }
}
