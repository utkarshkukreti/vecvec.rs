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
                 -> Option<ImmutableSlice<T>> {
        self.as_slice().slice(x, y, width, height)
    }

    pub fn slice_mut(&mut self,
                     x: usize,
                     y: usize,
                     width: usize,
                     height: usize)
                     -> Option<MutableSlice<T>> {
        self.as_mut_slice().slice_mut(x, y, width, height)
    }

    pub fn hsplit_at(&self, y: usize) -> Option<(ImmutableSlice<T>, ImmutableSlice<T>)> {
        self.as_slice().hsplit_at(y)
    }

    pub fn vsplit_at(&self, x: usize) -> Option<(ImmutableSlice<T>, ImmutableSlice<T>)> {
        self.as_slice().vsplit_at(x)
    }

    pub fn hsplit_at_mut(&mut self, y: usize) -> Option<(MutableSlice<T>, MutableSlice<T>)> {
        self.as_mut_slice().hsplit_at_mut(y)
    }

    pub fn vsplit_at_mut(&mut self, x: usize) -> Option<(MutableSlice<T>, MutableSlice<T>)> {
        self.as_mut_slice().vsplit_at_mut(x)
    }

    pub fn as_slice(&self) -> ImmutableSlice<T> {
        Slice {
            ptr: self as *const _,
            x: 0,
            y: 0,
            width: self.width,
            height: self.height,
            _mutability: Immutable { marker: marker::PhantomData },
        }
    }

    pub fn as_mut_slice(&self) -> MutableSlice<T> {
        Slice {
            ptr: self as *const _,
            x: 0,
            y: 0,
            width: self.width,
            height: self.height,
            _mutability: Mutable { marker: marker::PhantomData },
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

pub type ImmutableSlice<'a, T> = Slice<T, Immutable<'a, T>>;

pub type MutableSlice<'a, T> = Slice<T, Mutable<'a, T>>;

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

    pub fn slice(&self,
                 x: usize,
                 y: usize,
                 width: usize,
                 height: usize)
                 -> Option<ImmutableSlice<'a, T>> {
        if x + width <= self.width && y + height <= self.height {
            Some(Slice {
                ptr: self.ptr,
                x: self.x + x,
                y: self.y + y,
                width: width,
                height: height,
                _mutability: Immutable { marker: marker::PhantomData },
            })
        } else {
            None
        }
    }

    pub fn hsplit_at(&self, y: usize) -> Option<(ImmutableSlice<'a, T>, ImmutableSlice<'a, T>)> {
        if y <= self.height {
            Some((self.slice(0, 0, self.width, y).unwrap(),
                  self.slice(0, y, self.width, self.height - y).unwrap()))
        } else {
            None
        }
    }

    pub fn vsplit_at(&self, x: usize) -> Option<(ImmutableSlice<'a, T>, ImmutableSlice<'a, T>)> {
        if x <= self.width {
            Some((self.slice(0, 0, x, self.height).unwrap(),
                  self.slice(x, 0, self.width - x, self.height).unwrap()))
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

    pub fn slice_mut(&mut self,
                     x: usize,
                     y: usize,
                     width: usize,
                     height: usize)
                     -> Option<MutableSlice<'a, T>> {
        unsafe { self.slice_mut_unsafe(x, y, width, height) }
    }

    pub fn hsplit_at_mut(&mut self,
                         y: usize)
                         -> Option<(MutableSlice<'a, T>, MutableSlice<'a, T>)> {
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
                         -> Option<(MutableSlice<'a, T>, MutableSlice<'a, T>)> {
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
                                           -> Option<MutableSlice<'arbitrary, T>> {
        if x + width <= self.width && y + height <= self.height {
            Some(Slice {
                ptr: self.ptr,
                x: self.x + x,
                y: self.y + y,
                width: width,
                height: height,
                _mutability: Mutable { marker: marker::PhantomData },
            })
        } else {
            None
        }
    }
}

impl<T: PartialEq, M1, M2> PartialEq<Slice<T, M2>> for Slice<T, M1> {
    fn eq(&self, rhs: &Slice<T, M2>) -> bool {
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
