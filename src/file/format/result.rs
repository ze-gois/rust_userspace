ample::result!(
    Ok;
    "Human Ok";
    usize;
    [
        [1; USERSPACE_FILE_FORMAT_DEFAULT_OK; Default; usize; "ZE"; "Entry to ze"],
        // [2; USERSPACE_FILE_FORMAT_ELF_OK; Elf; crate::file::format::elf::Ok; "ZE"; "Entry to ze"],
    ];
    Error;
    "Human error";
    usize;
    [
        [1; USERSPACE_FILE_FORMAT_DEFAULT_ERROR; Default; usize; "ZE"; "Entry to ze"],
        // [2; USERSPACE_FILE_FORMAT_ELF_ERROR; Elf; crate::file::format::elf::Error; "ZE"; "Entry to ze"],
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
