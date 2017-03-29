pub trait WrappingIncrement<N> {
    fn wrapping_increment(&mut self, n2: N);
    fn wrapping_decrement(&mut self, n2: N);
}

impl WrappingIncrement<u32> for u32 {
    fn wrapping_increment(&mut self, n2: u32) {
        *self = self.wrapping_add(n2);
    }

    fn wrapping_decrement(&mut self, n2: u32) {
        *self = self.wrapping_sub(n2);
    }
}
