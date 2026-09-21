ample::result!(
    Ok;
    "File Ok";
    usize;
    [
        [1; USERSPACE_FILE_DEFAULT_OK; Default; usize; "Ok"; "Default file result"],
    ];
    Error;
    "File Error";
    usize;
    [
        [1; USERSPACE_FILE_DEFAULT_ERROR; Default; usize; "Error"; "Default file error"],
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
