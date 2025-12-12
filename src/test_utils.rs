use once_cell::sync::Lazy;
use std::sync::{Mutex, MutexGuard};

// garde context unique par test => voir les tests de resolvers
pub static TEST_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

pub fn lock_test() -> MutexGuard<'static, ()> {
    TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner())
}
