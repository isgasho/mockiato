use std::sync::{Arc, RwLock, RwLockWriteGuard};
use syntax::ext::base::ExtCtxt;

#[derive(Clone)]
pub(crate) struct Context<'a, 'b: 'a>(Arc<RwLock<&'a mut ExtCtxt<'b>>>);

impl<'a, 'b: 'a> Context<'a, 'b> {
    pub(crate) fn new(inner: &'a mut ExtCtxt<'b>) -> Self {
        Context(Arc::new(RwLock::new(inner)))
    }

    pub(crate) fn into_inner(&self) -> RwLockWriteGuard<&'a mut ExtCtxt<'b>> {
        self.0.write().unwrap()
    }
}
