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

        let slice_mut = vv.slice_mut(0, 1, 3, 2).unwrap();
        assert_eq!(slice_mut.x(), 0);
        assert_eq!(slice_mut.y(), 1);
        assert_eq!(slice_mut.width(), 3);
        assert_eq!(slice_mut.height(), 2);
    }
}
