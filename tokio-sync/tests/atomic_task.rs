extern crate futures;
#[macro_use]
extern crate loom;

#[path = "../src/atomic_task.rs"]
mod atomic_task;

use atomic_task::AtomicTask;

use loom::sync::atomic::AtomicUsize;
use loom::thread;

use futures::Async;
use futures::future::poll_fn;

use std::sync::Arc;
use std::sync::atomic::Ordering::Relaxed;

struct Chan {
    num: AtomicUsize,
    task: AtomicTask,
}

#[test]
fn basic_notification() {
    const NUM_NOTIFY: usize = 2;

    loom::fuzz_future(|| {
        let chan = Arc::new(Chan {
            num: AtomicUsize::new(0),
            task: AtomicTask::new(),
        });

        for _ in 0..NUM_NOTIFY {
            let chan = chan.clone();

            thread::spawn(move || {
                chan.num.fetch_add(1, Relaxed);
                chan.task.notify();
            });
        }

        poll_fn(move || {
            chan.task.register();

            println!(" + chan.num.load()");
            if NUM_NOTIFY == chan.num.load(Relaxed) {
                return Ok(Async::Ready(()));
            }

            Ok(Async::NotReady)
        })
    });
}
