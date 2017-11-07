extern crate dragorust_engine;

use dragorust_engine::*;

mod arena {
    use super::container::arena::*;
    use std::thread;
    use std::sync::atomic::*;
    use std::sync::Arc;
    use std::time;

    const THRAD_COUNT: usize = 4;

    #[test]
    fn new() {
        let arena = Arena::<i32>::new(4);
        assert!(arena.len() == 0);
    }

    #[test]
    fn add() {
        let arena = Arena::new(4);

        let i0 = arena.alloc(0);
        assert!(*i0 == 0);
        assert!(arena.len() == 1);

        let i1 = arena.alloc(10);
        let i2 = arena.alloc(20);
        let i3 = arena.alloc(30);
        assert!(*i0 == 0);
        assert!(*i1 == 10);
        assert!(*i2 == 20);
        assert!(*i3 == 30);
        assert!(arena.len() == 4);

        let i4 = arena.alloc(40);
        let i5 = arena.alloc(50);
        let i6 = arena.alloc(60);
        assert!(*i0 == 0);
        assert!(*i1 == 10);
        assert!(*i2 == 20);
        assert!(*i3 == 30);
        assert!(*i4 == 40);
        assert!(*i5 == 50);
        assert!(*i6 == 60);
        assert!(arena.len() == 7);
    }

    #[test]
    fn release() {
        let arena = Arena::new(4);

        let i0 = arena.alloc(0);
        let i1 = arena.alloc(10);
        let i2 = arena.alloc(20);
        let i3 = arena.alloc(30);
        assert!(arena.len() == 4);

        arena.release(i1);
        arena.release(i2);
        assert!(arena.len() == 2);

        let i4 = arena.alloc(40);
        let i5 = arena.alloc(50);
        assert!(*i0 == 0);
        assert!(*i3 == 30);
        assert!(*i4 == 40);
        assert!(*i5 == 50);
        assert!(arena.len() == 4);

        arena.release(i0);
        let i6 = arena.alloc(60);
        assert!(*i3 == 30);
        assert!(*i4 == 40);
        assert!(*i5 == 50);
        assert!(*i6 == 60);
        assert!(arena.len() == 4);

        let i7 = arena.alloc(70);
        assert!(*i3 == 30);
        assert!(*i4 == 40);
        assert!(*i5 == 50);
        assert!(*i6 == 60);
        assert!(*i7 == 70);
        assert!(arena.len() == 5);
    }

    #[test]
    fn multithread_simple_count() {
        let arena = Arena::new(4);
        let cnt = AtomicUsize::new(0);
        let shared = Arc::new((arena, cnt));

        let mut th = vec!();
        {
            for tid in 0..THRAD_COUNT {
                let shared = shared.clone();
                th.push(thread::spawn(move || {
                    let mut i = 0;
                    //while !shared.1.load(Ordering::Relaxed) {
                    while shared.1.fetch_add(1, Ordering::Relaxed) < THRAD_COUNT * 1000 {
                        let t = shared.0.alloc(format!("task gen {}", i).to_string());
                        assert!(*t == format!("task gen {}", i));
                        shared.0.release(t);
                        i += 1;
                    }

                    println!("tid: {} i: {}", tid, i);
                }));
            }
        }

        for t in th.drain(..) {
            t.join().unwrap();
        }
    }

    #[test]
    fn multithread_simple_time() {
        let arena = Arena::new(8);
        let terminate = AtomicBool::new(false);
        let shared = Arc::new((arena, terminate));

        let mut th = vec!();
        {
            for tid in 0..THRAD_COUNT {
                let shared = shared.clone();
                th.push(thread::spawn(move || {
                    let mut i = 0;
                    while !shared.1.load(Ordering::Relaxed) {
                        let t = shared.0.alloc(format!("task gen {}", i).to_string());
                        assert!(*t == format!("task gen {}", i));
                        shared.0.release(t);
                        i += 1;
                    }

                    println!("tid: {} count: {}", tid, i);
                }));
            }
        }

        thread::sleep(time::Duration::from_secs(1));
        shared.1.store(true, Ordering::Relaxed);

        for t in th.drain(..) {
            t.join().unwrap();
        }
    }

    #[test]
    #[ignore]
    fn multithread_2() {
        let arena = Arena::new(4);
        let terminate = AtomicBool::new(false);
        let shared = Arc::new((arena, terminate));

        let mut th = vec!();
        {
            for tid in 0..THRAD_COUNT {
                let shared = shared.clone();
                th.push(thread::spawn(move || {
                    let mut data = vec!();
                    let mut add = true;
                    let mut i = 0;
                    while !shared.1.load(Ordering::Relaxed) {
                        //println!("tid: {}, data:{}, arena:{}", tid, data.len(), shared.0.len());
                        if add {
                            if data.len() < 5 {
                                let t = shared.0.alloc(i);
                                assert!(*t == i);
                                data.push((i, t));
                            } else {
                                add = false;
                            }
                        } else {
                            if data.len() > 0 {
                                let t =
                                    if tid % 2 == 0 {
                                        data.swap_remove(0)
                                    } else {
                                        data.remove(0)
                                    };
                                assert!(t.0 == *t.1);
                                shared.0.release(t.1);
                            } else {
                                add = true;
                            }
                        }

                        i = i + 1;
                    }

                    println!("i: {}", i);
                }));
            }
        }
        thread::sleep(time::Duration::from_secs(1));
        shared.1.store(true, Ordering::Relaxed);

        for t in th.drain(..) {
            t.join().unwrap();
        }
    }
}