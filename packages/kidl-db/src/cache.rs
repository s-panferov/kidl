use std::sync::{Arc, Mutex, MutexGuard};

use rowan::NodeCache;

#[derive(Debug, Default)]
pub struct CacheInner {
    node: Mutex<NodeCache>,
}

#[derive(Debug, Clone, Default)]
pub struct Cache {
    inner: Arc<CacheInner>,
}

impl Cache {
    pub fn node(&self) -> MutexGuard<NodeCache> {
        self.inner.node.lock().unwrap()
    }
}

impl std::hash::Hash for Cache {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.inner).hash(state)
    }
}

impl PartialEq for Cache {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

impl Eq for Cache {}
