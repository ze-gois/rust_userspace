// use crate::dtype::ELFType;
// use crate::dtype::UChar as T;

ample::enum_labeled_typed!(
    pub enum OsABI,
    u8,
    "Operating System Application Binary Interface",
    [
        // [0;   None;       NONE;       "NONE";       "UNIX System V ABI"],
        // [3;   Linux;      LINUX;      "LINUX";      "Compatibility alias"],
        [0;   Sysv;       super::T; SYSV;       "SYSV";       "Alias"],
        [1;   Hpux;       super::T; HPUX;       "HPUX";       "HP-UX"],
        [2;   NetBSD;     super::T; NETBSD;     "NETBSD";     "NetBSD"],
        [3;   Gnu;        super::T; GNU;        "GNU";        "Object uses GNU ELF extensions"],
        [6;   Solaris;    super::T; SOLARIS;    "SOLARIS";    "Sun Solaris"],
        [7;   Aix;        super::T; AIX;        "AIX";        "IBM AIX"],
        [8;   Irix;       super::T; IRIX;       "IRIX";       "SGI Irix"],
        [9;   FreeBSD;    super::T; FREEBSD;    "FREEBSD";    "FreeBSD"],
        [10;  Tru64;      super::T; TRU64;      "TRU64";      "Compaq TRU64 UNIX"],
        [11;  Modesto;    super::T; MODESTO;    "MODESTO";    "Novell Modesto"],
        [12;  OpenBSD;    super::T; OPENBSD;    "OPENBSD";    "OpenBSD"],
        [64;  Armaeabi;   super::T; ARMAEABI;   "ARMAEABI";   "ARM EABI"],
        [97;  Arm;        super::T; ARM;        "ARM";        "ARM"],
        [98;  Undefined;  super::T; UNDEFINED;  "UNDEFINED";  "NotSpecificed"],
        [255; Standalone; super::T; STANDALONE; "STANDALONE"; "Standalone (embedded) application"],
    ]
);
