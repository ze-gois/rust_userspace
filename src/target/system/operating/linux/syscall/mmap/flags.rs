ample::flags!(
    i32;
    "Memory mapping flags";
    pub struct Flags {
        [0x01;     SHARED;                         MAP_SHARED;          "MAP_SHARED";          "Share updates with other mappings"],
        [0x02;     PRIVATE;                        MAP_PRIVATE;         "MAP_PRIVATE";         "Create a private copy-on-write mapping"],
        [0x03;     SHARED_VALIDATE;                MAP_SHARED_VALIDATE;"MAP_SHARED_VALIDATE"; "Share updates and validate unknown flags"],
        [0x10;     FIXED;                          MAP_FIXED;           "MAP_FIXED";           "Place the mapping at the requested address"],
        [0x20;     ANONYMOUS;                      MAP_ANONYMOUS;       "MAP_ANONYMOUS";       "Create a mapping not backed by a file"],
        [0x0100;   GROWS_DOWN;                     MAP_GROWSDOWN;       "MAP_GROWSDOWN";       "Mapping may grow downward"],
        [0x0800;   DENY_WRITE;                     MAP_DENYWRITE;       "MAP_DENYWRITE";       "Legacy deny-write flag"],
        [0x1000;   EXECUTABLE;                     MAP_EXECUTABLE;      "MAP_EXECUTABLE";      "Legacy executable-file flag"],
        [0x2000;   LOCKED;                         MAP_LOCKED;          "MAP_LOCKED";          "Request locked pages"],
        [0x4000;   NO_RESERVE;                     MAP_NORESERVE;       "MAP_NORESERVE";       "Do not reserve swap space"],
        [0x8000;   POPULATE;                       MAP_POPULATE;        "MAP_POPULATE";        "Populate page tables"],
        [0x10000;  NON_BLOCKING;                   MAP_NONBLOCK;        "MAP_NONBLOCK";        "Do not block while populating"],
        [0x20000;  STACK;                          MAP_STACK;           "MAP_STACK";           "Mapping intended for a stack"],
        [0x40000;  HUGE_TRANSLATION_LOOKASIDE_BUFFER; MAP_HUGETLB;      "MAP_HUGETLB";         "Use huge pages"],
        [0x80000;  SYNCHRONIZED;                   MAP_SYNC;            "MAP_SYNC";            "Synchronous mapping semantics"],
        [0x100000; FIXED_WITHOUT_REPLACEMENT;      MAP_FIXED_NOREPLACE;"MAP_FIXED_NOREPLACE"; "Do not replace an existing mapping"]
    }
);
