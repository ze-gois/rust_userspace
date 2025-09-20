pub mod entry;
pub use entry::*;

#[repr(C)]
#[derive(Debug)]
pub struct List {
    pub counter: usize,
    pub former: *mut Entry,
    pub latter: *mut Entry,
}

impl Default for List {
    fn default() -> List {
        List {
            counter: 0,
            former: core::ptr::null_mut(),
            latter: core::ptr::null_mut(),
        }
    }
}

impl List {
    #[rustfmt::skip]
    pub fn from_pointer(stack_pointer: crate::target::arch::Pointer) -> (List, crate::target::arch::Pointer) {
        let counter = unsafe { *(stack_pointer.0) } as usize;
        let argument_pointers = unsafe { (stack_pointer.0).add(1) as *const crate::target::arch::PointerType };
        let environment_pointer = unsafe { (stack_pointer.0).add(2 + counter) };

        if counter == 0 {
            return (List::default(), crate::target::arch::Pointer(environment_pointer));
        }

        use crate::memory::heap::Allocating;
        let list_pointer = Entry::allocate(counter);

        unsafe {
            // preenche cada Entry in-place
            for a in 0..counter {
                let entry_pointer = *(argument_pointers.add(a));
                let entry = Entry::from_pointer(crate::target::arch::Pointer(entry_pointer));
                core::ptr::write(list_pointer.add(a), entry);
            }
            // liga prev/next
            for a in 0..counter {
                let entry = list_pointer.add(a);
                (*entry).prev = if a == 0 { core::ptr::null_mut() } else { list_pointer.add(a - 1) };
                (*entry).next = if a + 1 == counter { core::ptr::null_mut() } else { list_pointer.add(a + 1) };
            }
        }

        let list = List {
            counter,
            former: list_pointer,
            latter: unsafe { list_pointer.add(counter - 1) },
        };

        (list, crate::target::arch::Pointer(environment_pointer))
    }

    pub fn print(&self) {
        crate::info!("Arguments {{\n");
        for a in 0..self.counter {
            if let Some(e) = self.get(a) {
                crate::info!(
                    "\t{:?} @ {:?}\n",
                    unsafe {
                        crate::target::arch::Pointer(
                            self.former.add(a) as crate::target::arch::PointerType
                        )
                    },
                    e
                );
            }
        }
        crate::info!("}} Arguments \n");
    }

    pub fn print_arguments(&self) {
        crate::info!("Argument count: {}\n", self.counter);
        for a in 0..self.counter {
            if let Some(entry) = self.get(a) {
                // Assumindo Entry tem campo `value: *crate::target::arch::PointerType` ou similar; ajustar conforme Entry real.
                // unsafe {
                // se Entry tiver método para converter a string, use-o aqui
                crate::info!("Arg {}: '{:?}'\n", a, entry.pointer);
                // }
            }
        }
    }

    pub fn get(&self, index: usize) -> Option<&Entry> {
        if index >= self.counter || self.former.is_null() {
            return None;
        }
        unsafe { Some(&*self.former.add(index)) }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Entry> {
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
}

pub struct Iter<'l> {
    list: &'l List,
    index: usize,
}

impl Drop for List {
    fn drop(&mut self) {
        if !self.former.is_null() && self.counter > 0 {
            unsafe {
                // primeiro dropar cada Entry in-place (isso chama Drop de Entry, se houver)
                for a in 0..self.counter {
                    let entry_ptr = self.former.add(a);
                    core::ptr::drop_in_place(entry_ptr);
                }

                // desaloca o bloco que foi alocado por alloc
                let total_size = core::mem::size_of::<Entry>() * self.counter as usize;
                let aligned_size =
                    (total_size + crate::memory::page::SIZE - 1) & !(crate::memory::page::SIZE - 1);

                let _ = crate::target::os::syscall::munmap(self.former as *mut u8, aligned_size);
                // opcional: limpar para evitar double-drop
                self.former = core::ptr::null_mut();
                self.latter = core::ptr::null_mut();
                self.counter = 0;
            }
        }
    }
}

impl<'l> Iterator for Iter<'l> {
    type Item = &'l Entry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.list.counter as usize {
            return None;
        }
        let item = self.list.get(self.index);
        self.index += 1;
        item
    }
}
