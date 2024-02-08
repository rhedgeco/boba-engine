use boba_core::pearl::SimpleEvent;

pub struct Init {
    _private: (),
}

impl SimpleEvent for Init {}

impl Init {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }
}

pub struct Suspend {
    _private: (),
}

impl SimpleEvent for Suspend {}

impl Suspend {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }
}

pub struct Resume {
    _private: (),
}

impl SimpleEvent for Resume {}

impl Resume {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }
}
