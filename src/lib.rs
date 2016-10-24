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
    }
}
