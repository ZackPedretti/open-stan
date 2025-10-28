enum Lane {
    T1,
    T2,
    T3,
    T4,
    T5,
    Corol,
    N { num: u32 },
    Citadine1,
    Citadine2,
}

impl Lane {
    pub fn new_num_line(num: u32) -> Option<Self> {
        if (num > 0 && num < 19)
            || (num > 19 && num < 25)
            || (num > 30 && num < 34)
            || (num > 50 && num < 68) {
            return Some(Lane::N { num });
        }
        None
    }
}