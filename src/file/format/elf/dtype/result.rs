result!(
    Ok;
    "Human Ok";
    usize;
    [
        [1; USERSPACE_DEFAULT_OK; Default; usize; "ZE"; "Entry to ze"],
        [2; USERSPACE_TARGET_OK; Target; crate::target::Ok; "ZE"; "Entry to ze"],
        [3; STDOUT; StdoutOk; usize; "ZE"; "Entry to ze"],
    ];
    Error;
    "Human error";
    usize;
    [
        [1; ERROR; Error; usize; "ZE"; "Entry to ze"],
        [2; USERSPACE_TARGET_ERR; Target; crate::target::Error; "ZE"; "Entry to ze"],
    ]
);

impl Ok {
    pub fn from_no(no: usize) -> Self {
        Ok::Default(no)
    }
}

impl Error {
    pub fn from_no(no: usize) -> Self {
        Error::Error(no)
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
