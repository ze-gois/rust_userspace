#![no_std]
#![no_main]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(incomplete_features)]
#![allow(unused_assignments)]
#![feature(generic_const_exprs)]
#![feature(generic_const_items)]
#![feature(const_trait_impl)]
#![feature(fundamental)]

use userspace;
use userspace::info;
use userspace::target;

use ample::traits::Bytes;

#[derive(Debug)]
pub struct Origin;

ample::trait_implement_primitives!();

ample::r#struct!(
    #[derive(Debug)]
    pub struct Example {
        pub field1: u8,
        pub field2: *const u32,
    }
);

fn demo_original_heap_allocation() {
    use userspace::memory::heap::Allocating;

    info!("=== Demonstrating Basic Heap Allocation ===\n");

    let slice = u16::allocate_slice(10);

    info!("Allocated slice: {:?}\n", slice);

    slice[5] = 42;

    info!("Modified slice: {:?}\n", slice);

    u16::deallocate_slice(slice);

    let a = 3u32;
    let b: *const u32 = &a;

    let e = Example {
        field1: 3,
        field2: b,
    };

    info!("Example struct: {:?}\n", e);

    let b = e.to_le_bytes();

    info!("Struct as bytes: {:?}\n", b);

    let x = Example::from_le_bytes(b);

    info!("Struct from bytes: {:?}\n", x);

    info!(
        "Pointer bytes: {:?}",
        <*const u32 as Bytes<Origin>>::to_le_bytes(&x.field2)
    );

    let ee = e.clone();

    info!("Original struct: {:?}\n", e);
    info!("Cloned struct: {:?}\n", ee);

    info!("{:?}\n", Example::default());
}

fn demo_linked_list() {
    info!("\n=== Demonstrating Linked List with Heap Allocation ===\n");

    // Create a new linked list using our SystemAllocator
    let mut list = ample::list::LinkedList::<
        Origin,
        userspace::Origin,
        u32,
        userspace::memory::heap::Allocator,
    >::new();

    info!("Created empty list\n");
    info!("List length: {}\n", list.numerosity());

    // Add elements to the list
    list.push_back(10);
    list.push_back(20);
    list.push_back(30);
    list.push_front(5);

    info!("Added elements: 5, 10, 20, 30\n");
    info!("List length: {}\n", list.numerosity());

    // Iterate through the list
    info!("List contents: \n");
    for value in list.iter() {
        info!("{} \n", value);
    }
    info!("\n\n");

    // Access front and back
    info!("Front element: {:?}\n", list.front());
    info!("Back element: {:?}\n", list.back());

    // Pop elements
    info!("Popped from front: {:?}\n", list.pop_front());
    info!("Popped from front: {:?}\n", list.pop_front());

    info!("List length after popping: {}\n", list.numerosity());

    // Check if we can still iterate
    info!("Remaining contents: \n");
    for value in list.iter() {
        info!("{} \n", value);
    }
    info!("\n\n");

    // Clear the list
    list.clear();
    info!("Cleared list, length: {}\n", list.numerosity());

    // Using a custom struct in the list
    info!("\n=== Demonstrating Linked List with Custom Type ===\n");

    // Define a custom struct using the struct macro
    ample::r#struct!(
        #[derive(Debug)]
        pub struct Person {
            pub id: u32,
            pub age: u8,
        }
    );

    macro_rules! lister {
        ($identifier:ident) => {
            ample::list::LinkedList::<
                Origin,
                userspace::Origin,
                $identifier,
                userspace::memory::heap::Allocator,
            >::new()
        };
    }

    // let mut person_list = ample::list::LinkedList::<
    //     Origin,
    //     userspace::Origin,
    //     Person,
    //     userspace::memory::heap::Allocator,
    // >::new();

    let mut person_list = lister!(Person);

    ample::r#enum!(
        u8;
        #[derive(Debug)]
        pub enum Our {
            A(u8) = 3,
            B(Person) = 1,
            C(Example) = 2,
        }
    );

    let mut our_list = lister!(Our);
    our_list.push_back(Our::A(1));
    our_list.push_back(Our::B(Person { id: 1, age: 25 }));
    our_list.push_back(Our::A(2));
    our_list.push_back(Our::B(Person { id: 2, age: 30 }));
    our_list.push_back(Our::A(3));
    our_list.push_back(Our::B(Person { id: 3, age: 40 }));
    info!("========> {:?}\n", our_list);

    person_list.push_back(Person { id: 1, age: 25 });
    person_list.push_back(Person { id: 2, age: 30 });
    person_list.push_back(Person { id: 3, age: 40 });

    info!("Added 3 persons to list\n");
    info!("List length: {}\n", person_list.numerosity());

    // Print front person
    if let Some(person) = person_list.front() {
        info!("First person: id={}, age={}\n", person.id, person.age);
    }

    // The list will automatically deallocate all nodes when it goes out of scope
    info!("Lists will be automatically cleaned up when they go out of scope\n");
}

#[unsafe(no_mangle)]
pub extern "C" fn entry(stack_pointer: crate::target::arch::PointerType) -> ! {
    let stack_pointer = crate::target::arch::Pointer(stack_pointer);

    info!("eXecuting Executable and Linkable Format\n\n\n");

    let argc = stack_pointer.0 as *const usize;
    info!("argc={:?}\n\n", unsafe { *argc });
    let stack = userspace::memory::Stack::from_pointer(stack_pointer);
    stack.print();
    stack.arguments.print();

    let arg0 = stack.arguments.get(0).unwrap();
    let arg0_pointer = arg0.pointer;

    if !arg0.pointer.0.is_null() {
        unsafe {
            let cstr = core::ffi::CStr::from_ptr(arg0.pointer.0 as *mut i8);
            let self_path = cstr.to_str().unwrap();
            userspace::info!("\n{:?}\n\n", self_path);
            // let identifier = userspace::file::format::elf::header::Identifier::from_path(self_path);
            // userspace::info!("{:?}\n\n", identifier);
        }
    }

    // let uchar32 = userspace::file::format::elf::dtype::class_32::UChar(3);

    // demo_original_heap_allocation();

    // New example of linked list with heap allocation
    // demo_linked_list();

    panic!();
}
