#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VecVec<T> {
    inner: Vec<T>,
    width: usize,
    height: usize,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
