use crate::info;
use crate::target::os::syscall;

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

use crate::target::arch::{Pointer, PointerType};

impl List {
    #[rustfmt::skip]
    pub fn from_pointer(environment_pointer: Pointer) -> (List, Pointer) {
        let environment_pointer: *mut PointerType = environment_pointer.0 as *mut PointerType;

        let mut counter = 0;
        unsafe {
            while !(*environment_pointer.add(counter)).is_null() {
                counter += 1;
            }
        }

        let auxiliary_pointer = unsafe { (environment_pointer as PointerType).add(1 + counter) };

        if counter == 0 {
            return (List::default(), Pointer(auxiliary_pointer));
        }

        let list_pointer = crate::memory::alloc::<Entry>(counter);

        unsafe {
            // preenche cada Entry in-place
            for a in 0..counter {
                let entry_pointer = *(environment_pointer.add(a));
                let entry = Entry::from_pointer(Pointer(entry_pointer));
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

        (list, Pointer(auxiliary_pointer))
    }

    pub fn print(&self) {
        info!("Environment {{\n");
        for a in 0..self.counter {
            if let Some(e) = self.get(a) {
                info!(
                    "\t{:?} @ {:?}\n",
                    unsafe { Pointer(self.former.add(a) as PointerType) },
                    e
                );
            }
        }
        info!("}} Environment \n");
    }

    pub fn print_arguments(&self) {
        info!("Environment count: {}\n", self.counter);
        for a in 0..self.counter {
            if let Some(entry) = self.get(a) {
                // Assumindo Entry tem campo `value: *PointerType` ou similar; ajustar conforme Entry real.
                // unsafe {
                // se Entry tiver método para converter a string, use-o aqui
                info!("Arg {}: '{:?}'\n", a, entry.pointer);
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

                let _ = syscall::munmap(self.former as *mut u8, aligned_size);
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

// pub fn from_pointer(environment_pointer: Pointer) -> Self {

//         info!("Environment count: {:?}\n\n", counter);

//         if counter == 0 {
//             return Self::default();
//         }

//         // Allocate memory using mmap
//         let entries_ptr = unsafe {
//             let size = core::mem::size_of::<Entry<'e>>() * counter;

//             // Align size to page boundary
//             let page_size = 4096;
//             let aligned_size = (size + page_size - 1) & !(page_size - 1);

//             // Define mmap constants locally since they're not accessible
//             const PROT_READ: i32 = 0x1;
//             const PROT_WRITE: i32 = 0x2;
//             const MAP_PRIVATE: i32 = 0x02;
//             const MAP_ANONYMOUS: i32 = 0x20;

//             let result = memory::mmap::mmap(
//                 ptr::null_mut(),
//                 aligned_size,
//                 PROT_READ | PROT_WRITE,
//                 MAP_PRIVATE | MAP_ANONYMOUS,
//                 -1,
//                 0,
//             );

//             match result {
//                 Ok(ptr) => ptr as *mut Entry<'e>,
//                 Err(_) => {
//                     // Allocation failed, return default vector
//                     info!("Failed to allocate memory for environment vector\n");
//                     return Self::default();
//                 }
//             }
//         };

//         // Create entries from the environment pointers
//         for i in 0..counter {
//             unsafe {
//                 let env_ptr = environment_pointer.add(i);
//                 let entry = Entry::from_pointer(env_ptr, i);
//                 ptr::write(entries_ptr.add(i), entry);
//                 // info!("Env {}: {:?}\n", i, (*entries_ptr.add(i)).value);
//             }
//         }

//         // Move past the environment array and the NULL terminator
//         let next_pointer = unsafe { environment_pointer.add(counter + 1) };

//         let environment = Self {
//             counter,
//             entries: entries_ptr,
//         };

//         environment
//     }
