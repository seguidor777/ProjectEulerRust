use core::iterator::{ Iterator, IteratorUtil };
use core::num::{ Zero, One };

pub struct UintRange {
    cur: uint,
    cnt: uint,
    step: int,
}

impl UintRange {
    pub fn new(start: uint, stop: uint, step: int) -> UintRange {
        if step == 0 {
            fail!("UintRange::new called with step == 0");
        }

        let mut cnt = 0;
        if step > 0 && start < stop {
            let diff = (stop - start);
            cnt = diff / (step as uint);
            if diff % (step as uint) != 0 { cnt += 1; }
        }
        if step < 0 && start > stop {
            let diff = (start - stop);
            cnt = diff / ((-step) as uint);
            if diff % ((-step) as uint) != 0 { cnt += 1; }
        }
        UintRange { cur: start, cnt: cnt, step: step }
    }
}

pub fn uint_range(start: uint, stop: uint) -> UintRange {
    UintRange::new(start, stop, 1)
}

impl Iterator<uint> for UintRange {
    fn next(&mut self) -> Option<uint> {
        if self.cnt == 0 { return None; }

        let val = self.cur;

        match self.step.cmp(&0) {
            Greater => {
                self.cnt -= 1;
                self.cur += (self.step as uint);
                return Some(val);
            },
            Less => {
                self.cnt -= 1;
                self.cur -= ((- self.step) as uint);
                return Some(val);
            },
            Equal => { fail!() }
        }
    }
}

pub struct Fibonacci<T> {
    prev: T,
    cur: T
}

impl<T: Zero + One> Fibonacci<T> {
    pub fn new() -> Fibonacci<T> { Fibonacci { prev: Zero::zero(), cur: One::one() } }
}

// Copy must be Clone
impl<T: Add<T,T> + Copy> Iterator<T> for Fibonacci<T> {
    fn next(&mut self) -> Option<T> {
        let next = self.prev + self.cur;
        // let cur  = self.cur.clone();
        let cur  = self.cur;
        // self.prev = cur.clone();
        self.prev = cur;
        self.cur  = next;
        // return Some(cur);
        return Some(cur);
    }
}

pub struct Triangle {
    idx: uint,
    cur:  uint
}

impl Triangle {
    pub fn new() -> Triangle { Triangle { idx: 1, cur: 1 } }
}

impl Iterator<uint> for Triangle {
    fn next(&mut self) -> Option<uint> {
        let cur = self.cur;
        self.idx += 1;
        self.cur += self.idx;
        return Some(cur);
    }
}



pub trait ExtIteratorUtil<A> {
    fn filter_map<'r, B>(self, f: &'r fn(A) -> Option<B>) -> FilterMapIterator<'r, A, B, Self>;
    fn windowed(self, n: uint) -> WindowedIterator<A, Self>;

    fn count_elem(self) -> uint;
    fn nth(self, n: uint) -> A;
}

impl<A, T: Iterator<A>> ExtIteratorUtil<A> for T {
    fn filter_map<'r, B>(self, f: &'r fn(A) -> Option<B>) -> FilterMapIterator<'r, A, B, T> {
        FilterMapIterator { iter: self, f: f }
    }

    fn windowed(self, n: uint) -> WindowedIterator<A, T> {
        WindowedIterator { iter: self, n: n, vs: ~[] }
    }

    fn count_elem(self) -> uint {
        let mut it = self;
        let mut cnt = 0;
        for it.advance |_| { cnt += 1; }
        return cnt;
    }

    fn nth(self, n: uint) -> A {
        let mut i = n;
        let mut it = self;
        loop {
            match it.next() {
                Some(x) => { if i == 0 { return x; }}
                None => { fail!("cannot get %uth element", n) }
            }
            i -= 1;
        }
    }
}

pub struct FilterMapIterator<'self, A, B, T> {
    priv iter: T,
    priv f: &'self fn(A) -> Option<B>
}

impl<'self, A, B, T: Iterator<A>> Iterator<B> for FilterMapIterator<'self, A, B, T> {
    #[inline]
    fn next(&mut self) -> Option<B> {
        loop {
            match self.iter.next() {
                None    => { return None; }
                Some(a) => {
                    match (self.f)(a) {
                        Some(b) => { return Some(b); }
                        None    => { loop; }
                    }
                }
            }
        }
    }
}

pub struct WindowedIterator<A, T> {
    priv iter: T,
    priv n: uint,
    priv vs: ~[A]
}

impl<'self, A: Clone, T: Iterator<A>> Iterator<~[A]> for WindowedIterator<A, T> {
    #[inline]
    fn next(&mut self) -> Option<~[A]> {
        if self.vs.len() == self.n {
            self.vs.shift();
        }
        while self.vs.len() < self.n {
            match self.iter.next() {
                Some(x) => { self.vs.push(x); }
                None    => { return None; }
            }
        }
        return Some(self.vs.clone());
    }
}

