result!(
    Ok;
    "Human Ok";
    usize;
    [
        [1; USERSPACE_FILE_DEFAULT_OK; Default; usize; "ZE"; "Entry to ze"],
        [2; USERSPACE_FILE_FORMAT_OK; Format; crate::file::format::Ok; "ZE"; "Entry to ze"],
    ];
    Error;
    "Human error";
    usize;
    [
        [1; USERSPACE_FILE_DEFAULT_ERROR; Default; usize; "ZE"; "Entry to ze"],
        [2; USERSPACE_FILE_FORMAT_ERROR; Format; crate::file::format::Error; "ZE"; "Entry to ze"],
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
