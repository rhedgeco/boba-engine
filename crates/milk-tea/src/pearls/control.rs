use boba_core::Pearl;

pub struct ControlFlow {
    exit: bool,
}

impl Pearl for ControlFlow {}

impl ControlFlow {
    pub(crate) fn new() -> Self {
        Self { exit: false }
    }

    pub fn will_exit(&self) -> bool {
        self.exit
    }

    pub fn set_exit(&mut self, exit: bool) {
        self.exit = exit
    }
}
