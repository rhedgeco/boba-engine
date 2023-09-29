use std::cmp::Ordering;

use crate::ComponentId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ComponentIdArray<const SIZE: usize> {
    ids: [ComponentId; SIZE],
}

impl<const SIZE: usize> ComponentIdArray<SIZE> {
    pub const fn build(mut ids: [ComponentId; SIZE]) -> Option<Self> {
        // bubble sort
        loop {
            let mut i = 1;
            let mut swapped = false;

            while i < SIZE {
                let left = ids[i - 1].raw_value();
                let right = ids[i].raw_value();

                if left > right {
                    ids[i - 1] = ComponentId::from_raw(right);
                    ids[i] = ComponentId::from_raw(left);
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

        Some(Self { ids })
    }

    pub const fn as_slice(&self) -> ComponentIdSlice {
        ComponentIdSlice { ids: &self.ids }
    }

    pub const fn as_raw_slice(&self) -> &[ComponentId] {
        &self.ids
    }

    pub const fn const_cmp_slice(&self, other: &ComponentIdSlice) -> Ordering {
        const_cmp(self.as_raw_slice(), other.as_raw_slice())
    }

    pub const fn const_cmp_array<const SIZE2: usize>(
        &self,
        other: &ComponentIdArray<SIZE2>,
    ) -> Ordering {
        const_cmp(self.as_raw_slice(), other.as_raw_slice())
    }
}

pub const fn unwrap_array<const SIZE: usize>(
    option: Option<ComponentIdArray<SIZE>>,
) -> ComponentIdArray<SIZE> {
    match option {
        Some(array) => array,
        None => panic!("Duplicate ids detected in query"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ComponentIdSlice<'a> {
    ids: &'a [ComponentId],
}

impl<'a> ComponentIdSlice<'a> {
    pub const fn as_raw_slice(&self) -> &[ComponentId] {
        self.ids
    }

    pub const fn const_cmp_slice(&self, other: &ComponentIdSlice) -> Ordering {
        const_cmp(self.as_raw_slice(), other.as_raw_slice())
    }

    pub const fn const_cmp_array<const SIZE: usize>(
        &self,
        other: &ComponentIdArray<SIZE>,
    ) -> Ordering {
        const_cmp(self.as_raw_slice(), other.as_raw_slice())
    }
}

const fn const_cmp(slice1: &[ComponentId], slice2: &[ComponentId]) -> Ordering {
    // loop over every item in slice1
    let mut i = 0;
    while i < slice1.len() {
        // if there is nothing left in slice2,
        // slice1 has to be greater than slice2
        if i >= slice2.len() {
            return Ordering::Greater;
        }

        // compare left value to right value
        let left = slice1[i].raw_value();
        let right = slice2[i].raw_value();
        if left > right {
            // if left is greater, then slice1 is greater
            return Ordering::Greater;
        } else if left < right {
            // if left is lesser, then slice1 is lesser
            return Ordering::Less;
        }

        // increment i for the next iteration
        i += 1;
    }

    // if slice2 still has items left,
    // slice1 has to be lesser than slice2
    if i < slice2.len() {
        return Ordering::Less;
    }

    // if we make it here, then every item in each slice matched
    Ordering::Equal
}
