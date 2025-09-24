// use super::ELFType;

crate::file_format_elf_dtype_class!(
    class_32,
    Class32,
    [
        [pub Addr,  u32, "Unsigned program address"],
        [pub Off,   u32, "Unsigned file offset"],
        [pub Half,  u16, "Unsigned medium integer"],
        [pub Word,  u32, "Unsigned integer"],
        [pub SWord, i32, "Signed integer"],
        [pub UChar,  u8, "Unsigned small integer"],
    ]
);

ample::result!(
    Ok;
    "Human Ok";
    usize;
    [
        [1; UCHAR_32_OK;   UChar;   UChar;     "UChar_32";  "UChar_32"],
        [3; HALF_32_OK;    Half;    Half;     "Half_32";   "Half_32"],
        [4; SWORD_32_OK;   SWord;   SWord;    "SWord_32";  "SWord_32"],
        [6; WORD_32_OK;    Word;    Word;     "Word_32";   "Word_32"],
        [7; OFF_32_OK;     Off;     Off;      "Off_32";    "Off_32"],
        [8; ADDR_32_OK;    Addr;    Addr;     "Addr_32";   "Addr_32"]
    ];
    Error;
    "Human error";
    usize;
    [
        [1; UCHAR_32_ERROR;   UChar;   UChar;     "UChar_32";  "UChar_32"],
        [3; HALF_32_ERROR;    Half;    Half;     "Half_32";   "Half_32"],
        [4; SWORD_32_ERROR;   SWord;   SWord;    "SWord_32";  "SWord_32"],
        [6; WORD_32_ERROR;    Word;    Word;     "Word_32";   "Word_32"],
        [7; OFF_32_ERROR;     Off;     Off;      "Off_32";    "Off_32"],
        [8; ADDR_32_ERROR;    Addr;    Addr;     "Addr_32";   "Addr_32"]
    ]
);

impl Ok {
    pub fn from_no(_no: usize) -> Self {
        Ok::Off(Off(0))
    }
}

impl Error {
    pub fn from_no(_no: usize) -> Self {
        Error::Off(Off(0))
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