pub trait AdditiveIterator<A> {
    fn sum(self) -> A;
}

impl<A: Add<A, A> + Zero, T: Iterator<A>> AdditiveIterator<A> for T {
    fn sum(self) -> A {
        let mut sum = Zero::zero::<A>();
        let mut it = self;
        for it.advance |n| { sum = sum + n; }
        return sum;
    }
}

pub trait OrderedIterator<A> {
    fn max(self) -> A;
    fn min(self) -> A;
}

impl<A: TotalOrd, T: Iterator<A>> OrderedIterator<A> for T {
    fn max(self) -> A {
        let mut it = self;
        let mut max = match it.next() {
            Some(x) => x,
            None => fail!("cannot get maximum element of empty iterator")
        };
        for it.advance |x| { if x.cmp(&max) == Greater { max = x; }}
        return max;
    }

    fn min(self) -> A {
        let mut it = self;
        let mut min = match it.next() {
            Some(x) => x,
            None => fail!("cannot get minimum element of empty iterator")
        };
        for it.advance |x| { if x.cmp(&min) == Less { min = x; }}
        return min;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::extvec;

    #[test]
    fn test_uint_range() {
        fn gen(start: uint, end: uint, step: int) -> ~[uint] {
            extvec::from_iter(UintRange::new(start, end, step))
        }
        assert_eq!(gen(0, 3, 1), ~[0, 1, 2]);
        assert_eq!(gen(13, 10, -1), ~[13, 12, 11]);
        assert_eq!(gen(20, 26, 2), ~[20, 22, 24]);
        assert_eq!(gen(36, 30, -2), ~[36, 34, 32]);
        assert_eq!(gen(uint::max_value - 2, uint::max_value, 2),
                   ~[uint::max_value - 2]);
        assert_eq!(gen(uint::max_value - 3, uint::max_value, 2),
                   ~[uint::max_value - 3, uint::max_value - 1]);
        assert_eq!(gen(uint::min_value + 2, uint::min_value, -2),
                   ~[uint::min_value + 2]);
        assert_eq!(gen(uint::min_value + 3, uint::min_value, -2),
                   ~[uint::min_value + 3, uint::min_value + 1]);
    }

    #[test]
    fn test_fibonacci() {
        let it = Fibonacci::new::<uint>();
        let fib = ~[ 1u, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233 ];
        assert_eq!(extvec::from_iter(it.take(fib.len())), fib);
    }

    #[test]
    fn test_triangle() {
        let it = Triangle::new();
        let tri = ~[1u, 3, 6, 10, 15, 21];
        assert_eq!(extvec::from_iter(it.take(tri.len())), tri);
    }

    #[test]
    fn test_filter_map() {
        let it  = uint_range(0, 10).filter_map(|x| if x.is_even() { Some(x*x) } else { None });
        let ans = ~[0*0u, 2*2, 4*4, 6*6, 8*8];
        assert_eq!(extvec::from_iter(it), ans);
    }

    #[test]
    fn test_count_elem() {
        assert_eq!(uint_range(0, 4).count_elem(), 4);
        assert_eq!(uint_range(0, 10).count_elem(), 10);
        assert_eq!(uint_range(10, 0).count_elem(), 0);
    }

    #[test]
    fn tespt_nth() {
        let v = &[0, 1, 2, 3, 4];
        for uint::range(0, v.len()) |i| {
            assert_eq!(v.iter().nth(i), &v[i]);
        }
    }

    #[test]
    #[should_fail]
    fn test_nth_fail() {
        let v = &[0, 1, 2, 3, 4];
        v.iter().nth(5);
    }

    #[test]
    fn test_sum() {
        assert_eq!(uint_range(0, 4).sum(), 6);
        assert_eq!(uint_range(0, 10).sum(), 45);
        assert_eq!(uint_range(10, 0).sum(), 0);
    }

    #[test]
    fn test_max() {
        assert_eq!(uint_range(0, 4).max(), 3);
        assert_eq!(uint_range(0, 10).max(), 9);
        let v = ~[0, 10, 9, 2, 3, 5];
        assert_eq!(v.iter().transform(|v| *v).max(), 10);
    }

    #[test]
    #[should_fail]
    fn test_max_fail() {
        uint_range(10, 0).max();
    }

    #[test]
    fn test_min() {
        assert_eq!(uint_range(0, 4).min(), 0);
        assert_eq!(uint_range(0, 10).min(), 0);
        let v = ~[0, 10, 9, 2, 3, 5];
        assert_eq!(v.iter().transform(|v| *v).min(), 0);
    }

    #[test] #[should_fail]
    fn test_min_fail() {
        uint_range(10, 0).min();
    }
}