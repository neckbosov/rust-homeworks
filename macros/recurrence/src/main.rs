use std::ops::Index;

macro_rules! recurrence {
    ($a:ident [$n:ident]: $rec_type:ty = $($init_val:expr),*;...;$nx:expr) => {{
        struct ShiftBuf<T> {
            buf: std::collections::VecDeque<T>,
            shift: usize,
        }

        impl<T> ShiftBuf<T> {
            fn new() -> Self {
                Self {
                    buf: Default::default(),
                    shift: 0,
                }
            }
            fn pop_front(&mut self) {
                self.shift += 1;
                self.buf.pop_front();
            }
            fn push_back(&mut self, value: T) {
                self.buf.push_back(value);
            }
            fn len(&self) -> usize {
                self.buf.len()
            }
        }
        impl<T> Index<usize> for ShiftBuf<T> {
            type Output = T;

            fn index(&self, index: usize) -> &Self::Output {
                &self.buf[index - self.shift]
            }
        }
        let mut buf = ShiftBuf::new();
        $({
            buf.push_back($init_val);
        })*

        struct RecIter {
            buf: ShiftBuf<$rec_type>,
            last_ind: usize,
        }
        impl Iterator for RecIter {
            type Item = $rec_type;
            fn next(&mut self) -> Option<Self::Item> {
                let $n = self.last_ind;
                self.last_ind+=1;
                if $n < self.buf.len() {
                    Some(self.buf[$n])
                } else {
                    let $a = &mut self.buf;
                    let new_elem = $nx;
                    //$a.pop_front();
                    $a.push_back(new_elem);
                    $a.pop_front();
                    Some(new_elem)
                }
            }
        }
        RecIter {buf, last_ind: 0}
    }};
}
fn main() {
    let fib = recurrence![a[n]: i64 = 0, 1; ...; a[n-1] + a[n-2]];

    for n in fib.take(10) { print!("{} ", n); }
    // 0 1 1 2 3 5 8 13 21 34

    let other = recurrence![f[i]:f64 = 1.0; ...; f[i-1] * i as f64];
    for n in other.take(10) { print!("{} ", n); }
    //1 1 2 6 24 120 720 5040 40320 362880
}