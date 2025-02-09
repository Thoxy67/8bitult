use commands::{READ_BINDINGS_COMMANDS, SET_BINDINGS_COMMANDS};
use types::KeyGroup;

pub mod commands;
pub mod read;
pub mod reset;
pub mod types;
pub mod utils;
pub mod write;

pub struct KeyboardCommand {
    pub first_group: KeyGroup,
    pub second_group: KeyGroup,
    pub final_commands: Vec<Vec<u8>>,
}

impl KeyboardCommand {
    pub fn new_read_command() -> Self {
        Self {
            first_group: KeyGroup::new(READ_BINDINGS_COMMANDS[0].to_vec(), 28, 60),
            second_group: KeyGroup::new(READ_BINDINGS_COMMANDS[1].to_vec(), 23, 55),
            final_commands: vec![],
        }
    }

    pub fn new_write_command() -> Self {
        Self {
            first_group: KeyGroup::new(SET_BINDINGS_COMMANDS[0].to_vec(), 29, 61),
            second_group: KeyGroup::new(SET_BINDINGS_COMMANDS[1].to_vec(), 24, 56),
            final_commands: SET_BINDINGS_COMMANDS[2..]
                .iter()
                .map(|&cmd| cmd.to_vec())
                .collect(),
        }
    }
}
