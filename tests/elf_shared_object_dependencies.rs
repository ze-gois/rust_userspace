use userspace::file::format::elf::{
    shared_object_dependencies::{
        OriginSubstitutionError, SearchDirectory, SearchPath, SharedObjectDependency,
    },
    ObjectFile,
};

const BASE: u64 = 0x400000;
const HEADER_SIZE: u64 = 64;
const PROGRAM_HEADER_SIZE: u64 = 56;
const PROGRAM_HEADER_COUNT: u64 = 2;
const DYNAMIC_OFFSET: u64 = HEADER_SIZE + PROGRAM_HEADER_SIZE * PROGRAM_HEADER_COUNT;
const DYNAMIC_SIZE: u64 = 8 * 16;
const STRING_OFFSET: u64 = DYNAMIC_OFFSET + DYNAMIC_SIZE;
const STRING_SIZE: u64 = 30;
const TOTAL_SIZE: u64 = STRING_OFFSET + STRING_SIZE;

fn half(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn word(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn sxword(bytes: &mut Vec<u8>, value: i64) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn xword(bytes: &mut Vec<u8>, value: u64) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn dynamic_entry(bytes: &mut Vec<u8>, tag: i64, payload: u64) {
    sxword(bytes, tag);
    xword(bytes, payload);
}

fn dynamic_entry_offset(index: usize) -> usize {
    DYNAMIC_OFFSET as usize + index * 16
}

fn fixture() -> Vec<u8> {
    let mut bytes = Vec::with_capacity(TOTAL_SIZE as usize);

    bytes.extend_from_slice(&[
        0x7f, b'E', b'L', b'F',
        2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);
    half(&mut bytes, 3);
    half(&mut bytes, 0x3e);
    word(&mut bytes, 1);
    xword(&mut bytes, 0);
    xword(&mut bytes, HEADER_SIZE);
    xword(&mut bytes, 0);
    word(&mut bytes, 0);
    half(&mut bytes, HEADER_SIZE as u16);
    half(&mut bytes, PROGRAM_HEADER_SIZE as u16);
    half(&mut bytes, PROGRAM_HEADER_COUNT as u16);
    half(&mut bytes, 64);
    half(&mut bytes, 0);
    half(&mut bytes, 0);

    word(&mut bytes, 1);
    word(&mut bytes, 4);
    xword(&mut bytes, 0);
    xword(&mut bytes, BASE);
    xword(&mut bytes, BASE);
    xword(&mut bytes, TOTAL_SIZE);
    xword(&mut bytes, TOTAL_SIZE);
    xword(&mut bytes, 0x1000);

    word(&mut bytes, 2);
    word(&mut bytes, 4);
    xword(&mut bytes, DYNAMIC_OFFSET);
    xword(&mut bytes, BASE + DYNAMIC_OFFSET);
    xword(&mut bytes, BASE + DYNAMIC_OFFSET);
    xword(&mut bytes, DYNAMIC_SIZE);
    xword(&mut bytes, DYNAMIC_SIZE);
    xword(&mut bytes, 8);

    dynamic_entry(&mut bytes, 5, BASE + STRING_OFFSET);
    dynamic_entry(&mut bytes, 10, STRING_SIZE);
    dynamic_entry(&mut bytes, 1, 1);
    dynamic_entry(&mut bytes, 14, 17);
    dynamic_entry(&mut bytes, 1, 9);
    dynamic_entry(&mut bytes, 29, 25);
    dynamic_entry(&mut bytes, 1, 1);
    dynamic_entry(&mut bytes, 0, 0);

    bytes.extend_from_slice(b"\0liba.so\0libb.so\0self.so\0/lib\0");

    assert_eq!(bytes.len(), TOTAL_SIZE as usize);
    bytes
}

#[test]
fn preserves_needed_shared_object_order_and_dynamic_entry_indices() {
    let bytes = fixture();
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    let dependencies = object
        .shared_object_dependencies_from_program_header(1)
        .expect("shared object dependencies must resolve");

    assert_eq!(dependencies.needed.len(), 3);

    assert_eq!(dependencies.needed[0].dynamic_entry_index, 2);
    assert_eq!(dependencies.needed[0].name, "liba.so");

    assert_eq!(dependencies.needed[1].dynamic_entry_index, 4);
    assert_eq!(dependencies.needed[1].name, "libb.so");

    assert_eq!(dependencies.needed[2].dynamic_entry_index, 6);
    assert_eq!(dependencies.needed[2].name, "liba.so");
}

#[test]
fn keeps_shared_object_identity_and_search_metadata_separate_from_needed_dependencies() {
    let bytes = fixture();
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    let dependencies = object
        .shared_object_dependencies_from_program_header(1)
        .expect("shared object dependencies must resolve");

    assert_eq!(dependencies.shared_object_name, Some("self.so"));
    assert_eq!(dependencies.runtime_search_path, None);
    assert_eq!(dependencies.run_path, Some("/lib"));
    assert_eq!(dependencies.search_path(), Some(SearchPath::RunPath("/lib")));
}

#[test]
fn ignores_runtime_search_path_in_shared_object() {
    let mut bytes = fixture();
    let tag = dynamic_entry_offset(5);
    bytes[tag..tag + 8].copy_from_slice(&15i64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    let dependencies = object
        .shared_object_dependencies_from_program_header(1)
        .expect("shared object dependencies must resolve");

    assert_eq!(dependencies.shared_object_name, Some("self.so"));
    assert_eq!(dependencies.runtime_search_path, None);
    assert_eq!(dependencies.run_path, None);
    assert_eq!(dependencies.search_path(), None);
}

#[test]
fn ignores_shared_object_name_and_uses_runtime_search_path_in_executable() {
    let mut bytes = fixture();
    bytes[16..18].copy_from_slice(&2u16.to_le_bytes());

    let tag = dynamic_entry_offset(5);
    bytes[tag..tag + 8].copy_from_slice(&15i64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    let dependencies = object
        .shared_object_dependencies_from_program_header(1)
        .expect("shared object dependencies must resolve");

    assert_eq!(dependencies.shared_object_name, None);
    assert_eq!(dependencies.runtime_search_path, Some("/lib"));
    assert_eq!(dependencies.run_path, None);
    assert_eq!(
        dependencies.search_path(),
        Some(SearchPath::RuntimeSearchPath("/lib")),
    );
}

#[test]
fn run_path_takes_precedence_over_runtime_search_path() {
    let mut bytes = fixture();
    bytes[16..18].copy_from_slice(&2u16.to_le_bytes());

    let rpath = dynamic_entry_offset(3);
    bytes[rpath..rpath + 8].copy_from_slice(&15i64.to_le_bytes());
    bytes[rpath + 8..rpath + 16].copy_from_slice(&1u64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    let dependencies = object
        .shared_object_dependencies_from_program_header(1)
        .expect("shared object dependencies must resolve");

    assert_eq!(dependencies.runtime_search_path, Some("liba.so"));
    assert_eq!(dependencies.run_path, Some("/lib"));
    assert_eq!(dependencies.search_path(), Some(SearchPath::RunPath("/lib")));
}

#[test]
fn distinguishes_direct_dependency_pathnames_from_names_that_require_search() {
    let name = SharedObjectDependency::new(2, "liba.so");
    assert_eq!(name.direct_pathname("/origin"), Ok(None));
    assert_eq!(name.requires_search("/origin"), Ok(true));

    let absolute_pathname = SharedObjectDependency::new(3, "/usr/lib/liba.so");
    assert_eq!(
        absolute_pathname.direct_pathname("/origin"),
        Ok(Some(String::from("/usr/lib/liba.so"))),
    );
    assert_eq!(absolute_pathname.requires_search("/origin"), Ok(false));

    let relative_pathname = SharedObjectDependency::new(4, "directory/liba.so");
    assert_eq!(
        relative_pathname.direct_pathname("/origin"),
        Ok(Some(String::from("directory/liba.so"))),
    );
    assert_eq!(relative_pathname.requires_search("/origin"), Ok(false));
}

#[test]
fn resolves_dependency_search_path_directories_in_order() {
    let search_path = SearchPath::RunPath("/first::/third:");

    assert_eq!(
        search_path.directories(),
        vec![
            SearchDirectory::Pathname("/first"),
            SearchDirectory::CurrentDirectory,
            SearchDirectory::Pathname("/third"),
            SearchDirectory::CurrentDirectory,
        ],
    );
}

#[test]
fn empty_dependency_search_path_means_current_directory() {
    let search_path = SearchPath::RuntimeSearchPath("");

    assert_eq!(
        search_path.directories(),
        vec![SearchDirectory::CurrentDirectory],
    );
}

#[test]
fn substitutes_origin_in_needed_dependency_name() {
    let dependency = SharedObjectDependency::new(2, "$ORIGIN/liba.so");
    assert_eq!(
        dependency.name_with_origin("/opt/application/lib"),
        Ok(String::from("/opt/application/lib/liba.so")),
    );

    let braced = SharedObjectDependency::new(3, "${ORIGIN}/libb.so");
    assert_eq!(
        braced.name_with_origin("/opt/application/lib"),
        Ok(String::from("/opt/application/lib/libb.so")),
    );
}

#[test]
fn substitutes_origin_in_run_path_but_not_deprecated_runtime_search_path() {
    let run_path = SearchPath::RunPath("$ORIGIN/lib:/usr/lib");
    assert_eq!(
        run_path.value_with_origin("/opt/application"),
        Ok(String::from("/opt/application/lib:/usr/lib")),
    );

    let runtime_search_path = SearchPath::RuntimeSearchPath("$ORIGIN/lib:/usr/lib");
    assert_eq!(
        runtime_search_path.value_with_origin("/opt/application"),
        Ok(String::from("$ORIGIN/lib:/usr/lib")),
    );
}

#[test]
fn reports_unspecified_dynamic_string_substitution_sequence() {
    let dependency = SharedObjectDependency::new(2, "$LIB/liba.so");

    assert_eq!(
        dependency.name_with_origin("/opt/application"),
        Err(OriginSubstitutionError::UnspecifiedSequence { byte_offset: 0 }),
    );

    let malformed = SharedObjectDependency::new(3, "lib/${ORIGIN");
    assert_eq!(
        malformed.name_with_origin("/opt/application"),
        Err(OriginSubstitutionError::UnspecifiedSequence { byte_offset: 4 }),
    );
}

#[test]
fn interprets_origin_substitution_before_needed_pathname_classification() {
    let dependency = SharedObjectDependency::new(5, "$ORIGIN/liba.so");

    assert_eq!(
        dependency.direct_pathname("/opt/application"),
        Ok(Some(String::from("/opt/application/liba.so"))),
    );
    assert_eq!(dependency.requires_search("/opt/application"), Ok(false));
}
