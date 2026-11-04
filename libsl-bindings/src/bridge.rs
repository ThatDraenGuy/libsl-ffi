use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
    sync::{Arc, Mutex},
};

use libsl::{
    ast,
    file::{CanonicalName, FileLoader, LoadedFile},
    AnnotationId, DeclId, ExprId, FileId, LibSl, PredId, StmtId, TyExprId,
};

#[derive(uniffi::Object)]
pub struct FFILibSl {
    inner: Mutex<LibSl>,
}

#[uniffi::export]
impl FFILibSl {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(LibSl::new()),
        }
    }

    pub fn parse_file(
        &self,
        file_name: String,
        contents: String,
    ) -> Result<FileId, libsl::parse::ParseError> {
        let mut inner = self.inner.lock().unwrap();
        inner.parse_file(file_name, &contents)
    }

    pub fn get_file(&self, id: FileId) -> ast::File {
        let inner = self.inner.lock().unwrap();
        inner.file_by_id(id).clone()
    }

    pub fn get_decls(&self) -> HashMap<DeclId, ast::Decl> {
        let inner = self.inner.lock().unwrap();
        inner
            .decls
            .iter()
            .map(|(id, decl)| (id, decl.clone()))
            .collect()
    }

    pub fn get_ty_exprs(&self) -> HashMap<TyExprId, ast::TyExpr> {
        let inner = self.inner.lock().unwrap();
        inner
            .ty_exprs
            .iter()
            .map(|(id, expr)| (id, expr.clone()))
            .collect()
    }

    pub fn get_exprs(&self) -> HashMap<ExprId, ast::Expr> {
        let inner = self.inner.lock().unwrap();
        inner
            .exprs
            .iter()
            .map(|(id, expr)| (id, expr.clone()))
            .collect()
    }

    pub fn get_stmts(&self) -> HashMap<StmtId, ast::Stmt> {
        let inner = self.inner.lock().unwrap();
        inner
            .stmts
            .iter()
            .map(|(id, expr)| (id, expr.clone()))
            .collect()
    }

    pub fn get_preds(&self) -> HashMap<PredId, ast::Pred> {
        let inner = self.inner.lock().unwrap();
        inner
            .preds
            .iter()
            .map(|(id, expr)| (id, expr.clone()))
            .collect()
    }

    pub fn get_annotations(&self) -> HashMap<AnnotationId, ast::Annotation> {
        let inner = self.inner.lock().unwrap();
        inner
            .annotations
            .iter()
            .map(|(id, expr)| (id, expr.clone()))
            .collect()
    }
}

/// Интерфейс для имплементации во внешнем языке
#[uniffi::export(with_foreign)]
pub trait ForeignFileLoader: Send + Sync {
    fn resolve_canonical_name(&self, path: String) -> Arc<dyn ForeignCanonicalName>;
    fn load(&self, path: Arc<dyn ForeignCanonicalName>) -> String;
}

/// Адаптер внешнего интерфейса [`ForeignFileLoader`]
#[derive(uniffi::Object)]
pub struct FFIFileLoader {
    inner: Arc<dyn ForeignFileLoader>,
    resolved_names: HashMap<String, FFICanonicalName>,
    loaded_files: HashMap<FFICanonicalName, String>,
}

#[uniffi::export]
impl FFIFileLoader {
    #[uniffi::constructor]
    pub fn new(inner: Arc<dyn ForeignFileLoader>) -> Self {
        Self {
            inner,
            resolved_names: HashMap::new(),
            loaded_files: HashMap::new(),
        }
    }
}

impl FileLoader for FFIFileLoader {
    type CanonicalName = FFICanonicalName;

    type Error = String;

    fn load<'a>(
        &'a mut self,
        path: &str,
    ) -> Result<libsl::file::LoadedFile<'a, Self::CanonicalName>, Self::Error> {
        let canonical_name = self
            .resolved_names
            .entry(path.to_owned())
            .or_insert_with(|| {
                let name = self.inner.resolve_canonical_name(path.to_owned());
                FFICanonicalName { inner: name }
            })
            .clone();

        if self.loaded_files.contains_key(&canonical_name) {
            let contents = self.loaded_files.get(&canonical_name).unwrap();
            return Ok(LoadedFile {
                canonical_name,
                contents,
            });
        }
        let res = self.inner.load(canonical_name.clone().inner);
        self.loaded_files.insert(canonical_name.clone(), res);
        let contents = self.loaded_files.get(&canonical_name).unwrap();

        Ok(LoadedFile {
            canonical_name,
            contents,
        })
    }

    fn get<'a>(&'a self, name: &Self::CanonicalName) -> &'a str {
        self.loaded_files.get(name).unwrap()
    }
}

#[uniffi::export(with_foreign)]
pub trait ForeignCanonicalName: Send + Sync {
    fn as_debug_string(&self) -> String;
    fn to_bytes(&self) -> Vec<u8>;
    fn equal(&self, other: Arc<dyn ForeignCanonicalName>) -> bool;
    fn clone(&self) -> Arc<dyn ForeignCanonicalName>;
}

pub struct FFICanonicalName {
    inner: Arc<dyn ForeignCanonicalName>,
}

impl CanonicalName for FFICanonicalName {}
impl Debug for FFICanonicalName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.inner.as_debug_string())
    }
}
impl Display for FFICanonicalName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.inner.as_debug_string())
    }
}
impl Hash for FFICanonicalName {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(&self.inner.to_bytes())
    }
}
impl PartialEq for FFICanonicalName {
    fn eq(&self, other: &Self) -> bool {
        self.inner.equal(other.inner.clone())
    }
}
impl Eq for FFICanonicalName {}
impl Clone for FFICanonicalName {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
