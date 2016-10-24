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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _vv = VecVec::new(4, 3, 'a');
    }
}
