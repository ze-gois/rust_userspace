result!(
    Ok;
    "Human Ok";
    usize;
    [
        [1; USERSPACE_FILE_FORMAT_ELF_DEFAULT_OK; Default; usize; "ZE"; "Entry to ze"],
        [2; USERSPACE_FILE_FORMAT_ELF_DTYPE_OK; DType; crate::file::format::elf::dtype::Ok; "ZE"; "Entry to ze"],
        [3; USERSPACE_FILE_FORMAT_ELF_HEADER_OK; Header; crate::file::format::elf::header::Ok; "ZE"; "Entry to ze"],
    ];
    Error;
    "Human error";
    usize;
    [
        [1; USERSPACE_FILE_FORMAT_ELF_DEFAULT_ERROR; Default; usize; "ZE"; "Entry to ze"],
        [2; USERSPACE_FILE_FORMAT_ELF_DTYPE_ERROR; DType; crate::file::format::elf::dtype::Error; "ZE"; "Entry to ze"],
        [3; USERSPACE_FILE_FORMAT_ELF_HEADER_ERROR; Header; crate::file::format::elf::header::Error; "ZE"; "Entry to ze"],
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
