ample::enum_labeled!(
    #[derive(Debug)]
    pub enum Version,
    u8,
    "ELF Version",
    [
        [0; Invalid;  EV_NONE;    "Invalid"; "Invalid version"],
        [1; Current;  EV_CURRENT; "Current"; "Current version"],
    ]
);
