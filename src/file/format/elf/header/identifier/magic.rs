ample::enum_labeled!(
    pub enum Magic,
    u8,
    "Dope",
    [
        [0x7f; Magic0; ELFMAG0; "Magic 0"; "Del (0x7f)"],
        [0x45; Magic1; ELFMAG1; "Magic 1"; "E (0x45)"],
        [0x4c; Magic2; ELFMAG2; "Magic 2"; "L (0x4c)"],
        [0x46; Magic3; ELFMAG3; "Magic 3"; "F (0x46)"],
    ]
);

impl core::fmt::Debug for Magic {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let d = self.discriminant();
        write!(f, "\n\t{:?}\t({:b} == {:} == 0x{:x})", d as char, d, d, d)
    }
}
