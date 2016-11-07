use std::{fmt, marker};

#[derive(Clone, PartialEq, Eq)]
pub struct VecVec<T> {
    inner: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> VecVec<T> {
    pub fn new(width: usize, height: usize, value: T) -> Self
        where T: Clone
    {
        VecVec {
            inner: vec![value; width * height],
            width: width,
            height: height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width && y < self.height {
            Some(&self.inner[y * self.width + x])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x < self.width && y < self.height {
            Some(&mut self.inner[y * self.width + x])
        } else {
            None
        }
    }

    pub fn slice(&self,
                 x: usize,
                 y: usize,
                 width: usize,
                 height: usize)
                 -> Option<Slice<T, Immutable<T>>> {
        if x + width <= self.width && y + height <= self.height {
            Some(Slice {
                ptr: self as *const _,
                x: x,
                y: y,
                width: width,
                height: height,
                _mutability: Immutable { marker: marker::PhantomData },
            })
        } else {
            None
        }
    }

    pub fn slice_mut(&mut self,
                     x: usize,
                     y: usize,
                     width: usize,
                     height: usize)
                     -> Option<Slice<T, Mutable<T>>> {
        unsafe { self.slice_mut_unsafe(x, y, width, height) }
    }

    pub fn hsplit_at(&self, y: usize) -> Option<(Slice<T, Immutable<T>>, Slice<T, Immutable<T>>)> {
        if y <= self.height {
            Some((self.slice(0, 0, self.width, y).unwrap(),
                  self.slice(0, y, self.width, self.height - y).unwrap()))
        } else {
            None
        }
    }

    pub fn vsplit_at(&self, x: usize) -> Option<(Slice<T, Immutable<T>>, Slice<T, Immutable<T>>)> {
        if x <= self.width {
            Some((self.slice(0, 0, x, self.height).unwrap(),
                  self.slice(x, 0, self.width - x, self.height).unwrap()))
        } else {
            None
        }
    }

    pub fn hsplit_at_mut(&mut self,
                         y: usize)
                         -> Option<(Slice<T, Mutable<T>>, Slice<T, Mutable<T>>)> {
        if y <= self.height {
            let width = self.width;
            let height = self.height;
            unsafe {
                Some((self.slice_mut_unsafe(0, 0, width, y).unwrap(),
                      self.slice_mut_unsafe(0, y, width, height - y).unwrap()))
            }
        } else {
            None
        }
    }

    pub fn vsplit_at_mut(&mut self,
                         x: usize)
                         -> Option<(Slice<T, Mutable<T>>, Slice<T, Mutable<T>>)> {
        if x <= self.width {
            let width = self.width;
            let height = self.height;
            unsafe {
                Some((self.slice_mut_unsafe(0, 0, x, height).unwrap(),
                      self.slice_mut_unsafe(x, 0, width - x, height).unwrap()))
            }
        } else {
            None
        }
    }

    // Unsafe because the lifetime attached to the return value is chosen by the
    // caller. The caller must ensure the chosen lifetime does not cause memory
    // unsafety.
    unsafe fn slice_mut_unsafe<'arbitrary>(&mut self,
                                           x: usize,
                                           y: usize,
                                           width: usize,
                                           height: usize)
                                           -> Option<Slice<T, Mutable<'arbitrary, T>>> {
        if x + width <= self.width && y + height <= self.height {
            Some(Slice {
                ptr: self as *mut _,
                x: x,
                y: y,
                width: width,
                height: height,
                _mutability: Mutable { marker: marker::PhantomData },
            })
        } else {
            None
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for VecVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.slice(0, 0, self.width, self.height).unwrap().fmt(f)
    }
}

pub struct Immutable<'a, T: 'a> {
    marker: marker::PhantomData<&'a VecVec<T>>,
}

pub struct Mutable<'a, T: 'a> {
    marker: marker::PhantomData<&'a mut VecVec<T>>,
}

pub struct Slice<T, Mutability> {
    ptr: *const VecVec<T>,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    _mutability: Mutability,
}

impl<'a, T, Mutability> Slice<T, Mutability> {
    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width && y < self.height {
            unsafe { (*self.ptr).get(self.x + x, self.y + y) }
        } else {
            None
        }
    }
}

