use std::{
    fmt::{Debug, Display},
    hash::Hash,
    marker::PhantomData,
};

pub struct Handle<T> {
    raw: u64,
    _type: PhantomData<*const T>,
}

impl<T> Hash for Handle<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.raw.hash(state);
    }
}

impl<T> Copy for Handle<T> {}
impl<T> Clone for Handle<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            raw: self.raw.clone(),
            _type: PhantomData,
        }
    }
}

impl<T> Eq for Handle<T> {}
impl<T> PartialEq for Handle<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl<T> Debug for Handle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Handle").field("raw", &self.raw).finish()
    }
}

impl<T> Display for Handle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = self.id();
        let name = core::any::type_name::<T>();
        write!(f, "Handle<{name}> {{ id: {id} }}")
    }
}

impl<T> Handle<T> {
    const GEN_OFFSET: u32 = u32::BITS;
    const META_OFFSET: u32 = u32::BITS + u16::BITS;

    /// Returns a new handle with the raw `id`
    #[inline]
    pub fn from_raw(raw: u64) -> Self {
        Self {
            raw,
            _type: PhantomData,
        }
    }

    /// Returns a new handle containing the raw parts `index`, `gen`, and `meta`
    #[inline]
    pub fn from_raw_parts(index: u32, gen: u16, meta: u16) -> Self {
        let id = (index as u64)
            + ((gen as u64) << Self::GEN_OFFSET)
            + ((meta as u64) << Self::META_OFFSET);

        Self::from_raw(id)
    }

    /// Decomposes this handle into its raw parts:
    /// - `u32`: index
    /// - `u16`: generation
    /// - `u16`: metadata
    #[inline]
    pub fn into_raw_parts(self) -> (u32, u16, u16) {
        (
            self.raw as u32,
            (self.raw >> Self::GEN_OFFSET) as u16,
            (self.raw >> Self::META_OFFSET) as u16,
        )
    }

    /// Consumes self and transforms it into a handle for another type
    ///
    /// # Warning
    /// While this is not unsafe and will not cause undefined behavior on its own.
    /// However it may not behave as expected. A handle should *usually* be used on the map it is associated with.
    #[inline]
    pub fn into_type<U>(self) -> Handle<U> {
        Handle::from_raw(self.raw)
    }

    // Returns the underlying `u64` used as an id for this handle
    #[inline]
    pub fn id(&self) -> u64 {
        self.raw
    }

    /// Returns the raw index value for this handle
    #[inline]
    pub fn index(&self) -> u32 {
        self.raw as u32
    }

    /// Returns the index value for this handle as a `usize`
    #[inline]
    pub fn uindex(&self) -> usize {
        self.raw as u32 as usize
    }

    /// Returns the raw generation value for this handle
    #[inline]
    pub fn generation(&self) -> u16 {
        (self.raw >> Self::GEN_OFFSET) as u16
    }

    /// Returns the raw metadata value for this handle
    #[inline]
    pub fn metadata(&self) -> u16 {
        (self.raw >> Self::META_OFFSET) as u16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_shifts() {
        let handle = Handle::<()>::from_raw_parts(1, 2, 3);
        assert!(handle.index() == 1);
        assert!(handle.uindex() == 1);
        assert!(handle.generation() == 2);
        assert!(handle.metadata() == 3);
    }
}
