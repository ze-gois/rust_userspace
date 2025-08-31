enum_labeled!(
    pub Index,
    u8,
    "Identifier byte-byte identification",
[
    [0; Mag0;         (); MAG0;         "Mag0";        "File identification 1st magic byte'"],
    [1; Mag1;         (); MAG1;         "Mag1";        "File identification 2nd magic byte'"],
    [2; Mag2;         (); MAG2;         "Mag2";        "File identification 3rd magic byte'"],
    [3; Mag3;         (); MAG3;         "Mag3";        "File identification 4th magic byte'"],
    [4; Class;        (); CLASS;        "Class";       "File class byte'"],
    [5; Data;         (); DATA;         "Data";        "Data encoding byte'"],
    [6; Version;      (); VERSION;      "Version";     "File version byte'"],
    [7; OsAbi;        (); OSABI;        "OsAbi";       "OS/ABI identification byte'"],
    [8; AbiVersion;   (); ABIVERSION;   "AbiVersion";  "ABI version byte'"],
    [9; Pad;          (); PAD;          "Pad";         "Start of padding bytes byte'"],
    [10; Unassigned1; (); UNASSIGNED1;  "Unassigned1"; "Unassigned 10th byte'"],
    [11; Unassigned2; (); UNASSIGNED2;  "Unassigned2"; "Unassigned 11th byte'"],
    [12; Unassigned3; (); UNASSIGNED3;  "Unassigned3"; "Unassigned 12th byte'"],
    [13; Unassigned4; (); UNASSIGNED4;  "Unassigned4"; "Unassigned 13th byte'"],
    [14; Unassigned5; (); UNASSIGNED5;  "Unassigned5"; "Unassigned 14th byte'"],
    [15; NIdent;      (); NIDENT;       "NIdent";      "Size of ELF identifier array byte'"]
]);

pub trait IndexTrait {}
