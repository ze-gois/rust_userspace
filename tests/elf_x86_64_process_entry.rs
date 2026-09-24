use std::process::Command;

use userspace::{
    file::format::elf::{
        ObjectFile,
        processor_specific::x86_64::{
            entry_point,
            stack::initial::{AuxiliaryEntry, Image as InitialStack},
        },
    },
    target::operating_system::{process_image, process_stack},
};

const CHILD_ENV: &str = "USERSPACE_X86_64_PROCESS_ENTRY_CHILD";
const PAGE_SIZE: usize = 0x1000;
const HEADER_SIZE: u64 = 64;
const PROGRAM_HEADER_SIZE: u64 = 56;
const ENTRY_OFFSET: u64 = 0x100;

fn half(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn word(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn xword(bytes: &mut Vec<u8>, value: u64) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn fixture() -> Vec<u8> {
    // The entry code validates:
    //   rsp % 16 == 0
    //   argc == 1
    //   rdx == 0
    //   rbp == 0
    // and terminates the whole subprocess with exit status 42.
    //
    // Failure terminates it with status 1.
    let code: [u8; 49] = [
        0x48, 0x89, 0xe0,             // mov rax, rsp
        0x83, 0xe0, 0x0f,             // and eax, 15
        0x75, 0x1d,                   // jnz fail
        0x48, 0x83, 0x3c, 0x24, 0x01, // cmp qword ptr [rsp], 1
        0x75, 0x16,                   // jne fail
        0x48, 0x85, 0xd2,             // test rdx, rdx
        0x75, 0x11,                   // jnz fail
        0x48, 0x85, 0xed,             // test rbp, rbp
        0x75, 0x0c,                   // jnz fail
        0xbf, 0x2a, 0x00, 0x00, 0x00, // mov edi, 42
        0xb8, 0xe7, 0x00, 0x00, 0x00, // mov eax, 231 (exit_group)
        0x0f, 0x05,                   // syscall
        0xbf, 0x01, 0x00, 0x00, 0x00, // fail: mov edi, 1
        0xb8, 0xe7, 0x00, 0x00, 0x00, // mov eax, 231 (exit_group)
        0x0f, 0x05,                   // syscall
    ];

    let total_size = ENTRY_OFFSET as usize + code.len();
    let mut bytes = Vec::with_capacity(total_size);

    bytes.extend_from_slice(&[
        0x7f, b'E', b'L', b'F',
        2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);
    half(&mut bytes, 3); // ET_DYN
    half(&mut bytes, 62); // EM_X86_64
    word(&mut bytes, 1); // EV_CURRENT
    xword(&mut bytes, ENTRY_OFFSET);
    xword(&mut bytes, HEADER_SIZE);
    xword(&mut bytes, 0);
    word(&mut bytes, 0);
    half(&mut bytes, HEADER_SIZE as u16);
    half(&mut bytes, PROGRAM_HEADER_SIZE as u16);
    half(&mut bytes, 1);
    half(&mut bytes, 64);
    half(&mut bytes, 0);
    half(&mut bytes, 0);

    // One executable PT_LOAD containing the whole object.
    word(&mut bytes, 1);
    word(&mut bytes, 5); // PF_R | PF_X
    xword(&mut bytes, 0);
    xword(&mut bytes, 0);
    xword(&mut bytes, 0);
    xword(&mut bytes, total_size as u64);
    xword(&mut bytes, total_size as u64);
    xword(&mut bytes, PAGE_SIZE as u64);

    bytes.resize(ENTRY_OFFSET as usize, 0);
    bytes.extend_from_slice(&code);

    assert_eq!(bytes.len(), total_size);
    bytes
}

#[test]
fn transfers_control_to_mapped_x86_64_process_entry() {
    let executable = std::env::current_exe()
        .expect("current integration-test executable must be known");

    let status = Command::new(executable)
        .arg("--exact")
        .arg("child_process_entry_transfer_control")
        .env(CHILD_ENV, "1")
        .status()
        .expect("process-entry child must start");

    assert_eq!(status.code(), Some(42));
}

#[test]
fn child_process_entry_transfer_control() {
    if std::env::var_os(CHILD_ENV).is_none() {
        return;
    }

    let bytes = fixture();
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    let mapping = process_image::map(&object, PAGE_SIZE)
        .expect("ELF process image must map");
    let entry = mapping
        .entry_address(&object)
        .expect("ELF entry must be executable");

    let argument = b"program\0";
    let arguments = [argument.as_ptr()];
    let auxiliary = [
        AuxiliaryEntry::new(6, PAGE_SIZE),
        AuxiliaryEntry::new(9, entry as usize),
    ];
    let initial_stack = InitialStack::new(&arguments, &[], &auxiliary);

    let mut stack_mapping =
        process_stack::map(0x4000).expect("process stack must map");
    let stack_pointer = stack_mapping
        .write(&initial_stack)
        .expect("initial process stack must write");

    unsafe {
        entry_point::transfer_control(
            entry,
            stack_pointer,
            None,
        )
    }
}
