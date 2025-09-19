pub mod entry;
pub use entry::*;
pub mod atype;

pub use atype::Type;
pub use atype::TypeTrait;

use crate::memory::allocation::heap::Allocating;

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
    pub fn from_pointer(auxiliary_pointer: crate::target::arch::Pointer) -> (List, crate::target::arch::Pointer) {
        let auxiliary_pointer: *mut crate::target::arch::PointerType = auxiliary_pointer.0 as *mut crate::target::arch::PointerType;

        let mut counter = 0;
        unsafe {
            loop {
                if auxiliary_pointer.add(counter).is_null() {
                    break;
                }
                let key_pointer = auxiliary_pointer.add(counter);
                let key_pointer = key_pointer as *mut usize;
                let value_pointer = key_pointer.add(1) as *mut u8;
                counter += 1;
                let pair = Type::from_pair(key_pointer, value_pointer);
                if pair.is_null() {
                    break
                }
            }
        }

        crate::info!("done counting\n");

        let latter_pointer = unsafe { (auxiliary_pointer as crate::target::arch::PointerType).add(1 + counter) };

        if counter == 0 {
            return (List::default(), crate::target::arch::Pointer(latter_pointer));
        }

        let list_pointer = Entry::allocate(counter);

        unsafe {
            // preenche cada Entry in-place
            for a in 0..counter {
                let entry_pointer = auxiliary_pointer.add(a) as crate::target::arch::PointerType;
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

        (list, crate::target::arch::Pointer(latter_pointer))
    }

    pub fn print(&self) {
        crate::info!("Auxiliary!!! {{\n");
        for a in 0..self.counter {
            if let Some(e) = self.get(a) {
                crate::info!("\t{:?} @ ", unsafe {
                    crate::target::arch::Pointer(
                        self.former.add(a) as crate::target::arch::PointerType
                    )
                },);
                crate::info!("{:?}\n", e);
            }
        }
        crate::info!("}} Auxiliary \n");
    }

    pub fn print_arguments(&self) {
        crate::info!("Auxiliary count: {}\n", self.counter);
        for a in 0..self.counter {
            if let Some(entry) = self.get(a) {
                // Assumindo Entry tem campo `value: *crate::target::arch::PointerType` ou similar; ajustar conforme Entry real.
                // unsafe {
                // se Entry tiver método para converter a string, use-o aqui
                crate::info!("Arg {}: '{:?}'", a, entry.pointer);
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

// pub fn from_pointer(auxiliary_pointer: crate::target::arch::Pointer) -> Self {

//         crate::info!("Environment count: {:?}\n\n", counter);

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
//                     crate::info!("Failed to allocate memory for environment vector\n");
//                     return Self::default();
//                 }
//             }
//         };

//         // Create entries from the environment pointers
//         for i in 0..counter {
//             unsafe {
//                 let env_ptr = auxiliary_pointer.add(i);
//                 let entry = Entry::from_pointer(env_ptr, i);
//                 ptr::write(entries_ptr.add(i), entry);
//                 // crate::info!("Env {}: {:?}\n", i, (*entries_ptr.add(i)).value);
//             }
//         }

//         // Move past the environment array and the NULL terminator
//         let next_pointer = unsafe { auxiliary_pointer.add(counter + 1) };

//         let environment = Self {
//             counter,
//             entries: entries_ptr,
//         };

//         environment
//     }

// pub mod atype;
// pub mod entry;

// pub use atype::Type;
// pub use entry::*;

// use human::info;

// use core::crate::target::arch::x86_64;

// #[repr(C)]
// #[derive(Debug, Clone, Copy)]
// pub struct Vector {
//     pub pointer: crate::target::arch::Pointer,
//     pub counter: Option<usize>,
//     pub entries: *mut Entry,
// }

// impl Default for Vector {
//     fn default() -> Self {
//         Self {
//             counter: None,
//             entries: 0x0 as *mut Entry,
//         }
//     }
// }

// impl Vector {
//     pub fn from_pointer(auxv_pointer: crate::target::arch::Pointer) -> Self {
//         Self::default()
//     }

//     pub fn new(entries: *mut Entry) -> Self {
//         Self {
//             entries,
//             counter: None,
//         }
//     }

//     pub fn counter(&mut self) -> usize {
//         if let Some(counter) = self.counter {
//             return counter;
//         }

//         let mut counter = 0;
//         let mut auxv = self.entries;
//         loop {
//             let auxv_entry = unsafe { *auxv.offset(counter as isize) };

//             if auxv_entry.atype == 0 {
//                 break;
//             }

//             counter += 1;
//         }

//         self.counter = Some(counter);
//         counter
//     }

//     pub fn get_by_index(&mut self, index: usize) -> Option<*mut Entry> {
//         if index < self.counter() {
//             return Some(unsafe { self.entries.offset(index as isize) });
//         }
//         None
//     }

//     pub fn get_by_type_id(&mut self, atype: usize) -> Option<*mut Entry> {
//         for av in 0..self.counter() {
//             let entry = unsafe { self.entries.offset(av as isize) };

//             if (unsafe { *entry }).atype == atype {
//                 return Some(entry);
//             }
//         }
//         None
//     }

//     pub fn get_by_type(&mut self, atype: Type) -> Option<*mut Entry> {
//         self.get_by_type_id(atype.to())
//     }

//     pub fn set_by_type_id(&mut self, atype: usize, value: usize) {
//         if let Some(entry) = self.get_by_type_id(atype) {
//             unsafe { (*entry).value = value };
//         }
//     }

//     pub fn set_by_type(&mut self, atype: Type, value: usize) {
//         self.set_by_type_id(atype.to(), value);
//     }

//     pub fn print(self) {
//         crate::info!("tobeaprint");
//         return ();
//         let counter = self.counter();

//         (0..counter).for_each(|av| {
//             let auxv_entry = unsafe { self.entries.offset(av as isize) };

//             let auxv_type = unsafe { Type::from((*auxv_entry).atype) as u64 };
//             crate::info!("Auxv: {} = ", auxv_type);
//             if !unsafe { ((*auxv_entry).value as *const u8).is_null() } {
//                 // crate::info!(auxv_entry.value as *const u8);
//             } else {
//                 crate::info!("NULL");
//             }
//             crate::info!("'\n");
//         });

//         crate::info!("auxv at: {} \n", self.pointer.0 as u64);
//         let mut av = 0;
//         unsafe {
//             while !self.entries.offset(av).is_null() && (*self.entries.offset(av)).atype != 0 {
//                 let auxv_entry = *self.entries.offset(av);

//                 let a = Type::from(auxv_entry.atype).as_str();
//                 let b = auxv_entry.atype as u64;
//                 crate::info!("\tAuxv: {} ({}) = ", a, b);

//                 if !(auxv_entry.value as *const u8).is_null() {
//                     let s = crate::misc::as_str(auxv_entry.value as *const u8);
//                     match auxv_entry.atype {
//                         31 => crate::info!("{}", s),
//                         _ => crate::info!("{}", auxv_entry.value as u64),
//                     }
//                 } else {
//                     crate::info!("NULL");
//                 }
//                 crate::info!("'\n");
//                 av += 1;
//             }
//         }
//         crate::info!("\n=======\nAuxvCount={};\n=======\n", av as u64);
//     }
// }
