extern crate vecvec;

use vecvec::VecVec;

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

    {
        let mut vv_clone = vv.clone();
        assert!(vv.slice(0, 0, 2, 2).unwrap() == vv_clone.slice_mut(0, 0, 2, 2).unwrap());
    }

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

    let mut vv = VecVec::new(4, 3, 0);
    for (i, (x, y)) in (0..4).flat_map(|x| (0..3).map(move |y| (x, y))).enumerate() {
        *vv.get_mut(x, y).unwrap() = i;
    }

    let slice = vv.as_slice();
    assert_eq!(slice.x(), 0);
    assert_eq!(slice.y(), 0);
    assert_eq!(slice.width(), vv.width());
    assert_eq!(slice.height(), vv.height());
    assert_eq!(slice, vv.slice(0, 0, vv.width(), vv.height()).unwrap());

    {
        let vv_clone = vv.clone();
        let mut slice = vv.as_mut_slice();
        assert_eq!(slice.x(), 0);
        assert_eq!(slice.y(), 0);
        assert_eq!(slice.width(), vv.width());
        assert_eq!(slice.height(), vv.height());
        assert_eq!(slice,
                   vv_clone.slice(0, 0, vv.width(), vv.height()).unwrap());
        *slice.get_mut(0, 0).unwrap() = 1;
    }
    assert_eq!(*vv.get(0, 0).unwrap(), 1);

    let mut vv = VecVec::new(4, 3, 0);
    for (i, (x, y)) in (0..3).flat_map(|y| (0..4).map(move |x| (x, y))).enumerate() {
        *vv.get_mut(x, y).unwrap() = i;
    }

    let slice = vv.as_slice();
    assert_eq!(slice.slice(0, 0, 0, 0).unwrap(), s![]);
    assert_eq!(slice.slice(0, 0, 2, 2).unwrap(), s![s![0, 1], s![4, 5]]);

    let (_0123_4567, _) = vv.hsplit_at(2).unwrap();
    let (_0123, _4567) = _0123_4567.hsplit_at(1).unwrap();
    assert_eq!(_0123, s![s![0, 1, 2, 3]]);
    assert_eq!(_4567, s![s![4, 5, 6, 7]]);
    let (_012, _3) = _0123.vsplit_at(3).unwrap();
    assert_eq!(_012, s![s![0, 1, 2]]);
    assert_eq!(_3, s![s![3]]);
    let (_4, _567) = _4567.vsplit_at(1).unwrap();
    assert_eq!(_4, s![s![4]]);
    assert_eq!(_567, s![s![5, 6, 7]]);
    let (empty, _567) = _567.vsplit_at(0).unwrap();
    assert_eq!(empty, s![s![]]);
    assert_eq!(_567, s![s![5, 6, 7]]);

    assert_eq!(vv.hsplit_at(4), None);
    assert_eq!(vv.hsplit_at(0).unwrap().0.vsplit_at(5), None);
    assert_eq!(vv.vsplit_at(5), None);

    let mut vv = VecVec::new(4, 3, 0);
    for (i, (x, y)) in (0..3).flat_map(|y| (0..4).map(move |x| (x, y))).enumerate() {
        *vv.get_mut(x, y).unwrap() = i;
    }

    {
        let mut slice = vv.slice_mut(1, 1, 2, 2).unwrap();
        *slice.slice_mut(0, 0, 1, 1).unwrap().get_mut(0, 0).unwrap() = 100;
        *slice.slice_mut(1, 1, 1, 1).unwrap().get_mut(0, 0).unwrap() = 101;
    }
    assert_eq!(*vv.get(1, 1).unwrap(), 100);
    assert_eq!(*vv.get(2, 2).unwrap(), 101);

    let mut vv = VecVec::new(4, 3, 0);
    for (i, (x, y)) in (0..3).flat_map(|y| (0..4).map(move |x| (x, y))).enumerate() {
        *vv.get_mut(x, y).unwrap() = i;
    }

    {
        let (mut _0123_4567, _) = vv.hsplit_at_mut(2).unwrap();
        let (mut _0123, mut _4567) = _0123_4567.hsplit_at_mut(1).unwrap();
        assert_eq!(_0123, s![s![0, 1, 2, 3]]);
        assert_eq!(_4567, s![s![4, 5, 6, 7]]);
        let (mut _012, _3) = _0123.vsplit_at_mut(3).unwrap();
        assert_eq!(_012, s![s![0, 1, 2]]);
        assert_eq!(_3, s![s![3]]);
        let (_4, mut _567) = _4567.vsplit_at_mut(1).unwrap();
        assert_eq!(_4, s![s![4]]);
        assert_eq!(_567, s![s![5, 6, 7]]);
        let (empty, mut _567) = _567.vsplit_at_mut(0).unwrap();
        assert_eq!(empty, s![s![]]);
        assert_eq!(_567, s![s![5, 6, 7]]);
        *_012.get_mut(0, 0).unwrap() = 100;
        *_567.get_mut(2, 0).unwrap() = 101;
    }

    assert_eq!(*vv.get(0, 0).unwrap(), 100);
    assert_eq!(*vv.get(3, 1).unwrap(), 101);

    assert_eq!(vv.hsplit_at_mut(4), None);
    assert_eq!(vv.hsplit_at_mut(0).unwrap().0.vsplit_at_mut(5), None);
    assert_eq!(vv.vsplit_at_mut(5), None);
}
