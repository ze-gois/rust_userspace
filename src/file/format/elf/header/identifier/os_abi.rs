// use crate::dtype::ELFType;
// use crate::dtype::UChar as T;

ample::enum_labeled!(
    pub enum OsABI,
    u8,
    "Operating System Application Binary Interface",
    [
        // [0;   None;       NONE;       "NONE";       "UNIX System V ABI"],
        // [3;   Linux;      LINUX;      "LINUX";      "Compatibility alias"],
        [0;   Sysv;       (); SYSV;       "SYSV";       "Alias"],
        [1;   Hpux;       (); HPUX;       "HPUX";       "HP-UX"],
        [2;   NetBSD;     (); NETBSD;     "NETBSD";     "NetBSD"],
        [3;   Gnu;        (); GNU;        "GNU";        "Object uses GNU ELF extensions"],
        [6;   Solaris;    (); SOLARIS;    "SOLARIS";    "Sun Solaris"],
        [7;   Aix;        (); AIX;        "AIX";        "IBM AIX"],
        [8;   Irix;       (); IRIX;       "IRIX";       "SGI Irix"],
        [9;   FreeBSD;    (); FREEBSD;    "FREEBSD";    "FreeBSD"],
        [10;  Tru64;      (); TRU64;      "TRU64";      "Compaq TRU64 UNIX"],
        [11;  Modesto;    (); MODESTO;    "MODESTO";    "Novell Modesto"],
        [12;  OpenBSD;    (); OPENBSD;    "OPENBSD";    "OpenBSD"],
        [64;  Armaeabi;   (); ARMAEABI;   "ARMAEABI";   "ARM EABI"],
        [97;  Arm;        (); ARM;        "ARM";        "ARM"],
        [98;  Undefined;  (); UNDEFINED;  "UNDEFINED";  "NotSpecificed"],
        [255; Standalone; (); STANDALONE; "STANDALONE"; "Standalone (embedded) application"],
    ]
);
