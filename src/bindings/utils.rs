use super::types::KeyBinding;

pub fn create_binding_command(
    base_cmd: &[u8],
    keys: &[KeyBinding],
    start_pos: usize,
    end_pos: usize,
) -> Vec<u8> {
    let mut cmd = base_cmd.to_vec();
    for (i, key_group) in keys.iter().enumerate() {
        let pos = start_pos + (i * 4);
        if pos + 3 < cmd.len() && pos < end_pos {
            cmd[pos..pos + 4].copy_from_slice(key_group);
        }
    }
    if end_pos < cmd.len() {
        cmd[end_pos..].fill(0);
    }
    cmd
}

pub fn parse_binding_notification(
    notification: &[u8],
    start_pos: usize,
    end_pos: usize,
) -> Vec<KeyBinding> {
    let mut bindings = Vec::new();

    let mut pos = start_pos;
    while pos + 3 < notification.len() && pos < end_pos {
        let binding = [
            notification[pos],
            notification[pos + 1],
            notification[pos + 2],
            notification[pos + 3],
        ];
        bindings.push(binding);
        pos += 4;
    }

    bindings
}
