pub(crate) struct KeyEvent {
    pub(crate) keycode: u32,
}

impl KeyEvent {
    pub(crate) fn new(keycode: u32) -> Self {
        Self { keycode }
    }
}
