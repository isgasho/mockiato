use super::trait_decl::TraitDecl;
use crate::syntax::ast::{GenericBound, PathSegment};
use crate::syntax::symbol::LocalInternedString;
use crate::syntax_pos::Span;

#[derive(Debug)]
pub(crate) struct TraitBounds<'a>(pub(crate) Vec<TraitBound<'a>>);

#[derive(Debug)]
pub(crate) struct TraitBound<'a> {
    pub(crate) identifier: LocalInternedString,
    pub(crate) segments: &'a [PathSegment],
    pub(crate) span: Span,
}

impl<'a> TraitBounds<'a> {
    pub(crate) fn parse(trait_decl: &'a TraitDecl) -> Self {
        TraitBounds(
            trait_decl
                .generic_bounds
                .iter()
                .filter_map(|generic_bound| {
                    if let GenericBound::Trait(poly_trait_ref, _trait_bound_modifier) =
                        generic_bound
                    {
                        Some(&poly_trait_ref.trait_ref.path)
                    } else {
                        None
                    }
                }).filter_map(|path| {
                    if let Some(last_segment) = path.segments.last() {
                        Some(TraitBound {
                            identifier: last_segment.ident.name.as_str(),
                            segments: &path.segments,
                            span: last_segment.ident.span,
                        })
                    } else {
                        None
                    }
                }).collect(),
        )
    }
}
