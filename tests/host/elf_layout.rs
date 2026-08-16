#![cfg(feature = "host_tests")]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Once;

use ample::traits::Bytes;
use userspace::file::format::elf::header::{Header32, Header64, Identifier};
use userspace::file::format::elf::segment::header::{
    Header32 as ProgramHeader32, Header64 as ProgramHeader64,
};

type Origin = userspace::Origin;

static BUILD_FIXTURES: Once = Once::new();

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("target/elf-fixtures")
}

fn ensure_fixtures() -> PathBuf {
    BUILD_FIXTURES.call_once(|| {
        let status = Command::new("sh")
            .arg("tests/elf_fixtures/build.sh")
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .status()
            .expect("failed to start ELF fixture builder");
        assert!(status.success(), "ELF fixture builder failed: {status}");
    });
    fixture_dir()
}

#[test]
fn generated_wire_sizes_match_elf_abi() {
    assert_eq!(<Identifier as Bytes<Origin, Origin>>::BYTES_SIZE, 16);
    assert_eq!(<Header32 as Bytes<Origin, Origin>>::BYTES_SIZE, 52);
    assert_eq!(<Header64 as Bytes<Origin, Origin>>::BYTES_SIZE, 64);
    assert_eq!(<ProgramHeader32 as Bytes<Origin, Origin>>::BYTES_SIZE, 32);
    assert_eq!(<ProgramHeader64 as Bytes<Origin, Origin>>::BYTES_SIZE, 56);

    assert_eq!(
        std::mem::size_of::<Identifier>(),
        <Identifier as Bytes<Origin, Origin>>::BYTES_SIZE
    );
    assert_eq!(
        std::mem::size_of::<Header64>(),
        <Header64 as Bytes<Origin, Origin>>::BYTES_SIZE
    );
    assert_eq!(
        std::mem::size_of::<ProgramHeader64>(),
        <ProgramHeader64 as Bytes<Origin, Origin>>::BYTES_SIZE
    );
}

#[test]
fn generated_types_decode_real_elf64_bytes() {
    let fixture = ensure_fixtures().join("static_exec");
    let bytes = fs::read(fixture).expect("failed to read static ELF fixture");

    let mut identifier_bytes = [0u8; 16];
    identifier_bytes.copy_from_slice(&bytes[..16]);
    let identifier =
        <Identifier as Bytes<Origin, Origin>>::from_bytes_pointer(identifier_bytes.as_ptr(), true);
    assert!(identifier.is_magical());
    assert_eq!(identifier.class.0, 2);
    assert_eq!(identifier.data.0, 1);
    assert_eq!(identifier.version.0, 1);

    let mut header_bytes = [0u8; 64];
    header_bytes.copy_from_slice(&bytes[..64]);
    let header =
        <Header64 as Bytes<Origin, Origin>>::from_bytes_pointer(header_bytes.as_ptr(), true);
    assert_eq!(header.e_type.0, 2);
    assert_eq!(header.e_machine.0, 62);
    assert_eq!(header.e_entry.0, 0x400000);
    assert_eq!(header.e_phoff.0, 64);
    assert_eq!(header.e_ehsize.0, 64);
    assert_eq!(header.e_phentsize.0, 56);
    assert_eq!(header.e_phnum.0, 5);

    let mut program_header_bytes = [0u8; 56];
    program_header_bytes.copy_from_slice(&bytes[64..120]);
    let program_header = <ProgramHeader64 as Bytes<Origin, Origin>>::from_bytes_pointer(
        program_header_bytes.as_ptr(),
        true,
    );
    assert_eq!(program_header.p_type.0, 1);
    assert_eq!(program_header.p_flags.0, 4);
    assert_eq!(program_header.p_offset.0, 0);
    assert_eq!(program_header.p_vaddr.0, 0x3ff000);
    assert_eq!(program_header.p_filesz.0, 0x158);
    assert_eq!(program_header.p_memsz.0, 0x158);
    assert_eq!(program_header.p_align.0, 0x1000);
}

#[test]
fn declared_enum_values_match_rust_discriminants() {
    use userspace::file::format::elf::segment::header::flag::Flag as SegmentFlag;
    use userspace::file::format::elf::segment::header::r#type::Flag as SegmentType;
    use userspace::target::os::syscall::mmap::{Flag as MapFlag, Prot};

    assert_eq!(SegmentFlag::R as u32, 4);
    assert_eq!(SegmentFlag::RWX as u32, 7);
    assert_eq!(SegmentType::Load as u32, 1);
    assert_eq!(MapFlag::Private as usize, 2);
    assert_eq!(Prot::Read as usize, 1);
}

