extern crate dragorust_engine;

use dragorust_engine::*;

mod arena {
    use super::container::arena::*;

    #[test]
    fn new() {
        let arena = Arena::<i32>::new();
        assert!(arena.is_empty());
        assert!(arena.len() == 0);
        assert!(arena.capacity() == 0);
    }

    #[test]
    fn add() {
        let mut arena = Arena::new();

        for i in 0..10 {
            let idx = arena.add(i * 10);
            assert!(arena[idx] == i * 10);
        }
        assert!(!arena.is_empty());
        assert!(arena.len() == 10);
    }

    fn add_many_(arena: &mut Arena<usize>) {
        let mut i2000 = !0;
        let mut i5764 = !0;
        for i in 0..6_000 {
            let idx = arena.add(i * 10);
            if i == 2000 {
                i2000 = idx;
            }
            if i == 5764 {
                i5764 = idx;
            }
            assert!(arena[idx] == i * 10);
        }
        assert!(!arena.is_empty());
        assert!(arena.len() == 6_000);

        assert!(arena.get(i2000) == Some(&20000));
        assert!(arena.get(i5764) == Some(&57640));
    }

    #[test]
    fn add_many() {
        let mut arena = Arena::new();
        add_many_(&mut arena);
    }

    #[test]
    fn new_with_capacity() {
        let mut arena = Arena::new_with_capacity(10);
        assert!(!arena.is_full());
        assert!(arena.capacity() >= 10);
        let cap = arena.capacity();

        for i in 0..cap {
            arena.add(i);
        }
        assert!(arena.len() == cap);
        assert!(arena.is_full());
        assert!(arena.capacity() == cap);

        arena.add(cap);
        assert!(arena.len() == cap + 1);
        assert!(arena.capacity() > cap);
    }

    fn remove_(arena: &mut Arena<usize>) {
        let i0 = arena.add(0);
        let i1 = arena.add(10);
        let i2 = arena.add(20);
        let i3 = arena.add(30);
        assert!(arena.len() == 4);

        assert!(arena.remove(i1) == Some(10));
        assert!(arena.remove(i2) == Some(20));
        assert!(arena.len() == 2);
        assert!(arena.get(i0) == Some(&0));
        assert!(arena.get(i3) == Some(&30));

        let i4 = arena.add(40);
        let i5 = arena.add(50);
        assert!(arena.get(i0) == Some(&0));
        assert!(arena.get(i3) == Some(&30));
        assert!(arena.get(i4) == Some(&40));
        assert!(arena.get(i5) == Some(&50));
        assert!(arena.len() == 4);

        assert!(arena.remove(i0) == Some(0));
        let i6 = arena.add(60);
        assert!(arena.get(i3) == Some(&30));
        assert!(arena.get(i4) == Some(&40));
        assert!(arena.get(i5) == Some(&50));
        assert!(arena.get(i6) == Some(&60));
        assert!(arena.len() == 4);

        let i7 = arena.add(70);
        assert!(arena.get(i3) == Some(&30));
        assert!(arena.get(i4) == Some(&40));
        assert!(arena.get(i5) == Some(&50));
        assert!(arena.get(i6) == Some(&60));
        assert!(arena.get(i7) == Some(&70));
        assert!(arena.len() == 5);
    }

    #[test]
    fn remove() {
        let mut arena = Arena::new();
        remove_(&mut arena);
    }

    #[test]
    #[ignore]
    fn invalid_remove() {
        let mut arena = Arena::new();
        let i0 = arena.add(0.to_string());
        let _ = arena.add(1.to_string());
        let _ = arena.add(2.to_string());
        let _ = arena.add(3.to_string());
        let _ = arena.add(4.to_string());
        let i5 = arena.add(5.to_string());

        assert!(arena.remove(i0) == Some("0".to_string()));
        assert!(arena.remove(i5) == Some("5".to_string()));

        assert!(arena.remove(!0) == None);
        assert!(arena.remove(10) == None);
        assert!(arena.remove(11) == None);

        assert!(arena.remove(i0) == None);
        assert!(arena.remove(i5) == None);
    }

    #[test]
    #[ignore]
    fn clear() {
        let mut arena = Arena::new();
        arena.add(10);
        arena.add(20);

        assert!(!arena.is_empty());
        assert!(arena.len() == 2);

        let cap = arena.capacity();
        arena.clear();

        assert!(arena.is_empty());
        assert!(arena.len() == 0);
        assert!(arena.capacity() == cap);

        add_many_(&mut arena);
        arena.clear();
        remove_(&mut arena);
    }

    #[test]
    fn indexing() {
        let mut arena = Arena::new();

        let a = arena.add(10);
        let b = arena.add(20);
        let c = arena.add(30);

        arena[b] += arena[c];
        assert!(arena[b] == 50);
        assert!(arena[a] == 10);
        assert!(arena[c] == 30);
    }

    #[test]
    #[should_panic]
    fn indexing_vacant() {
        let mut arena = Arena::new();

        let _ = arena.add(10);
        let b = arena.add(20);
        let _ = arena.add(30);

        arena.remove(b);
        arena[b];
    }

    #[test]
    #[should_panic]
    fn invalid_indexing() {
        let mut arena = Arena::new();

        arena.add(10);
        arena.add(20);
        arena.add(30);

        arena[100];
    }

    #[test]
    fn get() {
        let mut arena = Arena::new();

        let a = arena.add(10);
        let b = arena.add(20);
        let c = arena.add(30);

        *arena.get_mut(b).unwrap() += *arena.get(c).unwrap();
        assert!(arena.get(a) == Some(&10));
        assert!(arena.get(b) == Some(&50));
        assert!(arena.get(c) == Some(&30));

        arena.remove(b);
        assert!(arena.get(b) == None);
        assert!(arena.get_mut(b) == None);
    }
}