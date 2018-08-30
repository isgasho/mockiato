use crate::parse::trait_decl::TraitDecl;
use crate::path_resolver::DefId;

pub(crate) trait TraitBoundResolver {
    fn register_mocked_trait<'a>(&mut self, identifier: DefId, mocked_trait: &TraitDecl<'a>);
    fn resolve_trait_bound<'a>(&self, identifier: &'a str) -> Option<TraitBound<'a>>;
}

pub(crate) enum TraitBound<'a> {
    Derivable(String),
    AlreadyMockedTrait(TraitDecl<'a>),
}
