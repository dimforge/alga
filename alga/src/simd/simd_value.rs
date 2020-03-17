/// Trait implemented by Simd types as well as scalar types (f32, u32, etc.).
pub trait SimdValue: Copy {
    /// The type of the elements of each lane of this SIMD value.
    type Element: Copy;

    fn lanes() -> usize;
    fn splat(val: Self::Element) -> Self;
    fn extract(self, i: usize) -> Self::Element;
    unsafe fn extract_unchecked(self, i: usize) -> Self::Element;
    fn replace(self, i: usize, val: Self::Element) -> Self;
    unsafe fn replace_unchecked(self, i: usize, val: Self::Element) -> Self;

    #[inline(always)]
    fn map(self, f: impl Fn(Self::Element) -> Self::Element) -> Self {
        let mut result = self;

        for i in 0..Self::lanes() {
            unsafe { result = result.replace_unchecked(i, f(self.extract_unchecked(i))) }
        }

        result
    }

    #[inline(always)]
    fn zip_map(self, b: Self, f: impl Fn(Self::Element, Self::Element) -> Self::Element) -> Self {
        let mut result = self;

        for i in 0..Self::lanes() {
            unsafe {
                let a = self.extract_unchecked(i);
                let b = b.extract_unchecked(i);
                result = result.replace_unchecked(i, f(a, b))
            }
        }

        result
    }
}
