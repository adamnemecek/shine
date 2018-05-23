use std::sync::Mutex;
use std::usize;
use num_cpus;
use libconfig;

/// Thread ID manager. Allocates and release thread ids
struct ThreadIdManager {
    thread_counter: usize,
    thread_limit: usize,
    free_list: Vec<usize>,
}

impl ThreadIdManager {
    fn new() -> ThreadIdManager {
        info!("Max thread count: {}", get_max_thread_count());
        info!("Preferred thread count: {}", get_preferred_thread_count());

        ThreadIdManager {
            thread_counter: 0,
            thread_limit: get_max_thread_count(),
            free_list: Vec::new(),
        }
    }

    fn alloc(&mut self) -> usize {
        if let Some(id) = self.free_list.pop() {
            id
        } else {
            let id = self.thread_counter;
            self.thread_counter += 1;
            assert!(self.thread_counter <= self.thread_limit, "The running threads exceeds maximum allowed: {}", self.thread_limit);
            id
        }
    }
    fn dealloc(&mut self, id: usize) {
        self.free_list.push(id);
    }
}

lazy_static! {
    static ref THREAD_ID_MANAGER: Mutex<ThreadIdManager> = Mutex::new(ThreadIdManager::new());
}


/// Unique thread ID local to the current thread.
/// A thread ID may be reused after a thread exits.
struct ThreadId(usize);

impl ThreadId {
    fn new() -> ThreadId {
        let id = THREAD_ID_MANAGER.lock().unwrap().alloc();
        //println!("thread id alloc: {}", id);
        ThreadId(id)
    }
}

impl Drop for ThreadId {
    fn drop(&mut self) {
        //println!("thread id release: {}", self.0);
        THREAD_ID_MANAGER.lock().unwrap().dealloc(self.0);
    }
}

thread_local!(static THREAD_ID: ThreadId = ThreadId::new());


/// Returns the maximum number of threads to use.
pub fn get_max_thread_count() -> usize {
    if libconfig::MAX_THREAD_COUNT != 0 {
        libconfig::MAX_THREAD_COUNT
    } else {
        2 * num_cpus::get()
    }
}

/// Returns the preferred number of threads to use for data processing based
/// on the cpu information
pub fn get_preferred_thread_count() -> usize {
    if libconfig::PREFERRED_THREAD_COUNT != 0 {
        libconfig::PREFERRED_THREAD_COUNT
    } else {
        num_cpus::get()
    }
}

/// Returns the id of the current thread
pub fn get() -> usize {
    THREAD_ID.with(|id| id.0)
}
