use std::sync::atomic::{AtomicI32, Ordering};

static mut COUNTER: AtomicI32 = AtomicI32::new(0);

pub unsafe fn fresh_name(prefix: &str) -> String {
    let i = COUNTER.fetch_add(1, Ordering::SeqCst);
    return format!("{}_{}", prefix, i)
}
