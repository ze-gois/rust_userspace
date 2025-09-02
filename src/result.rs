ample::result!(
    Ok;
    "Human Ok";
    usize;
    [
        [1; USERSPACE_DEFAULT_OK; Default; usize; "ZE"; "Entry to ze"],
        [2; USERSPACE_TARGET_OK; Target; crate::target::Ok; "ZE"; "Entry to ze"],
        [3; USERSPACE_FILE_OK; File; crate::file::Ok; "ZE"; "Entry to ze"],
        [99; STDOUT; StdoutOk; usize; "ZE"; "Entry to ze"],
    ];
    Error;
    "Human error";
    usize;
    [
        [1; ERROR; Error; usize; "ZE"; "Entry to ze"],
        [2; USERSPACE_TARGET_ERROR; Target; crate::target::Error; "ZE"; "Entry to ze"],
        [3; USERSPACE_FILE_ERROR; File; crate::file::Error; "ZE"; "Entry to ze"],
        [99; STDOUT_ERRPR; Stdout; usize; "ZE"; "Entry to ze"],
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

// // use result::ErrorTrait;

// pub mod error {
//     define_error!("ELF", []);
// }

// // define_error_nested! (
// //     "ELF",
// //     [
// //         [0; Elf;     self::error;     ERR_ELF;     "Local error";   "ERR_ELF"],
// //         // [1; DType;   crate::dtype;    ERR_DTYPE;   "Datatype erro"; "ERR_DTYPE"],
// //         [2; Human;   human::result;   ERR_HUMAN;   "Human error";   "ERR_HUMAN"],
// //         [3; Syscall; syscall::result; ERR_SYSCALL; "Syscall error"; "ERR_SYSCALL"]
// //     ]
// // );

// pub mod macro_types {
//     #[allow(non_camel_case_types)]
//     pub type Mi8 = *const i8;
//     pub type elf_error = super::error::Error;
//     pub type dtype_error = crate::dtype::Error;
// }

// mod ok {
//     enum_typed!(Ok; usize; "AT_TYPE"; crate::result::macro_types; [
//         [0;    Null;            usize;  |p: Franco| { *p as usize };     AT_NULL;          "Null";          "End of vector"],
//         [1;    Ignore;          usize;  |p: Franco| { *p as usize };     AT_IGNORE;        "Ignore";        "Entry should be ignored"],
//     ]);
// }

// error_typed!("ELF Type"; crate::result::macro_types; [
//    [0; Elf;     elf_error; p; { elf_error::from_ptr(p) };  { p.to_no() as Franco };                ERR_ELF; "ERR_ELF";   "ERR_ELF"],
//    [1; DType; dtype_error; p; { dtype_error::from_ptr(p)}; { p as *const dtype_error as Franco}; ERR_DTYPE; "ERR_DTYPE"; "ERR_DTYPE"],
// ]);

// pub type Result = core::result::Result<ok::Ok, Error>;
