use super::base::SimpleMilkTeaEvent;

pub struct Init {
    _private: (),
}

impl SimpleMilkTeaEvent for Init {}

impl Init {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }
}

pub struct Suspend {
    _private: (),
}

impl SimpleMilkTeaEvent for Suspend {}

impl Suspend {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }
}

pub struct Resume {
    _private: (),
}

impl SimpleMilkTeaEvent for Resume {}

impl Resume {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }
}
