pub mod class_32;
pub mod class_64;
pub use class_64::UChar;

#[macro_use]
pub mod macros;
pub mod traits;
pub use traits::Trait;

pub mod result;
pub use result::{Error, Ok, Result};

// pub mod endianness;
// pub use endianness::Endianness;

// pub mod ok {
//     r#struct!(pub OurStruct {
//         value : usize,
//         inform : u8,
//     });

//     result!(
//         Ok;
//         "Human Ok";
//         usize;
//         [
//             [1; ZE_ENTRY; HumanOk; usize; "ZE"; "Entry to ze"],
//             [2; SYSCALL; SyscallOk; OurStruct; "ZE"; "Entry to ze"],
//             [3; STDOUT; StdoutOk; usize; "ZE"; "Entry to ze"],
//         ]
//     );

//     impl Ok {
//         pub fn from_no(no: usize) -> Self {
//             Ok::HumanOk(no)
//         }
//     }
// }

// pub mod error {
//     result!(
//         Error;
//         "Human error";
//         usize;
//         [
//             [0; ERR_NULL;       Null;    usize;     "UChar_64";  "UChar_64"],
//             [1; ERR_UCHAR_64;   UChar;   u8;     "UChar_64";  "UChar_64"],
//             [2; ERR_SXWORD_64;  SXWord;  i64;   "SXWord_64"; "SXWord_64"],
//             [3; ERR_HALF_64;    Half;    u16;     "Half_64";   "Half_64"],
//             [4; ERR_SWORD_64;   SWord;   i32;    "SWord_64";  "SWord_64"],
//             [5; ERR_XWORD_64;   XWord;   u64;    "XWord_64";  "XWord_64"],
//             [6; ERR_WORD_64;    Word;    u32;     "Word_64";   "Word_64"],
//             [7; ERR_OFF_64;     Off;     u64;      "Off_64";    "Off_64"],
//             [8; ERR_ADDR_64;    Addr;    u64;     "Addr_64";   "Addr_64"]
//         ]
//     );

//     impl Error {
//         pub fn from_no(no: usize) -> Self {
//             Error::Null(no)
//         }
//     }
// }

// pub use error::Error;
// pub use ok::Ok;

// pub type Result = core::result::Result<Ok, Error>;

// pub fn handle_result(result: usize) -> Result {
//     if (result as isize) < 0 {
//         Err(Error::from_no(result))
//     } else {
//         Ok(Ok::from_no(result))
//     }
// }
