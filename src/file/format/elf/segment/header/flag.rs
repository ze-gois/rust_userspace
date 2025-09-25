ample::enum_flag!(
    u64;
    "Segment Flags";
    pub enum Flag {
        [0;              None;        NONE;        "none"; "All access denied All access denied"],
        [1;                 X;        PF_X;        "PF_X"; "Execute"],
        [2;                 W;        PF_W;        "PF_W"; "Write"],
        [4;                 R;        PF_R;        "PF_R"; "Read"],
        [3;                WX;       PF_WX;       "PF_WX"; "Write, execute Read, write, execute"],
        [5;                RX;       PF_RX;       "PF_RX"; "Read, execute Read, execute"],
        [6;                RW;       PF_RW;       "PF_RW"; "Read, write Read, write, execute"],
        [7;               RWX;      PF_RWX;      "PF_RWX"; "Read, write, execute Read, write, execute"],
        [0x0ff00000;   MASKOS;   PF_MASKOS;   "PF_MASKOS"; "Unspecified"],
        [0xf0000000; MASKPROC; PF_MASKPROC; "PF_MASKPROC"; "Unspecified"],
    }
);
