use std::sync::atomic::{AtomicUsize, Ordering};

static BY_ATOMIC_SINGLETON: AtomicUsize = AtomicUsize::new(0);

#[cfg(test)]
#[test]
fn test_by_atomic() {
    BY_ATOMIC_SINGLETON.fetch_add(1, Ordering::SeqCst);
    assert_eq!(1, BY_ATOMIC_SINGLETON.load(Ordering::SeqCst));
}
