ample::result!(
    Ok;
    "Human Ok";
    ();
    [
        [1; USERSPACE_FILE_FORMAT_ELF_LOADER_OK; Default; usize; "ZE"; "Entry to ze"],
    ];
    Error;
    "Human error";
    usize;
    [
        [1; USERSPACE_FILE_FORMAT_ELF_LOADER_INVALID_HEADER; InvalidHeader; usize; "InvalidHeader"; "InvalidHeader"],
        [2; USERSPACE_FILE_FORMAT_ELF_LOADER_UNSUPPORTED_CLASS; UnsupportedClass; usize; "UnsupportedClass"; "UnsupportedClass"],
        [3; USERSPACE_FILE_FORMAT_ELF_LOADER_UNSUPPORTED_ENDIANNESS; UnsupportedEndianness; usize; "UnsupportedEndianness"; "UnsupportedEndianness"],
        [4; USERSPACE_FILE_FORMAT_ELF_LOADER_UNSUPPORTED_TYPE; UnsupportedType; usize; "UnsupportedType"; "UnsupportedType"],
        [5; USERSPACE_FILE_FORMAT_ELF_LOADER_UNSUPPORTED_MACHINE; UnsupportedMachine; usize; "UnsupportedMachine"; "UnsupportedMachine"],
        [6; USERSPACE_FILE_FORMAT_ELF_LOADER_INVALID_PROGRAM_HEADER_TABLE; InvalidProgramHeaderTable; usize; "InvalidProgramHeaderTable"; "InvalidProgramHeaderTable"],
        [7; USERSPACE_FILE_FORMAT_ELF_LOADER_INVALID_PROGRAM_HEADER; InvalidProgramHeader; usize; "InvalidProgramHeader"; "InvalidProgramHeader"],
        [8; USERSPACE_FILE_FORMAT_ELF_LOADER_UNSUPPORTED_INTERPRETER; UnsupportedInterpreter; usize; "UnsupportedInterpreter"; "UnsupportedInterpreter"],
        [9; USERSPACE_FILE_FORMAT_ELF_LOADER_UNSUPPORTED_DYNAMIC_LINKING; UnsupportedDynamicLinking; usize; "UnsupportedDynamicLinking"; "UnsupportedDynamicLinking"],
        [10; USERSPACE_FILE_FORMAT_ELF_LOADER_UNSUPPORTED_TLS; UnsupportedTls; usize; "UnsupportedTls"; "UnsupportedTls"],
        [11; USERSPACE_FILE_FORMAT_ELF_LOADER_NO_LOADABLE_SEGMENTS; NoLoadableSegments; usize; "NoLoadableSegments"; "NoLoadableSegments"],
        [12; USERSPACE_FILE_FORMAT_ELF_LOADER_ENTRY_OUTSIDE_EXECUTABLE_SEGMENT; EntryOutsideExecutableSegment; usize; "EntryOutsideExecutableSegment"; "EntryOutsideExecutableSegment"],
        [13; USERSPACE_FILE_FORMAT_ELF_LOADER_MAPPING_FAILED; MappingFailed; usize; "MappingFailed"; "MappingFailed"],
        [14; USERSPACE_FILE_FORMAT_ELF_LOADER_PROTECTION_FAILED; ProtectionFailed; usize; "ProtectionFailed"; "ProtectionFailed"],
        [15; USERSPACE_FILE_FORMAT_ELF_LOADER_FILE_READ_FAILED; FileReadFailed; usize; "FileReadFailed"; "FileReadFailed"],
        [16; USERSPACE_FILE_FORMAT_ELF_LOADER_FILE_OPEN_FAILED; FileOpenFailed; usize; "FileOpenFailed"; "FileOpenFailed"],
        [17; USERSPACE_FILE_FORMAT_ELF_LOADER_FILE_METADATA_FAILED; FileMetadataFailed; usize; "FileMetadataFailed"; "FileMetadataFailed"],
        [18; USERSPACE_FILE_FORMAT_ELF_LOADER_ADDRESS_OVERFLOW; AddressOverflow; usize; "AddressOverflow"; "AddressOverflow"],
        [19; USERSPACE_FILE_FORMAT_ELF_LOADER_INVALID_INTERPRETER; InvalidInterpreter; usize; "InvalidInterpreter"; "InvalidInterpreter"],
        [20; USERSPACE_FILE_FORMAT_ELF_LOADER_INTERPRETER_UNAVAILABLE; InterpreterUnavailable; usize; "InterpreterUnavailable"; "InterpreterUnavailable"],
        [21; USERSPACE_FILE_FORMAT_ELF_LOADER_STACK_CONSTRUCTION_FAILED; StackConstructionFailed; usize; "StackConstructionFailed"; "StackConstructionFailed"],
    ]
);

impl Ok {
    pub fn from_no(no: usize) -> Self {
        Ok::Default(no)
    }
}

impl Error {
    pub fn from_no(no: usize) -> Self {
        Error::Default(no)
    }
}

pub type Result = core::result::Result<Ok, Error>;

pub fn handle_result(result: usize) -> Result {
    if (result as isize) < 0 {
        Err(Error::from_no(result))
    } else {
        Ok(Ok::from_no(result))
    }
}
