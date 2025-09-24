ample::enum_labeled!(
    pub enum Index,
    u8,
    "Identifier byte-byte identification",
[
    [0; Magic0;       super::Magic;   MAG0;         "Mag0";        "File identification 1st magic byte'"],
    [1; Magic1;       super::Magic;   MAG1;         "Mag1";        "File identification 2nd magic byte'"],
    [2; Magic2;       super::Magic;   MAG2;         "Mag2";        "File identification 3rd magic byte'"],
    [3; Magic3;       super::Magic;   MAG3;         "Mag3";        "File identification 4th magic byte'"],
    [4; Class;        super::Class;   CLASS;        "Class";       "File class byte'"],
    [5; Data;         super::Data;    DATA;         "Data";        "Data encoding byte'"],
    [6; Version;      super::T;       VERSION;      "Version";     "File version byte'"],
    [7; OsAbi;        super::OsABI;   OSABI;        "OsAbi";       "OS/ABI identification byte'"],
    [8; AbiVersion;   super::T;       ABIVERSION;   "AbiVersion";  "ABI version byte'"],
    [9; Pad;          super::T;       PAD;          "Pad";         "Start of padding bytes byte'"],
    [10; Unassigned1; super::T;       UNASSIGNED1;  "Unassigned1"; "Unassigned 10th byte'"],
    [11; Unassigned2; super::T;       UNASSIGNED2;  "Unassigned2"; "Unassigned 11th byte'"],
    [12; Unassigned3; super::T;       UNASSIGNED3;  "Unassigned3"; "Unassigned 12th byte'"],
    [13; Unassigned4; super::T;       UNASSIGNED4;  "Unassigned4"; "Unassigned 13th byte'"],
    [14; Unassigned5; super::T;       UNASSIGNED5;  "Unassigned5"; "Unassigned 14th byte'"],
    [16; NIdent;      super::T;       NIDENT;       "NIdent";      "Size of ELF identifier array byte'"]
]);

pub trait IndexTrait {}
