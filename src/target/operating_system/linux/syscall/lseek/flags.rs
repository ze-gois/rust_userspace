pub enum Flag {
    SET = 0,
    CUR = 1,
    END = 2,
}

impl Flag {
    pub fn to(self) -> i32 {
        self as i32
    }
}
