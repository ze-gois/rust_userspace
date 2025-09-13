// use super::ELFType;

crate::file_format_elf_dtype_class!(
    class_32,
    Class32,
    [
        [pub Null, usize],
        [pub UChar, u8],
        [pub SXWord, i32],
        [pub Half, u16],
        [pub SWord, i32],
        [pub XWord, u32],
        [pub Word, u32],
        [pub Off, u32],
        [pub Addr, u32],
    ]
);

ample::result!(
    Ok;
    "Human Ok";
    usize;
    [
        [0; NULL_OK;       Null;    Null;  "UChar_32";  "UChar_32"],
        [1; UCHAR_32_OK;   UChar;   UChar;     "UChar_32";  "UChar_32"],
        [2; SXWORD_32_OK;  SXWord;  SXWord;   "SXWord_32"; "SXWord_32"],
        [3; HALF_32_OK;    Half;    Half;     "Half_32";   "Half_32"],
        [4; SWORD_32_OK;   SWord;   SWord;    "SWord_32";  "SWord_32"],
        [5; XWORD_32_OK;   XWord;   XWord;    "XWord_32";  "XWord_32"],
        [6; WORD_32_OK;    Word;    Word;     "Word_32";   "Word_32"],
        [7; OFF_32_OK;     Off;     Off;      "Off_32";    "Off_32"],
        [8; ADDR_32_OK;    Addr;    Addr;     "Addr_32";   "Addr_32"]
    ];
    Error;
    "Human error";
    usize;
    [
        [0; NULL_ERROR;       Null;    Null;     "UChar_32";  "UChar_32"],
        [1; UCHAR_32_ERROR;   UChar;   UChar;     "UChar_32";  "UChar_32"],
        [2; SXWORD_32_ERROR;  SXWord;  SXWord;   "SXWord_32"; "SXWord_32"],
        [3; HALF_32_ERROR;    Half;    Half;     "Half_32";   "Half_32"],
        [4; SWORD_32_ERROR;   SWord;   SWord;    "SWord_32";  "SWord_32"],
        [5; XWORD_32_ERROR;   XWord;   XWord;    "XWord_32";  "XWord_32"],
        [6; WORD_32_ERROR;    Word;    Word;     "Word_32";   "Word_32"],
        [7; OFF_32_ERROR;     Off;     Off;      "Off_32";    "Off_32"],
        [8; ADDR_32_ERROR;    Addr;    Addr;     "Addr_32";   "Addr_32"]
    ]
);

impl Ok {
    pub fn from_no(no: usize) -> Self {
        Ok::Null(Null(no))
    }
}

impl Error {
    pub fn from_no(no: usize) -> Self {
        Error::Null(Null(no))
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
