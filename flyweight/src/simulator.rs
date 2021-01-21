use crate::{thread, AtomicU64, Duration, Id, Ordering};

pub struct Redis {
    stock_used: &'static AtomicU64,
}

impl Redis {
    pub fn new() -> Self {
        static STOCK_USED: AtomicU64 = AtomicU64::new(0);
        thread::spawn(move || loop {
            STOCK_USED.fetch_add(1, Ordering::Relaxed);
            thread::sleep(Duration::new(1, 0));
        });
        Self {
            stock_used: &STOCK_USED,
        }
    }

    pub fn get_stock_used(&self, _id: Id) -> u64 {
        self.stock_used.load(Ordering::Relaxed)
    }
}
