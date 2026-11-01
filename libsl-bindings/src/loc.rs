pub use libsl::loc::*; //re-export для uniffi-scaffolding

uniffi::custom_type!(Loc, FFILoc, {
    remote,
    lower: |loc| loc.into(),
    try_lift: |loc| Ok(loc.into())
});

#[uniffi::export]
pub fn wrap(span: Span) -> Loc {
    Loc::Span(span)
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum FFILoc {
    #[default]
    Synthetic,
    LocSpan {
        span: Span,
    },
}

impl From<Loc> for FFILoc {
    fn from(value: Loc) -> Self {
        match value {
            Loc::Synthetic => Self::Synthetic,
            Loc::Span(span) => Self::LocSpan { span },
        }
    }
}
impl From<FFILoc> for Loc {
    fn from(value: FFILoc) -> Self {
        match value {
            FFILoc::Synthetic => Self::Synthetic,
            FFILoc::LocSpan { span } => Self::Span(span),
        }
    }
}
