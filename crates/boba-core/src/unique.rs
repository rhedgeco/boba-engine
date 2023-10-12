use indexmap::IndexMap;

use crate::{
    pearl::{PearlExt, PearlId},
    Pearl,
};
use std::{
    any::Any,
    cell::{Ref, RefCell, RefMut},
    ops::{Deref, DerefMut},
};

pub struct UniqueEntry<P: Pearl> {
    pearl: RefCell<P>,
}

impl<P: Pearl> UniqueEntry<P> {
    pub(crate) fn new(pearl: P) -> Self {
        Self {
            pearl: RefCell::new(pearl),
        }
    }
}

impl<P: Pearl> UniqueEntry<P> {
    pub fn borrow(&self) -> Option<UniqueRef<P>> {
        let pearl = self.pearl.try_borrow().ok()?;
        Some(UniqueRef { pearl })
    }

    pub fn borrow_mut(&self) -> Option<UniqueMut<P>> {
        let pearl = self.pearl.try_borrow_mut().ok()?;
        Some(UniqueMut { pearl })
    }
}

pub struct UniqueRef<'a, P: Pearl> {
    pearl: Ref<'a, P>,
}

impl<'a, P: Pearl> Deref for UniqueRef<'a, P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        self.pearl.deref()
    }
}

pub struct UniqueMut<'a, P: Pearl> {
    pearl: RefMut<'a, P>,
}

impl<'a, P: Pearl> DerefMut for UniqueMut<'a, P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.pearl.deref_mut()
    }
}

impl<'a, P: Pearl> Deref for UniqueMut<'a, P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        self.pearl.deref()
    }
}

#[derive(Default)]
pub struct UniquePearlMap {
    pearls: IndexMap<PearlId, Box<dyn Any>>,
}

impl UniquePearlMap {
    pub fn len(&self) -> usize {
        self.pearls.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pearls.is_empty()
    }

    pub fn get<P: Pearl>(&self) -> Option<UniqueRef<P>> {
        let any = self.pearls.get(&P::id())?;
        let unique = any.downcast_ref::<UniqueEntry<P>>();
        let unique = unique.expect("Internal Error: Faulty downcast");
        unique.borrow()
    }

    pub fn get_mut<P: Pearl>(&self) -> Option<UniqueMut<P>> {
        let any = self.pearls.get(&P::id())?;
        let unique = any.downcast_ref::<UniqueEntry<P>>();
        let unique = unique.expect("Internal Error: Faulty downcast");
        unique.borrow_mut()
    }

    pub fn insert<P: Pearl>(&mut self, pearl: P) -> Option<P> {
        let unique = Box::new(UniqueEntry::new(pearl));
        let any = self.pearls.insert(P::id(), unique);
        *any?.downcast().expect("Internal Error: Faulty downcast")
    }

    pub fn remove<P: Pearl>(&mut self) -> Option<P> {
        let any = self.pearls.remove(&P::id())?;
        let pearl: UniqueEntry<P> = *any.downcast().expect("Internal Error: Faulty downcast");
        let pearl = pearl.pearl.into_inner();
        Some(pearl)
    }
}
