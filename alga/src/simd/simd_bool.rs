pub trait SimdBool: Copy {
    fn and(self) -> bool;
    fn or(self) -> bool;
    fn xor(self) -> bool;
    fn all(self) -> bool;
    fn any(self) -> bool;
    fn none(self) -> bool;
}

impl SimdBool for bool {
    #[inline(always)]
    fn and(self) -> bool {
        self
    }

    #[inline(always)]
    fn or(self) -> bool {
        self
    }

    #[inline(always)]
    fn xor(self) -> bool {
        self
    }

    #[inline(always)]
    fn all(self) -> bool {
        self
    }

    #[inline(always)]
    fn any(self) -> bool {
        self
    }

    #[inline(always)]
    fn none(self) -> bool {
        !self
    }
}
