// use super::ELFType;

crate::file_format_elf_dtype_class!(
    class_64,
    Class64,
    [
        [pub Addr,   u64, "Unsigned program address"],
        [pub Off,    u64, "Unsigned file offset"],
        [pub Half,   u16, "Unsigned medium integer"],
        [pub Word,   u32, "Unsigned integer"],
        [pub SWord,  i32, "Signed integer"],
        [pub XWord,  u64, "Unsigned long integer"],
        [pub SXWord, i64, "Signed long integer"],
        [pub UChar,  u8,  "Unsigned small integer"],
    ]
);

ample::result!(
    Ok;
    "Human Ok";
    usize;
    [

        [1; UCHAR_64_OK;   UChar;   UChar;     "UChar_64";  "UChar_64"],
        [2; SXWORD_64_OK;  SXWord;  SXWord;   "SXWord_64"; "SXWord_64"],
        [3; HALF_64_OK;    Half;    Half;     "Half_64";   "Half_64"],
        [4; SWORD_64_OK;   SWord;   SWord;    "SWord_64";  "SWord_64"],
        [5; XWORD_64_OK;   XWord;   XWord;    "XWord_64";  "XWord_64"],
        [6; WORD_64_OK;    Word;    Word;     "Word_64";   "Word_64"],
        [7; OFF_64_OK;     Off;     Off;      "Off_64";    "Off_64"],
        [8; ADDR_64_OK;    Addr;    Addr;     "Addr_64";   "Addr_64"]
    ];
    Error;
    "Human error";
    usize;
    [
        [1; UCHAR_64_ERROR;   UChar;   UChar;     "UChar_64";  "UChar_64"],
        [2; SXWORD_64_ERROR;  SXWord;  SXWord;   "SXWord_64"; "SXWord_64"],
        [3; HALF_64_ERROR;    Half;    Half;     "Half_64";   "Half_64"],
        [4; SWORD_64_ERROR;   SWord;   SWord;    "SWord_64";  "SWord_64"],
        [5; XWORD_64_ERROR;   XWord;   XWord;    "XWord_64";  "XWord_64"],
        [6; WORD_64_ERROR;    Word;    Word;     "Word_64";   "Word_64"],
        [7; OFF_64_ERROR;     Off;     Off;      "Off_64";    "Off_64"],
        [8; ADDR_64_ERROR;    Addr;    Addr;     "Addr_64";   "Addr_64"]
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
