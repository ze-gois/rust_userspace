// use super::ELFType;

crate::publish_types!(class_64, Class64,[
    [pub Null, usize],
    [pub UChar, u8],
    [pub SXWord, i64],
    [pub Half, u16],
    [pub SWord, i32],
    [pub XWord, u64],
    [pub Word, u32],
    [pub Off, u64],
    [pub Addr, u64],
]);

ample::result!(
    Ok;
    "Human Ok";
    usize;
    [
        [0; NULL_OK;       Null;    Null;  "UChar_64";  "UChar_64"],
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
        [0; NULL_ERROR;       Null;    Null;     "UChar_64";  "UChar_64"],
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
