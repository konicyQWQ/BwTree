use std::sync::atomic::{AtomicPtr, Ordering};

pub struct LockfreeList<T> {
    head: AtomicPtr<Node<T>>,
}

impl<T> LockfreeList<T> {
    pub fn new() -> Self {
        LockfreeList {
            head: AtomicPtr::new(std::ptr::null_mut()),
        }
    }

    pub fn push_front(&self, value: T) {
        let node = Box::into_raw(Box::new(Node::new(value)));

        loop {
            let current = self.head.load(Ordering::Relaxed);

            unsafe { (*node).next.store(current, Ordering::Relaxed) };

            if self
                .head
                .compare_exchange_weak(current, node, Ordering::SeqCst, Ordering::Relaxed)
                .is_ok()
            {
                break;
            }
        }
    }
}

pub struct Node<T> {
    value: T,
    next: AtomicPtr<Node<T>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            next: AtomicPtr::new(std::ptr::null_mut()),
        }
    }
}
