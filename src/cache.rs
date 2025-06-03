use crate::{UArch, UHSize};

pub struct Cache<V: Default + Clone> {
    data: Box<[(UArch, V)]>,
    cap: UArch,
    access_count: UArch,
    hit_count: UArch,
}
impl<V: Default + Clone> Cache<V> {
    pub fn new(capacity: UArch) -> Self {
        let cap = capacity.next_power_of_two();
        Self {
            data: vec![(!0, V::default()); cap as UHSize].into_boxed_slice(),
            cap: cap - 1,
            access_count: 0,
            hit_count: 0,
        }
    }

    pub fn hit_ratio(&self) -> f64 {
        if self.access_count == 0 {
            return 0.0;
        }
        self.hit_count as f64 / self.access_count as f64
    }

    #[inline]
    pub fn get(&mut self, key: UArch) -> Option<&V> {
        self.access_count += 1;

        let idx = (key & self.cap) as UHSize;
        let entry = unsafe { self.data.get_unchecked(idx) };

        if entry.0 == key {
            self.hit_count += 1;
            Some(&self.data[idx].1)
        } else {
            None
        }
    }

    #[inline]
    pub fn put(&mut self, key: UArch, value: V) {
        let idx = (key & self.cap) as UHSize;
        unsafe {
            *self.data.get_unchecked_mut(idx) = (key, value);
        }
    }
}
