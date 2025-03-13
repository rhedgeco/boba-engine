use std::sync::atomic::{AtomicU64, Ordering};

use derive_more::Display;

#[repr(transparent)]
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UniqueId(u64);

impl UniqueId {
    pub fn new() -> Self {
        Self({
            static GENERATOR: AtomicU64 = AtomicU64::new(0);
            GENERATOR.fetch_add(1, Ordering::Relaxed)
        })
    }

    pub fn value(self) -> u64 {
        self.0
    }
}
