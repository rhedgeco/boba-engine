use crate::ComponentId;

use super::{ComponentIdArray, ComponentIdSlice};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ComponentQuery {
    Ref(ComponentId),
    Mut(ComponentId),
}

impl ComponentQuery {
    pub const fn id(&self) -> ComponentId {
        match self {
            Self::Ref(id) => *id,
            Self::Mut(id) => *id,
        }
    }

    pub const fn is_ref(&self) -> bool {
        match self {
            Self::Ref(_) => true,
            _ => false,
        }
    }

    pub const fn is_mut(&self) -> bool {
        match self {
            Self::Mut(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ComponentQueryArray<const SIZE: usize> {
    ids: ComponentIdArray<SIZE>,
    ordered: [ComponentQuery; SIZE],
    sorted: [ComponentQuery; SIZE],
}

impl<const SIZE: usize> ComponentQueryArray<SIZE> {
    pub const fn new(queries: [ComponentQuery; SIZE]) -> Option<Self> {
        let ordered = queries;
        let mut sorted = queries;

        // bubble sort
        loop {
            let mut i = 1;
            let mut swapped = false;

            while i < SIZE {
                let left = sorted[i - 1].id().raw_value();
                let right = sorted[i].id().raw_value();

                if left > right {
                    let tmp = sorted[i - 1];
                    sorted[i - 1] = sorted[i];
                    sorted[i] = tmp;
                    swapped = true;
                } else if left == right {
                    return None;
                }

                i += 1;
            }

            if !swapped {
                break;
            }
        }

        // create id array out of sorted parts
        let mut index = 0;
        let mut ids = [ComponentId::from_raw(0); SIZE];
        while index < SIZE {
            ids[index] = sorted[index].id();
            index += 1;
        }
        let Some(ids) = ComponentIdArray::build(ids) else {
            return None;
        };

        Some(Self {
            ids,
            ordered,
            sorted,
        })
    }

    pub const fn as_id_array(&self) -> &ComponentIdArray<SIZE> {
        &self.ids
    }

    pub const fn as_id_slice(&self) -> ComponentIdSlice {
        self.ids.as_slice()
    }

    pub const fn as_slice(&self) -> ComponentQuerySlice {
        ComponentQuerySlice {
            ids: self.ids.as_slice(),
            ordered: &self.ordered,
            sorted: &self.sorted,
        }
    }

    pub const fn as_ordered_slice(&self) -> &[ComponentQuery] {
        &self.ordered
    }

    pub const fn as_sorted_slice(&self) -> &[ComponentQuery] {
        &self.sorted
    }

    pub const fn alias_overlaps_slice(&self, other: &ComponentQuerySlice) -> bool {
        alias_overlaps(self.as_sorted_slice(), other.as_sorted_slice())
    }

    pub const fn alias_overlaps_array<const SIZE2: usize>(
        &self,
        other: &ComponentQueryArray<SIZE2>,
    ) -> bool {
        alias_overlaps(self.as_sorted_slice(), other.as_sorted_slice())
    }
}

pub const fn unwrap_query<const SIZE: usize>(
    option: Option<ComponentQueryArray<SIZE>>,
) -> ComponentQueryArray<SIZE> {
    match option {
        Some(array) => array,
        None => panic!("Duplicate ids detected in query"),
    }
}

pub struct ComponentQuerySlice<'a> {
    ids: ComponentIdSlice<'a>,
    ordered: &'a [ComponentQuery],
    sorted: &'a [ComponentQuery],
}

impl<'a> ComponentQuerySlice<'a> {
    pub const fn as_id_slice(&self) -> &ComponentIdSlice<'a> {
        &self.ids
    }

    pub const fn as_ordered_slice(&self) -> &[ComponentQuery] {
        self.ordered
    }

    pub const fn as_sorted_slice(&self) -> &[ComponentQuery] {
        self.sorted
    }

    pub const fn alias_overlaps_slice(&self, other: &ComponentQuerySlice) -> bool {
        alias_overlaps(self.as_sorted_slice(), other.as_sorted_slice())
    }

    pub const fn alias_overlaps_array<const SIZE: usize>(
        &self,
        other: &ComponentQueryArray<SIZE>,
    ) -> bool {
        alias_overlaps(self.as_sorted_slice(), other.as_sorted_slice())
    }
}

const fn alias_overlaps(slice1: &[ComponentQuery], slice2: &[ComponentQuery]) -> bool {
    let mut index1 = 0;
    let mut index2 = 0;

    while index1 < slice1.len() {
        if index2 >= slice2.len() {
            return false;
        }

        let query1 = &slice1[index1];
        let query2 = &slice2[index2];
        let value1 = query1.id().raw_value();
        let value2 = query2.id().raw_value();
        if value1 < value2 {
            index1 += 1;
        } else if value1 > value2 {
            index2 += 1;
        } else if query1.is_mut() || query2.is_mut() {
            return true;
        }
    }

    false
}
