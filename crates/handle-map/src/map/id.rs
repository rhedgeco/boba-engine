use std::sync::atomic::{AtomicU16, Ordering};

/// A simple ZST wrapper for generating unique `u16` values for use in handle maps.
///
/// This is used to generate all the map ids internally and can be used to ensure that custom implemented maps do not use the same id.
pub struct HandleMapId {
    // use private `()` to prevent struct from being created
    _private: (),
}

impl HandleMapId {
    /// Returns a new unique u16 value
    pub fn generate() -> u16 {
        static ID_GEN: AtomicU16 = AtomicU16::new(0);
        ID_GEN.fetch_add(1, Ordering::Relaxed)
    }
}
