ample::flags!(
    u32;
    "Getrandom flags";
    pub struct Flags {
        [0x0001; NON_BLOCKING; GRND_NONBLOCK; "GRND_NONBLOCK"; "Do not block waiting for entropy"],
        [0x0002; RANDOM;       GRND_RANDOM;   "GRND_RANDOM";   "Use the random source semantics"],
        [0x0004; INSECURE;     GRND_INSECURE; "GRND_INSECURE"; "Permit output before the entropy pool is initialized"]
    }
);
