// use crate::dtype::ELFType;
// use crate::dtype::UChar as T;

ample::enum_labeled!(
    pub enum OsABI,
    u8,
    "Operating System Application Binary Interface",
    [
        [0  ; NONE       ; ELFOSABI_NONE                 ; "NONE"       ; "No extensions or unspecified"],
        [1  ; HPUX       ; ELFOSABI_HPUX                 ; "HPUX"       ; "Hewlett-Packard HP-UX"],
        [2  ; NETBSD     ; ELFOSABI_NETBSD               ; "NETBSD"     ; "NetBSD"],
        // [3  ; GNU        ; ELFOSABI_GNU                  ; "GNU"        ; "GNU"],
        [3  ; LINUX      ; ELFOSABI_LINUX , ELFOSABI_GNU ; "LINUX"      ; "Linux (historical—alias for ELFOSABI_GNU)"],
        [6  ; SOLARIS    ; ELFOSABI_SOLARIS              ; "SOLARIS"    ; "Sun Solaris"],
        [7  ; AIX        ; ELFOSABI_AIX                  ; "AIX"        ; "AIX"],
        [8  ; IRIX       ; ELFOSABI_IRIX                 ; "IRIX"       ; "IRIX"],
        [9  ; FREEBSD    ; ELFOSABI_FREEBSD              ; "FREEBSD"    ; "FreeBSD"],
        [10 ; TRU64      ; ELFOSABI_TRU64                ; "TRU64"      ; "Compaq TRU64 UNIX"],
        [11 ; MODESTO    ; ELFOSABI_MODESTO              ; "MODESTO"    ; "Novell Modesto"],
        [12 ; OPENBSD    ; ELFOSABI_OPENBSD              ; "OPENBSD"    ; "Open BSD"],
        [13 ; OPENVMS    ; ELFOSABI_OPENVMS              ; "OPENVMS"    ; "Open VMS"],
        [14 ; NSK        ; ELFOSABI_NSK                  ; "NSK"        ; "Hewlett-Packard Non-Stop Kernel"],
        [15 ; AROS       ; ELFOSABI_AROS                 ; "AROS"       ; "Amiga Research OS"],
        [16 ; FENIXOS    ; ELFOSABI_FENIXOS              ; "FENIXOS"    ; "The FenixOS highly scalable multi-core OS"],
        [17 ; CLOUDABI   ; ELFOSABI_CLOUDABI             ; "CLOUDABI"   ; "Nuxi CloudABI"],
        [18 ; OPENVOS    ; ELFOSABI_OPENVOS              ; "OPENVOS"    ; "Stratus Technologies OpenVOS"],
        // 64-255 Architecture-specific value range
        // [0   ; Sysv       ; SYSV                          ; "SYSV"       ; "Alias"],
        // [1   ; Hpux       ; HPUX                          ; "HPUX"       ; "HP-UX"],
        // [2   ; NetBSD     ; NETBSD                        ; "NETBSD"     ; "NetBSD"],
        // [3   ; Gnu        ; GNU                           ; "GNU"        ; "Object uses GNU ELF extensions"],
        // [6   ; Solaris    ; SOLARIS                       ; "SOLARIS"    ; "Sun Solaris"],
        // [7   ; Aix        ; AIX                           ; "AIX"        ; "IBM AIX"],
        // [8   ; Irix       ; IRIX                          ; "IRIX"       ; "SGI Irix"],
        // [9   ; FreeBSD    ; FREEBSD                       ; "FREEBSD"    ; "FreeBSD"],
        // [10  ; Tru64      ; TRU64                         ; "TRU64"      ; "Compaq TRU64 UNIX"],
        // [11  ; Modesto    ; MODESTO                       ; "MODESTO"    ; "Novell Modesto"],
        // [12  ; OpenBSD    ; OPENBSD                       ; "OPENBSD"    ; "OpenBSD"],
        // [64  ; Armaeabi   ; ARMAEABI                      ; "ARMAEABI"   ; "ARM EABI"],
        // [97  ; Arm        ; ARM                           ; "ARM"        ; "ARM"],
        // [98  ; Undefined  ; UNDEFINED                     ; "UNDEFINED"  ; "NotSpecificed"],
        // [255 ; Standalone ; STANDALONE                    ; "STANDALONE" ; "Standalone (embedded) application"],
    ]
);
