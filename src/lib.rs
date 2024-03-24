#[derive(Debug)]
pub struct VecPigeonhole<V> {
    free: Option<usize>,
    slots: Vec<Slot<V>>,
}

#[derive(Debug)]
pub enum Slot<V> {
    Free(Option<usize>),
    Used(V),
}

impl<V> VecPigeonhole<V> {
    pub fn new() -> Self {
        Self {
            free: None,
            slots: Vec::new(),
        }
    }

    pub fn insert(&mut self, item: V) -> usize {
        match self.free {
            Some(id) => {
                let next = std::mem::replace(&mut self.slots[id], Slot::Used(item));
                let Slot::Free(next) = next else {
                    unreachable!()
                };
                self.free = next;
                id
            }
            None => {
                let id = self.slots.len();
                self.slots.push(Slot::Used(item));
                id
            }
        }
    }

    pub fn remove(&mut self, id: usize) -> Result<V, ()> {
        let Some(entry_ref) = self.slots.get_mut(id) else {
            return Err(());
        };

        let free = self.free;
        let entry = std::mem::replace(entry_ref, Slot::Free(free));

        match entry {
            entry @ Slot::Free(_) => {
                *entry_ref = entry;
                Err(())
            }
            Slot::Used(item) => {
                self.free = Some(id);
                Ok(item)
            }
        }
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut V> {
        match self.slots.get_mut(idx)? {
            Slot::Free(_) => None,
            Slot::Used(item) => Some(item),
        }
    }

    pub fn get(&self, id: usize) -> Option<&V> {
        match self.slots.get(id)? {
            Slot::Free(_) => None,
            Slot::Used(item) => Some(item),
        }
    }

    pub fn iter(&self) -> Iter<'_, V> {
        Iter {
            iter: self.slots.iter(),
        }
    }

    pub fn into_iter(self) -> IntoIter<V> {
        IntoIter {
            iter: self.slots.into_iter(),
        }
    }
}

impl<V> Default for VecPigeonhole<V> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Iter<'a, T>
where
    Self: 'a,
{
    iter: std::slice::Iter<'a, Slot<T>>,
}

impl<'a, T> Iterator for Iter<'a, T>
where
    Self: 'a,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.find_map(|e| match e {
            Slot::Free(_) => None,
            Slot::Used(v) => Some(v),
        })
    }
}

pub struct IntoIter<T> {
    iter: std::vec::IntoIter<Slot<T>>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.find_map(|e| match e {
            Slot::Free(_) => None,
            Slot::Used(v) => Some(v),
        })
    }
}
