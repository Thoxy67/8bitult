pub type KeyBinding = [u8; 4];

#[derive(Debug)]
pub struct KeyGroup {
    pub command: Vec<u8>,
    pub start_pos: usize,
    pub end_pos: usize,
}

impl KeyGroup {
    pub fn new(command: Vec<u8>, start_pos: usize, end_pos: usize) -> Self {
        Self {
            command,
            start_pos,
            end_pos,
        }
    }
}
