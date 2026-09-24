#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Growth {
    Downward,
    Upward,
}

/// A generic stack memory region.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Region {
    lower: usize,
    upper: usize,
    pointer: usize,
    growth: Growth,
}

impl Region {
    pub fn new(lower: usize, upper: usize, pointer: usize, growth: Growth) -> Option<Self> {
        if lower > upper || pointer < lower || pointer > upper {
            return None;
        }

        Some(Self { lower, upper, pointer, growth })
    }

    pub fn empty(lower: usize, upper: usize, growth: Growth) -> Option<Self> {
        let pointer = match growth {
            Growth::Downward => upper,
            Growth::Upward => lower,
        };
        Self::new(lower, upper, pointer, growth)
    }

    pub const fn lower(&self) -> usize { self.lower }
    pub const fn upper(&self) -> usize { self.upper }
    pub const fn pointer(&self) -> usize { self.pointer }
    pub const fn growth(&self) -> Growth { self.growth }
    pub const fn capacity(&self) -> usize { self.upper - self.lower }

    pub const fn used(&self) -> usize {
        match self.growth {
            Growth::Downward => self.upper - self.pointer,
            Growth::Upward => self.pointer - self.lower,
        }
    }

    pub const fn remaining(&self) -> usize { self.capacity() - self.used() }
    pub const fn is_empty(&self) -> bool { self.used() == 0 }
    pub const fn contains(&self, address: usize) -> bool {
        address >= self.lower && address < self.upper
    }
}
