use std::marker;

#[derive(Clone, Debug, PartialEq, Eq)]
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

    pub fn slice(&self, x: usize, y: usize, width: usize, height: usize) -> Option<Slice<T>> {
        if x + width <= self.width && y + height <= self.height {
            Some(Slice {
                inner: self,
                x: x,
                y: y,
                width: width,
                height: height,
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
                     -> Option<SliceMut<T>> {
        if x + width <= self.width && y + height <= self.height {
            Some(SliceMut {
                inner: self as *mut _,
                x: x,
                y: y,
                width: width,
                height: height,
                marker: marker::PhantomData,
            })
        } else {
            None
        }
    }

    pub fn hsplit_at(&self, y: usize) -> Option<(Slice<T>, Slice<T>)> {
        if y <= self.height {
            Some((self.slice(0, 0, self.width, y).unwrap(),
                  self.slice(0, y, self.width, self.height - y).unwrap()))
        } else {
            None
        }
    }

    pub fn vsplit_at(&self, x: usize) -> Option<(Slice<T>, Slice<T>)> {
        if x <= self.width {
            Some((self.slice(0, 0, x, self.height).unwrap(),
                  self.slice(x, 0, self.width - x, self.height).unwrap()))
        } else {
            None
        }
    }
}

pub struct Slice<'a, T: 'a> {
    inner: &'a VecVec<T>,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl<'a, T: 'a> Slice<'a, T> {
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
            self.inner.get(self.x + x, self.y + y)
        } else {
            None
        }
    }
}

pub struct SliceMut<'a, T: 'a> {
    inner: *mut VecVec<T>,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    marker: marker::PhantomData<&'a mut VecVec<T>>,
}

impl<'a, T: 'a> SliceMut<'a, T> {
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
            unsafe { (*self.inner).get(self.x + x, self.y + y) }
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x < self.width && y < self.height {
            unsafe { (*self.inner).get_mut(self.x + x, self.y + y) }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(first.width(), 4);
        assert_eq!(first.height(), 1);
        assert_eq!(*first.get(0, 0).unwrap(), 0);
        assert_eq!(*first.get(1, 0).unwrap(), 1);
        assert_eq!(*first.get(2, 0).unwrap(), 2);
        assert_eq!(*first.get(3, 0).unwrap(), 3);
        assert_eq!(first.get(4, 0), None);
        assert_eq!(first.get(0, 1), None);

        assert_eq!(rest.x(), 0);
        assert_eq!(rest.y(), 1);
        assert_eq!(rest.width(), 4);
        assert_eq!(rest.height(), 2);
        assert_eq!(*rest.get(0, 0).unwrap(), 4);
        assert_eq!(*rest.get(1, 0).unwrap(), 5);
        assert_eq!(*rest.get(2, 0).unwrap(), 6);
        assert_eq!(*rest.get(3, 0).unwrap(), 7);
        assert_eq!(*rest.get(0, 1).unwrap(), 8);
        assert_eq!(*rest.get(1, 1).unwrap(), 9);
        assert_eq!(*rest.get(2, 1).unwrap(), 10);
        assert_eq!(*rest.get(3, 1).unwrap(), 11);
        assert_eq!(rest.get(4, 0), None);
        assert_eq!(rest.get(4, 1), None);
        assert_eq!(rest.get(0, 3), None);

        let (_, empty) = vv.hsplit_at(3).unwrap();
        assert_eq!(empty.width(), 4);
        assert_eq!(empty.height(), 0);
        assert_eq!(empty.get(0, 0), None);
        assert_eq!(empty.get(1, 0), None);
        assert_eq!(empty.get(0, 1), None);

        assert!(vv.hsplit_at(4).is_none());
        assert!(vv.hsplit_at(5).is_none());

        let mut vv = VecVec::new(4, 3, 0);
        for (i, (x, y)) in (0..4).flat_map(|x| (0..3).map(move |y| (x, y))).enumerate() {
            *vv.get_mut(x, y).unwrap() = i;
        }

        let (first, rest) = vv.vsplit_at(1).unwrap();

        assert_eq!(first.x(), 0);
        assert_eq!(first.y(), 0);
        assert_eq!(first.width(), 1);
        assert_eq!(first.height(), 3);
        assert_eq!(*first.get(0, 0).unwrap(), 0);
        assert_eq!(*first.get(0, 1).unwrap(), 1);
        assert_eq!(*first.get(0, 2).unwrap(), 2);
        assert_eq!(first.get(0, 3), None);
        assert_eq!(first.get(1, 0), None);
        assert_eq!(first.get(4, 0), None);

        assert_eq!(rest.x(), 1);
        assert_eq!(rest.y(), 0);
        assert_eq!(rest.width(), 3);
        assert_eq!(rest.height(), 3);
        assert_eq!(*rest.get(0, 0).unwrap(), 3);
        assert_eq!(*rest.get(0, 1).unwrap(), 4);
        assert_eq!(*rest.get(0, 2).unwrap(), 5);
        assert_eq!(*rest.get(1, 0).unwrap(), 6);
        assert_eq!(*rest.get(1, 1).unwrap(), 7);
        assert_eq!(*rest.get(1, 2).unwrap(), 8);
        assert_eq!(*rest.get(2, 0).unwrap(), 9);
        assert_eq!(*rest.get(2, 1).unwrap(), 10);
        assert_eq!(*rest.get(2, 2).unwrap(), 11);
        assert_eq!(rest.get(2, 3), None);
        assert_eq!(rest.get(3, 2), None);
        assert_eq!(rest.get(3, 0), None);
        assert_eq!(rest.get(0, 3), None);

        let (_, empty) = vv.vsplit_at(4).unwrap();
        assert_eq!(empty.width(), 0);
        assert_eq!(empty.height(), 3);
        assert_eq!(empty.get(0, 0), None);
        assert_eq!(empty.get(1, 0), None);
        assert_eq!(empty.get(0, 1), None);

        assert!(vv.vsplit_at(5).is_none());
    }
}
