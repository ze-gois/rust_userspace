pub type Word = usize;

pub trait TypeTrait {
    fn from_pair(key: *const Word, value: *const Word) -> Self;
    fn is_null(&self) -> bool;
}

macro_rules! auxiliary_types {
    (
        [
            $(
                [
                    $discriminant:expr;
                    $variant:ident;
                    $payload:ty;
                    $constant:ident;
                    $acronym:expr;
                    $description:expr
                ]
            ),* $(,)?
        ]
    ) => {
        ample::enum_labeled_typed!(
            #[derive(Debug)]
            pub enum Type,
            usize,
            "AT_TYPE",
            [
                $(
                    [
                        $discriminant;
                        $variant;
                        $payload;
                        $constant;
                        $acronym;
                        $description
                    ]
                ),*,
                [
                    0xffff_ffff_ffff_ffff;
                    Unknown;
                    usize;
                    AT_UNKNOWN;
                    "Unknown";
                    "Unknown auxiliary-vector entry"
                ]
            ]
        );

        ample::enum_labeled_typed!(
            #[derive(Debug)]
            pub enum TypeUnit,
            usize,
            "AT_TYPE",
            [
                $(
                    [
                        $discriminant;
                        $variant;
                        ();
                        $constant;
                        $acronym;
                        $description
                    ]
                ),*,
                [
                    0xffff_ffff_ffff_ffff;
                    Unknown;
                    ();
                    AT_UNKNOWN;
                    "Unknown";
                    "Unknown auxiliary-vector entry"
                ]
            ]
        );

        impl TypeUnit {
            pub fn from_discriminant(discriminant: usize) -> Self {
                match discriminant {
                    $($discriminant => Self::$variant(()),)*
                    _ => Self::Unknown(()),
                }
            }
        }

        impl TypeTrait for Type {
            fn from_pair(key: *const Word, value: *const Word) -> Self {
                let discriminant = unsafe { *key };
                let raw_value = unsafe { *value };

                match TypeUnit::from_discriminant(discriminant) {
                    $(TypeUnit::$variant(()) => Self::$variant(raw_value as $payload),)*
                    TypeUnit::Unknown(()) => Self::Unknown(raw_value),
                }
            }

            fn is_null(&self) -> bool {
                matches!(self, Self::Null(0))
            }
        }
    };
}

auxiliary_types!([
    [0;  Null;            usize;      AT_NULL;              "Null";            "End of vector"],
    [1;  Ignore;          usize;      AT_IGNORE;            "Ignore";          "Entry should be ignored"],
    [2;  ExecFD;          i32;        AT_EXECFD;            "ExecFD";          "File descriptor of program"],
    [3;  PHdr;            usize;      AT_PHDR;              "PHdr";            "Program headers for program"],
    [4;  PHEnt;           u16;        AT_PHENT;             "PHEnt";           "Size of program header entry"],
    [5;  PHNum;           u16;        AT_PHNUM;             "PHNum";           "Number of program headers"],
    [6;  PageSz;          usize;      AT_PAGESZ;            "PageSz";          "System page size"],
    [7;  Base;            usize;      AT_BASE;              "Base";            "Base address of interpreter"],
    [8;  Flags;           u32;        AT_FLAGS;             "Flags";           "Flags"],
    [9;  Entry;           usize;      AT_ENTRY;             "Entry";           "Entry point of program"],
    [10; NotELF;          i32;        AT_NOTELF;            "NotELF";          "Program is not ELF"],
    [11; UID;             u32;        AT_UID;               "UID";             "Real uid"],
    [12; EUID;            u32;        AT_EUID;              "EUID";            "Effective uid"],
    [13; GID;             u32;        AT_GID;               "GID";             "Real gid"],
    [14; EGID;            u32;        AT_EGID;              "EGID";            "Effective gid"],
    [15; Platform;        *const i8;  AT_PLATFORM;          "Platform";        "String identifying CPU"],
    [16; HwCap;           u64;        AT_HWCAP;             "HwCap";           "Architecture capability hints"],
    [17; ClkTck;          usize;      AT_CLKTCK;            "ClkTck";          "Frequency at which times() increments"],
    [23; Secure;          i32;        AT_SECURE;            "Secure";          "Secure mode"],
    [24; BasePlatform;    *const i8;  AT_BASE_PLATFORM;     "BasePlatform";    "Real platform string"],
    [25; Random;          *const u8;  AT_RANDOM;            "Random";          "Address of 16 random bytes"],
    [26; HwCap2;          u64;        AT_HWCAP2;            "HwCap2";          "Extension of AT_HWCAP"],
    [27; RSeqFeatureSize; usize;      AT_RSEQ_FEATURE_SIZE; "RSeqFeatureSize"; "rseq supported feature size"],
    [28; RSeqAlign;       usize;      AT_RSEQ_ALIGN;        "RSeqAlign";       "rseq allocation alignment"],
    [29; HwCap3;          u64;        AT_HWCAP3;            "HwCap3";          "Extension of AT_HWCAP"],
    [30; HwCap4;          u64;        AT_HWCAP4;            "HwCap4";          "Extension of AT_HWCAP"],
    [31; ExecFn;          *const i8;  AT_EXECFN;            "ExecFn";          "Filename of program"],
    [32; SysInfo;         usize;      AT_SYSINFO;           "SysInfo";         "System info, x86 specific"],
    [33; SysInfoEhdr;     usize;      AT_SYSINFO_EHDR;      "SysInfoEhdr";     "System info ELF header"],
    [51; MinSigStackSz;   usize;      AT_MINSIGSTKSZ;       "MinSigStackSz";   "Minimum signal stack size"]
]);