impl<'a, T: 'a> Slice<T, Mutable<'a, T>> {
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x < self.width && y < self.height {
            unsafe { (*(self.ptr as *mut VecVec<T>)).get_mut(self.x + x, self.y + y) }
        } else {
            None
        }
    }
}

impl<T: PartialEq, Mutability> PartialEq for Slice<T, Mutability> {
    fn eq(&self, rhs: &Self) -> bool {
        if self.width() == rhs.width() && self.height() == rhs.height() {
            for y in 0..self.height() {
                for x in 0..self.width() {
                    if self.get(x, y).unwrap() != rhs.get(x, y).unwrap() {
                        return false;
                    }
                }
            }
            true
        } else {
            false
        }
    }
}

impl<T: fmt::Debug, Mutability> fmt::Debug for Slice<T, Mutability> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct Row<'a, T: fmt::Debug + 'a, Mutability: 'a>(&'a Slice<T, Mutability>, usize);

        impl<'a, T: fmt::Debug, Mutability> fmt::Debug for Row<'a, T, Mutability> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_list()
                    .entries((0..self.0.width).map(|x| self.0.get(x, self.1).unwrap()))
                    .finish()
            }
        }

        f.debug_list()
            .entries((0..self.height).map(|y| Row(self, y)))
            .finish()
    }
}

