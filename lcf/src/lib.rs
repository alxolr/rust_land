use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

static NTHREADS: i64 = 4;

#[no_mangle]
pub extern "C" fn lcf(n1: i64, n2: i64, done: extern "C" fn(i64)) {
    let lcf = Arc::new(AtomicUsize::new(0));
    let mut children = vec![];

    for t_number in 0..NTHREADS {
        let thread_lcf = Arc::clone(&lcf);

        children.push(thread::spawn(move || {
            single_lcf_thread(n1, n2, 2 + t_number, NTHREADS, thread_lcf);
        }));

        for child in children {
            let _ = child.join();
        }

        done(lcf.load(Ordering::Relaxed) as i64);
    }
}

fn single_lcf_thread(n1: i64, n2: i64, start: i64, step: i64, lcf: Arc<AtomicUsize>) {
    let mut check = start;
    let stop = if n1 < n2 { n1 + 1 } else { n2 + 1 };
    while check < stop {
        if (n1 % check == 0) && (n2 % check == 0) {
            lcf.store(check as usize, Ordering::Relaxed);
        }
        if lcf.load(Ordering::Relaxed) != 0 {
            return;
        }
        check += step;
    }
}
