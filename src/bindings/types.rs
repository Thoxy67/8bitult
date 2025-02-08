use crate::keyboard;

pub type KeyBinding = [u8; 4];

pub const KEY_BINDINGS: [KeyBinding; 16] = [
    // First 8 keys (for first command)
    [
        keyboard::KEY_Q,
        keyboard::KEY_NULL,
        keyboard::KEY_NULL,
        keyboard::KEY_NULL,
    ], // A
    [
        keyboard::KEY_Q,
        keyboard::KEY_NULL,
        keyboard::KEY_NULL,
        keyboard::KEY_NULL,
    ], // B
    [
        keyboard::KEY_Q,
        keyboard::KEY_NULL,
        keyboard::KEY_NULL,
        keyboard::KEY_NULL,
    ], // X
    [
        keyboard::KEY_Q,
        keyboard::KEY_NULL,
        keyboard::KEY_NULL,
        keyboard::KEY_NULL,
    ], // Y
    [
        keyboard::KEY_Q,
        keyboard::KEY_NULL,
        keyboard::KEY_NULL,
        keyboard::KEY_NULL,
    ], // L1
    [
        keyboard::KEY_Q,
        keyboard::KEY_NULL,
        keyboard::KEY_NULL,
        keyboard::KEY_NULL,
    ], // R1
    [
        keyboard::KEY_Q,
        keyboard::KEY_NULL,
        keyboard::KEY_NULL,
        keyboard::KEY_NULL,
    ], // L2
    [
        keyboard::KEY_Q,
        keyboard::KEY_NULL,
        keyboard::KEY_NULL,
        keyboard::KEY_NULL,
    ], // R2
    // Next 8 keys (for second command)
    [
        keyboard::KEY_Q,
        keyboard::KEY_NULL,
        keyboard::KEY_NULL,
        keyboard::KEY_NULL,
    ], // SELECT
    [
        keyboard::KEY_Q,
        keyboard::KEY_NULL,
        keyboard::KEY_NULL,
        keyboard::KEY_NULL,
    ], // START
    [
        keyboard::KEY_Q,
        keyboard::KEY_NULL,
        keyboard::KEY_NULL,
        keyboard::KEY_NULL,
    ], // HOME
    [
        keyboard::KEY_Q,
        keyboard::KEY_NULL,
        keyboard::KEY_NULL,
        keyboard::KEY_NULL,
    ], // LOGO
    [
        keyboard::KEY_Q,
        keyboard::KEY_NULL,
        keyboard::KEY_NULL,
        keyboard::KEY_NULL,
    ], // UP
    [
        keyboard::KEY_Q,
        keyboard::KEY_NULL,
        keyboard::KEY_NULL,
        keyboard::KEY_NULL,
    ], // DOWN
    [
        keyboard::KEY_Q,
        keyboard::KEY_NULL,
        keyboard::KEY_NULL,
        keyboard::KEY_NULL,
    ], // LEFT
    [
        keyboard::KEY_Q,
        keyboard::KEY_NULL,
        keyboard::KEY_NULL,
        keyboard::KEY_NULL,
    ], // RIGHT
];
