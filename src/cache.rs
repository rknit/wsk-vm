use crate::{UArch, UHSize};

pub struct Cache<V: Default + Copy> {
    keys: Box<[UArch]>,
    values: Box<[V]>,
    cap: UArch,
}
impl<V: Default + Copy> Cache<V> {
    pub fn new(capacity: UArch) -> Self {
        let cap = capacity.next_power_of_two();
        Self {
            keys: vec![!0; cap as UHSize].into_boxed_slice(),
            values: vec![V::default(); cap as UHSize].into_boxed_slice(),
            cap: cap - 1,
        }
    }

    #[inline]
    pub fn get(&mut self, key: UArch) -> Option<&V> {
        let idx = (key & self.cap) as UHSize;

        if unsafe { *self.keys.get_unchecked(idx) } == key {
            Some(&self.values[idx])
        } else {
            None
        }
    }

    #[inline]
    pub fn put(&mut self, key: UArch, value: V) {
        let idx = (key & self.cap) as UHSize;
        unsafe {
            *self.keys.get_unchecked_mut(idx) = key;
            *self.values.get_unchecked_mut(idx) = value;
        }
    }
}
