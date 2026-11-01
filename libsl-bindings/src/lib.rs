pub mod ast;
pub mod loc;

use ast::*;
use libsl::{AnnotationId, DeclId, ExprId, FileId, PredId, StmtId, TyExprId};
use loc::*;
use slotmap::{Key, KeyData};
use std::num::NonZeroUsize;

uniffi::custom_type!(usize, u64, {
    remote,
    lower: |num| num as u64,
    try_lift: |num| Ok(num as usize)
});

uniffi::custom_type!(NonZeroUsize, u64, {
    remote,
    lower: |num| num.get() as u64,
    try_lift: |num| Ok(NonZeroUsize::try_from(num as usize)?)
});

uniffi::custom_type!(FileId, u64, {
    remote,
    lower: |id| id.data().as_ffi(),
    try_lift: |num| Ok(KeyData::from_ffi(num).into())
});

uniffi::custom_type!(DeclId, u64, {
    remote,
    lower: |id| id.data().as_ffi(),
    try_lift: |num| Ok(KeyData::from_ffi(num).into())
});

uniffi::custom_type!(TyExprId, u64, {
    remote,
    lower: |id| id.data().as_ffi(),
    try_lift: |num| Ok(KeyData::from_ffi(num).into())
});

uniffi::custom_type!(ExprId, u64, {
    remote,
    lower: |id| id.data().as_ffi(),
    try_lift: |num| Ok(KeyData::from_ffi(num).into())
});

uniffi::custom_type!(StmtId, u64, {
    remote,
    lower: |id| id.data().as_ffi(),
    try_lift: |num| Ok(KeyData::from_ffi(num).into())
});

uniffi::custom_type!(PredId, u64, {
    remote,
    lower: |id| id.data().as_ffi(),
    try_lift: |num| Ok(KeyData::from_ffi(num).into())
});

uniffi::custom_type!(AnnotationId, u64, {
    remote,
    lower: |id| id.data().as_ffi(),
    try_lift: |num| Ok(KeyData::from_ffi(num).into())
});

uniffi::include_scaffolding!("libsl");

// use slotmap::Key;
// use std::error::Error;

// use libsl::{
//     diag::DummyDiagCtx,
//     file::{self, FsFileLoader},
//     sema::ImportCtx,
//     FileId,
// };

// fn test() -> Result<(), Box<dyn Error>> {
//     let mut libsl = libsl::LibSl::new();
//     let mut file_loader = FsFileLoader::new("test".into())?;
//     let import_ctx = ImportCtx::new(&mut libsl, &mut file_loader);
//     let mut diag = DummyDiagCtx {};
//     let mut sema = import_ctx.into_sema();
//     sema.analyze(&mut diag)?;

//     Ok(())
// }