impl<'a, T: PartialEq, Mutability> PartialEq<&'a [&'a [T]]> for Slice<T, Mutability> {
    fn eq(&self, rhs: &&[&[T]]) -> bool {
        if rhs.len() == self.height() && rhs.iter().all(|row| row.len() == self.width()) {
            rhs.iter().enumerate().all(|(y, row)| {
                row.iter().enumerate().all(|(x, cell)| self.get(x, y).unwrap() == cell)
            })
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! s {
        ($($expr:expr),*) => { &[$($expr),*][..] }
    }

    #[test]
    fn it_works() {
        let mut vv = VecVec::new(4, 3, 0);

        assert_eq!(vv.width(), 4);
        assert_eq!(vv.height(), 3);

        for (i, (x, y)) in (0..10).flat_map(|x| (0..10).map(move |y| (x, y))).enumerate() {
            if x < 4 && y < 3 {
                assert_eq!(*vv.get(x, y).unwrap(), 0);
                *vv.get_mut(x, y).unwrap() = i;
                assert_eq!(*vv.get(x, y).unwrap(), i);
                assert_eq!(*vv.get_mut(x, y).unwrap(), i);
            } else {
                assert_eq!(vv.get(x, y), None);
                assert_eq!(vv.get_mut(x, y), None);
            }
        }

        assert!(vv.slice(0, 0, 0, 0).is_some());
        assert!(vv.slice(0, 0, 1, 1).is_some());
        assert!(vv.slice(0, 0, 4, 3).is_some());
        assert!(vv.slice(4, 3, 0, 0).is_some());

        assert!(vv.slice(0, 0, 5, 1).is_none());
        assert!(vv.slice(0, 0, 1, 4).is_none());
        assert!(vv.slice(0, 0, 1, 4).is_none());
        assert!(vv.slice(4, 3, 0, 1).is_none());
        assert!(vv.slice(4, 3, 1, 0).is_none());
        assert!(vv.slice(9, 9, 0, 0).is_none());

        {
            let slice = vv.slice(0, 1, 3, 2).unwrap();
            assert_eq!(slice.x(), 0);
            assert_eq!(slice.y(), 1);
            assert_eq!(slice.width(), 3);
            assert_eq!(slice.height(), 2);

            for x in 0..10 {
                for y in 0..10 {
                    if x < 3 && y < 2 {
                        assert_eq!(slice.get(x, y), vv.get(x, y + 1));
                    } else {
                        assert_eq!(slice.get(x, y), None);
                    }
                }
            }
        }

        let mut vv_clone = vv.clone();
        let mut slice_mut = vv_clone.slice_mut(0, 1, 3, 2).unwrap();
        assert_eq!(slice_mut.x(), 0);
        assert_eq!(slice_mut.y(), 1);
        assert_eq!(slice_mut.width(), 3);
        assert_eq!(slice_mut.height(), 2);
        for (i, (x, y)) in (0..10).flat_map(|x| (0..10).map(move |y| (x, y))).enumerate() {
            if x < 3 && y < 2 {
                assert_eq!(*slice_mut.get(x, y).unwrap(), *vv.get(x, y + 1).unwrap());
                *slice_mut.get_mut(x, y).unwrap() = i;
                assert_eq!(*slice_mut.get(x, y).unwrap(), i);
                assert_eq!(*slice_mut.get_mut(x, y).unwrap(), i);
            } else {
                assert_eq!(slice_mut.get(x, y), None);
                assert_eq!(slice_mut.get_mut(x, y), None);
            }
        }

        let mut vv = VecVec::new(4, 3, 0);
        for (i, (x, y)) in (0..3).flat_map(|y| (0..4).map(move |x| (x, y))).enumerate() {
            *vv.get_mut(x, y).unwrap() = i;
        }

        let (first, rest) = vv.hsplit_at(1).unwrap();

        assert_eq!(first.x(), 0);
        assert_eq!(first.y(), 0);
        assert!(first == s![s![0, 1, 2, 3]]);

        assert_eq!(rest.x(), 0);
        assert_eq!(rest.y(), 1);
        assert!(rest == s![s![4, 5, 6, 7], s![8, 9, 10, 11]]);

        let (_, empty) = vv.hsplit_at(3).unwrap();
        assert_eq!(empty.width(), 4);
        assert_eq!(empty.height(), 0);
        assert!(empty == s![]);

        assert!(vv.hsplit_at(4).is_none());
        assert!(vv.hsplit_at(5).is_none());

        let mut vv = VecVec::new(4, 3, 0);
        for (i, (x, y)) in (0..4).flat_map(|x| (0..3).map(move |y| (x, y))).enumerate() {
            *vv.get_mut(x, y).unwrap() = i;
        }

        let (first, rest) = vv.vsplit_at(1).unwrap();

        assert_eq!(first.x(), 0);
        assert_eq!(first.y(), 0);
        assert!(first == s![s![0], s![1], s![2]]);

        assert_eq!(rest.x(), 1);
        assert_eq!(rest.y(), 0);
        assert!(rest == s![s![3, 6, 9], s![4, 7, 10], s![5, 8, 11]]);

        let (_, empty) = vv.vsplit_at(4).unwrap();
        assert!(empty == s![s![], s![], s![]]);

        assert!(vv.vsplit_at(5).is_none());

        let mut vv = VecVec::new(4, 3, 0);
        for (i, (x, y)) in (0..3).flat_map(|y| (0..4).map(move |x| (x, y))).enumerate() {
            *vv.get_mut(x, y).unwrap() = i;
        }

        {
            let (mut first, mut rest) = vv.hsplit_at_mut(1).unwrap();

            assert_eq!(first.x(), 0);
            assert_eq!(first.y(), 0);
            assert!(first == s![s![0, 1, 2, 3]]);

            assert_eq!(rest.x(), 0);
            assert_eq!(rest.y(), 1);
            assert!(rest == s![s![4, 5, 6, 7], s![8, 9, 10, 11]]);

            for x in 0..10 {
                for y in 0..10 {
                    if let Some(i) = first.get_mut(x, y) {
                        *i = *i * *i;
                    }
                    if let Some(i) = rest.get_mut(x, y) {
                        *i = *i * *i;
                    }
                }
            }
        }

        for (i, (x, y)) in (0..3).flat_map(|y| (0..4).map(move |x| (x, y))).enumerate() {
            assert_eq!(*vv.get(x, y).unwrap(), i * i);
        }

        {
            let (_, empty) = vv.hsplit_at_mut(3).unwrap();
            assert_eq!(empty.width(), 4);
            assert!(empty == s![]);
        }

        assert!(vv.hsplit_at_mut(4).is_none());
        assert!(vv.hsplit_at_mut(5).is_none());

        let mut vv = VecVec::new(4, 3, 0);
        for (i, (x, y)) in (0..4).flat_map(|x| (0..3).map(move |y| (x, y))).enumerate() {
            *vv.get_mut(x, y).unwrap() = i;
        }

        {
            let (mut first, mut rest) = vv.vsplit_at_mut(1).unwrap();

            assert_eq!(first.x(), 0);
            assert_eq!(first.y(), 0);
            assert!(first == s![s![0], s![1], s![2]]);

            assert_eq!(rest.x(), 1);
            assert_eq!(rest.y(), 0);
            assert!(rest == s![s![3, 6, 9], s![4, 7, 10], s![5, 8, 11]]);

            for x in 0..10 {
                for y in 0..10 {
                    if let Some(i) = first.get_mut(x, y) {
                        *i = *i * *i;
                    }
                    if let Some(i) = rest.get_mut(x, y) {
                        *i = *i * *i;
                    }
                }
            }
        }

        for (i, (x, y)) in (0..4).flat_map(|x| (0..3).map(move |y| (x, y))).enumerate() {
            assert_eq!(*vv.get(x, y).unwrap(), i * i);
        }

        {
            let (_, empty) = vv.vsplit_at_mut(4).unwrap();
            assert!(empty == s![s![], s![], s![]]);
        }

        assert!(vv.vsplit_at_mut(5).is_none());

        let mut vv = VecVec::new(7, 7, 'a');
        assert!(vv.slice(0, 0, 2, 2).unwrap() == vv.slice(1, 1, 2, 2).unwrap());
        assert!(vv.slice(1, 2, 3, 4).unwrap() == vv.slice(2, 1, 3, 4).unwrap());
        assert!(vv.slice(5, 5, 0, 0).unwrap() == vv.slice(2, 2, 0, 0).unwrap());
        assert!(vv.slice(0, 0, 2, 2).unwrap() != vv.slice(2, 2, 0, 0).unwrap());
        *vv.get_mut(1, 2).unwrap() = 'b';
        assert!(vv.slice(0, 0, 2, 2).unwrap() != vv.slice(1, 1, 2, 2).unwrap());
        assert!(vv.slice(1, 2, 3, 4).unwrap() != vv.slice(2, 1, 3, 4).unwrap());
        assert!(vv.slice(5, 5, 0, 0).unwrap() == vv.slice(2, 2, 0, 0).unwrap());
        assert!(vv.slice(0, 0, 2, 2).unwrap() != vv.slice(2, 2, 0, 0).unwrap());
        assert!(vv.slice(5, 5, 1, 1).unwrap() == vv.slice(5, 5, 1, 1).unwrap());

        assert!(vv.slice(0, 0, 2, 2).unwrap() == s![s!['a', 'a'], s!['a', 'a']]);
        assert!(vv.slice(0, 0, 2, 2).unwrap() != s![s!['a', 'a']]);
        assert!(vv.slice(0, 0, 2, 2).unwrap() != s![s!['a', 'a'], s!['a', 'a'], s!['a', 'a']]);
        assert!(vv.slice(2, 2, 0, 0).unwrap() == s![]);
        assert!(vv.slice(1, 2, 2, 2).unwrap() == s![s!['b', 'a'], s!['a', 'a']]);

        #[derive(Clone, Debug)]
        struct Foo;
        let vv = VecVec::new(2, 3, Foo);
        assert_eq!(format!("{:?}", vv), "[[Foo, Foo], [Foo, Foo], [Foo, Foo]]");
        assert_eq!(format!("{:?}", vv.slice(0, 0, 0, 0).unwrap()), "[]");
        assert_eq!(format!("{:?}", vv.slice(0, 0, 1, 1).unwrap()), "[[Foo]]");
        assert_eq!(format!("{:?}", vv.slice(0, 0, 1, 2).unwrap()),
                   "[[Foo], [Foo]]");
        assert_eq!(format!("{:?}", vv.slice(0, 0, 2, 1).unwrap()),
                   "[[Foo, Foo]]");
    }
}
