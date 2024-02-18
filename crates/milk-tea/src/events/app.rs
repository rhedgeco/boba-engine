pub struct Init {
    _private: (),
}

impl Init {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }
}

pub struct Suspend {
    _private: (),
}

impl Suspend {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }
}

pub struct Resume {
    _private: (),
}

impl Resume {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }
}

pub struct Update {
    _private: (),
}

impl Update {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }
}
