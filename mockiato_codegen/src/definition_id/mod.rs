use crate::rustc::hir::def_id;
use crate::rustc_resolve::Resolver as ResolverImpl;
use crate::syntax::ext::base::Resolver as SyntaxResolver;

mod resolver;

pub(crate) use self::resolver::{ContextResolver, Resolver};

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub(crate) struct DefId(pub(crate) def_id::DefId);

#[cfg(test)]
impl DefId {
    pub(crate) fn dummy(value: u32) -> Self {
        DefId(def_id::DefId::local(def_id::DefIndex::from_raw_u32(value)))
    }
}

fn transmute_resolver(mut resolver: &mut SyntaxResolver) -> &mut &mut ResolverImpl {
    // Behold — The mighty transmutation
    unsafe { &mut *(&mut resolver as *mut &mut dyn SyntaxResolver as *mut &mut ResolverImpl) }
}