use crossbeam::epoch::{self, Atomic, Guard, Owned, Shared};
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};

pub struct LockFreeList<T> {
    head: Atomic<Node<T>>,
}

impl<T> LockFreeList<T> {
    pub fn new() -> Self {
        LockFreeList {
            head: Atomic::null(),
        }
    }

    pub fn push_front(&self, value: T) {
        let guard = &epoch::pin();
        let mut current = self.head.load(Relaxed, guard);
        let mut node = Owned::new(Node::new(value));
        loop {
            node.next.store(current, Relaxed);

            match self
                .head
                .compare_exchange_weak(current, node, Release, Relaxed, guard)
            {
                Ok(_) => break,
                Err(err) => {
                    current = err.current;
                    node = err.new;
                }
            }
        }
    }

    pub fn iter_with_guard<'a>(&self, guard: &'a Guard) -> Iter<'a, T> {
        let next = Some(self.head.load(Acquire, &guard));
        Iter { next, guard }
    }
}

 struct Node<T> {
    value: T,
    next: Atomic<Node<T>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            next: Atomic::null(),
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<Shared<'a, Node<T>>>,
    guard: &'a Guard,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            Some(node) => {
                let node_ref = unsafe { node.as_ref()? };
                self.next = Some(node_ref.next.load(Acquire, self.guard));
                Some(&node_ref.value)
            }
            None => None,
        }
    }
}

mod test {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_push_correct() {
        let list = Arc::new(LockFreeList::new());
        let ranges = vec![0..100, 100..200];

        let handles = ranges
            .into_iter()
            .map(|range| {
                let list = list.clone();
                std::thread::spawn(move || {
                    for i in range {
                        let _ = list.push_front(i);
                    }
                })
            })
            .collect::<Vec<_>>();

        for handle in handles {
            assert!(handle.join().is_ok());
        }

        let guard = epoch::pin();
        for number in list.iter_with_guard(&guard) {
            assert!(*number < 200, "unexpected number {}", *number);
        }
    }
}
