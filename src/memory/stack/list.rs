use crate::memory::heap::Allocating;

pub trait LinkedEntry {
    fn set_links(&mut self, previous: *mut Self, next: *mut Self);
}

#[repr(C)]
#[derive(Debug)]
pub struct List<T> {
    pub counter: usize,
    pub former: *mut T,
    pub latter: *mut T,
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self {
            counter: 0,
            former: core::ptr::null_mut(),
            latter: core::ptr::null_mut(),
        }
    }
}

impl<T> List<T> {
    pub fn from_values<F>(counter: usize, mut value_at: F) -> Self
    where
        T: Allocating<T> + LinkedEntry,
        F: FnMut(usize) -> T,
    {
        if counter == 0 {
            return Self::default();
        }

        let pointer = T::allocate(counter);
        if pointer.is_null() {
            return Self::default();
        }

        unsafe {
            for index in 0..counter {
                core::ptr::write(pointer.add(index), value_at(index));
            }

            for index in 0..counter {
                let entry = &mut *pointer.add(index);
                let previous = if index == 0 {
                    core::ptr::null_mut()
                } else {
                    pointer.add(index - 1)
                };
                let next = if index + 1 == counter {
                    core::ptr::null_mut()
                } else {
                    pointer.add(index + 1)
                };
                entry.set_links(previous, next);
            }
        }

        Self {
            counter,
            former: pointer,
            latter: unsafe { pointer.add(counter - 1) },
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.counter || self.former.is_null() {
            return None;
        }
        unsafe { Some(&*self.former.add(index)) }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.counter || self.former.is_null() {
            return None;
        }
        unsafe { Some(&mut *self.former.add(index)) }
    }

    pub fn len(&self) -> usize {
        self.counter
    }

    pub fn is_empty(&self) -> bool {
        self.counter == 0
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            list: self,
            index: 0,
        }
    }

    pub fn print(&self)
    where
        T: core::fmt::Debug,
    {
        crate::info!("Stack list {{\n");
        for (index, entry) in self.iter().enumerate() {
            crate::info!("\t[{}] {:?}\n", index, entry);
        }
        crate::info!("}} Stack list\n");
    }

    pub fn print_values(&self)
    where
        T: core::fmt::Debug,
    {
        crate::info!("Stack list count: {}\\n", self.counter);
        for (index, entry) in self.iter().enumerate() {
            crate::info!("Entry {}: {:?}\\n", index, entry);
        }
    }

    pub fn print_arguments(&self)
    where
        T: core::fmt::Debug,
    {
        self.print_values();
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        if self.former.is_null() || self.counter == 0 {
            return;
        }

        unsafe {
            for index in 0..self.counter {
                core::ptr::drop_in_place(self.former.add(index));
            }

            let total_size = core::mem::size_of::<T>() * self.counter;
            let aligned_size =
                (total_size + crate::memory::page::SIZE - 1) & !(crate::memory::page::SIZE - 1);
            let _ = crate::target::os::syscall::munmap(self.former as *mut u8, aligned_size);
        }

        self.former = core::ptr::null_mut();
        self.latter = core::ptr::null_mut();
        self.counter = 0;
    }
}

pub struct Iter<'list, T> {
    list: &'list List<T>,
    index: usize,
}

impl<'list, T> Iterator for Iter<'list, T> {
    type Item = &'list T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.list.counter {
            return None;
        }

        let item = self.list.get(self.index);
        self.index += 1;
        item
    }
}
