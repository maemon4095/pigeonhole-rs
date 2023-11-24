pub struct VecPigeonhole<V> {
    free: Option<usize>,
    slots: Vec<Slot<V>>,
}

pub enum Slot<V> {
    Free(Option<usize>),
    Used(V),
}

impl<V> Slot<V> {
    fn used(&self) -> Option<&V> {
        match self {
            Slot::Free(_) => None,
            Slot::Used(v) => Some(v),
        }
    }
}

impl<V> VecPigeonhole<V> {
    pub fn new() -> Self {
        Self {
            free: None,
            slots: Vec::new(),
        }
    }

    fn grow(&mut self) {
        let last_len = self.slots.len();
        let next_free_idx = last_len;
        let free_slots = std::iter::successors(Some(Slot::Free(Some(next_free_idx + 1))), |n| {
            let Slot::Free(Some(n)) = n else {
                unreachable!()
            };
            Some(Slot::Free(Some(n + 1)))
        })
        .take(last_len)
        .chain(std::iter::once(Slot::Free(None)));

        self.free = Some(next_free_idx);
        self.slots.extend(free_slots);
    }

    pub fn insert(&mut self, item: V) -> usize {
        match self.free {
            Some(idx) => {
                let next = std::mem::replace(&mut self.slots[idx], Slot::Used(item));
                let Slot::Free(next) = next else {
                    unreachable!()
                };
                self.free = next;
                idx
            }
            None => {
                self.grow();
                self.insert(item)
            }
        }
    }

    pub fn remove(&mut self, idx: usize) -> Result<V, ()> {
        match &self.slots[idx] {
            Slot::Free(_) => Err(()),
            Slot::Used(_) => {
                let next = self.free.replace(idx);
                let last_slot = std::mem::replace(&mut self.slots[idx], Slot::Free(next));

                let Slot::Used(item) = last_slot else {
                    unreachable!()
                };

                Ok(item)
            }
        }
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut V> {
        match &mut self.slots[idx] {
            Slot::Free(_) => None,
            Slot::Used(item) => Some(item),
        }
    }

    pub fn get(&self, idx: usize) -> Option<&V> {
        match &self.slots[idx] {
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
        self.iter.find_map(Slot::used)
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
