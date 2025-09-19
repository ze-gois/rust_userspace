pub mod endianness {
    ample::enum_labeled!(
        pub enum Endianness,
        u8,
        "endianness",
        [
            [0; None;      ();   NONE; "no"; "No endianness provided"],
            [1; LSB;       bool; LSB;  "no"; "No endianness provided"],
            [2; MSB;       bool; MSB;  "no"; "No endianness provided"],
            [3; Number;    ();   NUM;  "no"; "No endianness provided"],
            [4; Undefined; ();   UN;   "no"; "No endianness provided"]
        ]
    );
}

pub use endianness::Endianness;

// define_error!(
//     "DType",
//     [
//         [0; InvalidData; NONE; "NONE";  "No " ],
//         [1; InvalidEndian; LSB; "LSB";  "No endianness provided" ],
//         [2; InvalidType; MSB; "MSB";  "No endianness provided" ],
//         [3; ShorterData; NUM; "NUM";  "No endianness provided" ],
//     ]
// );