#[test]
fn loader_handles_static_pie_dynamic_and_collision_fixtures() {
    let directory = ensure_fixtures();

    let static_image = userspace::file::format::elf::segment::load_path(
        directory.join("static_exec").to_str().unwrap(),
    )
    .expect("static ET_EXEC fixture should load");
    assert!(static_image.direct_entry);
    assert_eq!(static_image.base, 0);
    assert_eq!(static_image.entry, 0x400000);

    let pie_image = userspace::file::format::elf::segment::load_path(
        directory.join("pie_minimal").to_str().unwrap(),
    )
    .expect("static ET_DYN fixture should load");
    assert!(pie_image.direct_entry);
    assert_ne!(pie_image.base, 0);

    let dynamic_image = userspace::file::format::elf::segment::load_path(
        directory.join("dynamic_pie").to_str().unwrap(),
    )
    .expect("dynamic PIE fixture should load");
    assert!(!dynamic_image.direct_entry);
    assert!(dynamic_image.interpreter.is_some());
    assert!(dynamic_image.dynamic);

    let collision_image = userspace::file::format::elf::segment::load_path(
        directory.join("collision_100000").to_str().unwrap(),
    )
    .expect("collision fixture should load outside the userspace link range");
    assert_eq!(collision_image.entry, 0x100000);
}

#[test]
fn auxv_unknown_entries_keep_their_raw_key_and_value() {
    use userspace::memory::stack::auxiliary::Entry;
    use userspace::memory::stack::auxiliary::atype::{Type, TypeTrait};
    use userspace::target::arch::Pointer;

    let pair = [0x1234usize, 0xfeed_face_cafe_beefusize];
    let entry = Entry::from_pointer(Pointer(pair.as_ptr() as *const u64));
    assert_eq!(entry.raw_key(), pair[0]);
    assert_eq!(entry.raw_value(), pair[1]);

    let decoded = unsafe { Type::from_pair(pair.as_ptr(), pair.as_ptr().add(1) as *const u8) };
    match decoded {
        Type::Unknown(value) => assert_eq!(value, pair[1]),
        _ => panic!("unknown auxv key was not preserved"),
    }
}

#[test]
fn stack_rebuild_synthesizes_auxv_and_refreshes_random() {
    use userspace::memory::stack::auxiliary::atype::constants::{
        AT_BASE, AT_ENTRY, AT_EXECFN, AT_NULL, AT_PHDR, AT_PHENT, AT_PHNUM, AT_PLATFORM, AT_RANDOM,
    };

    let argv0 = b"launcher\0";
    let environment = b"TEST=value\0";
    let platform = b"x86_64\0";
    let mut initial_stack = vec![
        1usize,
        argv0.as_ptr() as usize,
        0,
        environment.as_ptr() as usize,
        0,
        AT_PLATFORM,
        platform.as_ptr() as usize,
        AT_NULL,
        0,
    ];

    let rebuilt = userspace::memory::stack::build::build_initial_stack(
        initial_stack.as_mut_ptr() as *const u64,
        "/target",
        core::ptr::null(),
        0x400000,
        0x400040,
        56,
        5,
        0x70000000,
    )
    .expect("stack rebuild should succeed");

    let rebuilt = rebuilt as *const usize;
    let auxv = unsafe { rebuilt.add(5) };
    let mut found_phdr = false;
    let mut found_phent = false;
    let mut found_phnum = false;
    let mut found_base = false;
    let mut found_entry = false;
    let mut found_execfn = false;
    let mut found_random = false;

    for index in 0..32 {
        let key = unsafe { *auxv.add(index * 2) };
        let value = unsafe { *auxv.add(index * 2 + 1) };
        match key {
            AT_PHDR => {
                found_phdr = value == 0x400040;
            }
            AT_PHENT => {
                found_phent = value == 56;
            }
            AT_PHNUM => {
                found_phnum = value == 5;
            }
            AT_BASE => {
                found_base = value == 0x70000000;
            }
            AT_ENTRY => {
                found_entry = value == 0x400000;
            }
            AT_EXECFN => {
                let path = unsafe { std::ffi::CStr::from_ptr(value as *const i8) };
                found_execfn = path.to_bytes() == b"/target";
            }
            AT_RANDOM => {
                let random = unsafe { std::slice::from_raw_parts(value as *const u8, 16) };
                found_random = value != 0 && random.iter().any(|byte| *byte != 0);
            }
            AT_NULL => break,
            _ => {}
        }
    }

    assert!(found_phdr);
    assert!(found_phent);
    assert!(found_phnum);
    assert!(found_base);
    assert!(found_entry);
    assert!(found_execfn);
    assert!(found_random);
}
