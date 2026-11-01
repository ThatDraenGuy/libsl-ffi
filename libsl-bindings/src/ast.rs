pub use libsl::ast::*; //re-export для uniffi-scaffolding
use libsl::{loc::Loc, DeclId, ExprId, TyExprId};

uniffi::custom_type!(DeclKind, FFIDeclKind, {
    remote,
    lower: |kind| kind.into(),
    try_lift: |kind| Ok(kind.into())
});
#[derive(Debug, Default, Clone)]
pub enum FFIDeclKind {
    #[default]
    Dummy,
    Import {
        decl: DeclImport,
    },
    Include {
        decl: DeclInclude,
    },
    SemanticTy {
        decl: DeclSemanticTy,
    },
    TyAlias {
        decl: DeclTyAlias,
    },
    Struct {
        decl: DeclStruct,
    },
    Enum {
        decl: DeclEnum,
    },
    Annotation {
        decl: DeclAnnotation,
    },
    Action {
        decl: DeclAction,
    },
    Automaton {
        decl: DeclAutomaton,
    },
    Function {
        decl: DeclFunction,
    },
    Variable {
        decl: DeclVariable,
    },
    State {
        decl: DeclState,
    },
    Shift {
        decl: DeclShift,
    },
    Constructor {
        decl: DeclConstructor,
    },
    Destructor {
        decl: DeclDestructor,
    },
    Proc {
        decl: DeclProc,
    },
    Pred {
        decl: DeclPred,
    },
}
impl From<DeclKind> for FFIDeclKind {
    fn from(value: DeclKind) -> Self {
        match value {
            DeclKind::Dummy => Self::Dummy,
            DeclKind::Import(decl) => Self::Import { decl },
            DeclKind::Include(decl) => Self::Include { decl },
            DeclKind::SemanticTy(decl) => Self::SemanticTy { decl },
            DeclKind::TyAlias(decl) => Self::TyAlias { decl },
            DeclKind::Struct(decl) => Self::Struct { decl },
            DeclKind::Enum(decl) => Self::Enum { decl },
            DeclKind::Annotation(decl) => Self::Annotation { decl },
            DeclKind::Action(decl) => Self::Action { decl },
            DeclKind::Automaton(decl) => Self::Automaton { decl },
            DeclKind::Function(decl) => Self::Function { decl },
            DeclKind::Variable(decl) => Self::Variable { decl },
            DeclKind::State(decl) => Self::State { decl },
            DeclKind::Shift(decl) => Self::Shift { decl },
            DeclKind::Constructor(decl) => Self::Constructor { decl },
            DeclKind::Destructor(decl) => Self::Destructor { decl },
            DeclKind::Proc(decl) => Self::Proc { decl },
            DeclKind::Pred(decl) => Self::Pred { decl },
        }
    }
}
impl From<FFIDeclKind> for DeclKind {
    fn from(value: FFIDeclKind) -> Self {
        match value {
            FFIDeclKind::Dummy => Self::Dummy,
            FFIDeclKind::Import { decl } => Self::Import(decl),
            FFIDeclKind::Include { decl } => Self::Include(decl),
            FFIDeclKind::SemanticTy { decl } => Self::SemanticTy(decl),
            FFIDeclKind::TyAlias { decl } => Self::TyAlias(decl),
            FFIDeclKind::Struct { decl } => Self::Struct(decl),
            FFIDeclKind::Enum { decl } => Self::Enum(decl),
            FFIDeclKind::Annotation { decl } => Self::Annotation(decl),
            FFIDeclKind::Action { decl } => Self::Action(decl),
            FFIDeclKind::Automaton { decl } => Self::Automaton(decl),
            FFIDeclKind::Function { decl } => Self::Function(decl),
            FFIDeclKind::Variable { decl } => Self::Variable(decl),
            FFIDeclKind::State { decl } => Self::State(decl),
            FFIDeclKind::Shift { decl } => Self::Shift(decl),
            FFIDeclKind::Constructor { decl } => Self::Constructor(decl),
            FFIDeclKind::Destructor { decl } => Self::Destructor(decl),
            FFIDeclKind::Proc { decl } => Self::Proc(decl),
            FFIDeclKind::Pred { decl } => Self::Pred(decl),
        }
    }
}

uniffi::custom_type!(SemanticTyKind, FFISemanticTyKind, {
    remote,
    lower: |kind| kind.into(),
    try_lift: |kind| Ok(kind.into())
});
#[derive(Debug, Default, Clone)]
pub enum FFISemanticTyKind {
    /// A simple semantic type.
    #[default]
    Simple,

    /// A semantic type with enumerated named values.
    Enumerated { values: Vec<SemanticTyEnumValue> },
}
impl From<SemanticTyKind> for FFISemanticTyKind {
    fn from(value: SemanticTyKind) -> Self {
        match value {
            SemanticTyKind::Simple => Self::Simple,
            SemanticTyKind::Enumerated(values) => Self::Enumerated { values },
        }
    }
}
impl From<FFISemanticTyKind> for SemanticTyKind {
    fn from(value: FFISemanticTyKind) -> Self {
        match value {
            FFISemanticTyKind::Simple => Self::Simple,
            FFISemanticTyKind::Enumerated { values } => Self::Enumerated(values),
        }
    }
}

uniffi::custom_type!(Contract, FFIContract, {
    remote,
    lower: |contract| contract.into(),
    try_lift: |contract| Ok(contract.into())
});
#[derive(Debug, Clone)]
pub enum FFIContract {
    Requires { contract: ContractRequires },
    Ensures { contract: ContractEnsures },
    Assigns { contract: ContractAssigns },
}
impl From<Contract> for FFIContract {
    fn from(value: Contract) -> Self {
        match value {
            Contract::Requires(contract) => Self::Requires { contract },
            Contract::Ensures(contract) => Self::Ensures { contract },
            Contract::Assigns(contract) => Self::Assigns { contract },
        }
    }
}
impl From<FFIContract> for Contract {
    fn from(value: FFIContract) -> Self {
        match value {
            FFIContract::Requires { contract } => Self::Requires(contract),
            FFIContract::Ensures { contract } => Self::Ensures(contract),
            FFIContract::Assigns { contract } => Self::Assigns(contract),
        }
    }
}

uniffi::custom_type!(TyExprKind, FFITyExprKind, {
    remote,
    lower: |kind| kind.into(),
    try_lift: |kind| Ok(kind.into())
});
#[derive(Debug, Default, Clone)]
pub enum FFITyExprKind {
    #[default]
    Dummy,
    PrimitiveLit {
        expr: TyExprPrimitiveLit,
    },
    Name {
        expr: TyExprName,
    },
    Pointer {
        expr: TyExprPointer,
    },
    Intersection {
        expr: TyExprIntersection,
    },
    Union {
        expr: TyExprUnion,
    },
}
impl From<TyExprKind> for FFITyExprKind {
    fn from(value: TyExprKind) -> Self {
        match value {
            TyExprKind::Dummy => Self::Dummy,
            TyExprKind::PrimitiveLit(expr) => Self::PrimitiveLit { expr },
            TyExprKind::Name(expr) => Self::Name { expr },
            TyExprKind::Pointer(expr) => Self::Pointer { expr },
            TyExprKind::Intersection(expr) => Self::Intersection { expr },
            TyExprKind::Union(expr) => Self::Union { expr },
        }
    }
}
impl From<FFITyExprKind> for TyExprKind {
    fn from(value: FFITyExprKind) -> Self {
        match value {
            FFITyExprKind::Dummy => Self::Dummy,
            FFITyExprKind::PrimitiveLit { expr } => Self::PrimitiveLit(expr),
            FFITyExprKind::Name { expr } => Self::Name(expr),
            FFITyExprKind::Pointer { expr } => Self::Pointer(expr),
            FFITyExprKind::Intersection { expr } => Self::Intersection(expr),
            FFITyExprKind::Union { expr } => Self::Union(expr),
        }
    }
}

uniffi::custom_type!(PredKind, FFIPredKind, {
    remote,
    lower: |kind| kind.into(),
    try_lift: |kind| Ok(kind.into())
});
#[derive(Debug, Default, Clone)]
pub enum FFIPredKind {
    #[default]
    Dummy,
    Block {
        pred: PredBlock,
    },
    Named {
        pred: PredNamed,
    },
    Decl {
        pred: DeclId,
    },
    If {
        pred: PredIf,
    },
    Expr {
        pred: ExprId,
    },
}
impl From<PredKind> for FFIPredKind {
    fn from(value: PredKind) -> Self {
        match value {
            PredKind::Dummy => Self::Dummy,
            PredKind::Block(pred) => Self::Block { pred },
            PredKind::Named(pred) => Self::Named { pred },
            PredKind::Decl(pred) => Self::Decl { pred },
            PredKind::If(pred) => Self::If { pred },
            PredKind::Expr(pred) => Self::Expr { pred },
        }
    }
}
impl From<FFIPredKind> for PredKind {
    fn from(value: FFIPredKind) -> Self {
        match value {
            FFIPredKind::Dummy => Self::Dummy,
            FFIPredKind::Block { pred } => Self::Block(pred),
            FFIPredKind::Named { pred } => Self::Named(pred),
            FFIPredKind::Decl { pred } => Self::Decl(pred),
            FFIPredKind::If { pred } => Self::If(pred),
            FFIPredKind::Expr { pred } => Self::Expr(pred),
        }
    }
}

uniffi::custom_type!(StmtKind, FFIStmtKind, {
    remote,
    lower: |kind| kind.into(),
    try_lift: |kind| Ok(kind.into())
});
#[derive(Debug, Default, Clone)]
pub enum FFIStmtKind {
    #[default]
    Dummy,
    Decl {
        stmt: DeclId,
    },
    If {
        stmt: StmtIf,
    },
    Assign {
        stmt: StmtAssign,
    },
    Cancel {
        stmt: StmtCancel,
    },
    Expr {
        stmt: ExprId,
    },
}
impl From<StmtKind> for FFIStmtKind {
    fn from(value: StmtKind) -> Self {
        match value {
            StmtKind::Dummy => Self::Dummy,
            StmtKind::Decl(stmt) => Self::Decl { stmt },
            StmtKind::If(stmt) => Self::If { stmt },
            StmtKind::Assign(stmt) => Self::Assign { stmt },
            StmtKind::Cancel(stmt) => Self::Cancel { stmt },
            StmtKind::Expr(stmt) => Self::Expr { stmt },
        }
    }
}
impl From<FFIStmtKind> for StmtKind {
    fn from(value: FFIStmtKind) -> Self {
        match value {
            FFIStmtKind::Dummy => Self::Dummy,
            FFIStmtKind::Decl { stmt } => Self::Decl(stmt),
            FFIStmtKind::If { stmt } => Self::If(stmt),
            FFIStmtKind::Assign { stmt } => Self::Assign(stmt),
            FFIStmtKind::Cancel { stmt } => Self::Cancel(stmt),
            FFIStmtKind::Expr { stmt } => Self::Expr(stmt),
        }
    }
}

uniffi::custom_type!(ExprKind, FFIExprKind, {
    remote,
    lower: |kind| kind.into(),
    try_lift: |kind| Ok(kind.into())
});
#[derive(Debug, Default, Clone)]
pub enum FFIExprKind {
    #[default]
    Dummy,
    PrimitiveLit {
        expr: ExprPrimitiveLit,
    },
    ArrayLit {
        expr: ExprArrayLit,
    },
    SetLit {
        expr: ExprSetLit,
    },
    ProcCall {
        expr: ExprProcCall,
    },
    ActionCall {
        expr: ExprActionCall,
    },
    Instantiate {
        expr: ExprInstantiate,
    },
    Name {
        expr: ExprName,
    },
    Prev {
        expr: ExprPrev,
    },
    Field {
        expr: ExprField,
    },
    Deref {
        expr: ExprDeref,
    },
    Index {
        expr: ExprIndex,
    },
    HasConcept {
        expr: ExprHasConcept,
    },
    Cast {
        expr: ExprCast,
    },
    TyCompare {
        expr: ExprTyCompare,
    },
    Unary {
        expr: ExprUnary,
    },
    Binary {
        expr: ExprBinary,
    },
}
impl From<ExprKind> for FFIExprKind {
    fn from(value: ExprKind) -> Self {
        match value {
            ExprKind::Dummy => Self::Dummy,
            ExprKind::PrimitiveLit(expr) => Self::PrimitiveLit { expr },
            ExprKind::ArrayLit(expr) => Self::ArrayLit { expr },
            ExprKind::SetLit(expr) => Self::SetLit { expr },
            ExprKind::ProcCall(expr) => Self::ProcCall { expr },
            ExprKind::ActionCall(expr) => Self::ActionCall { expr },
            ExprKind::Instantiate(expr) => Self::Instantiate { expr },
            ExprKind::Name(expr) => Self::Name { expr },
            ExprKind::Prev(expr) => Self::Prev { expr },
            ExprKind::Field(expr) => Self::Field { expr },
            ExprKind::Deref(expr) => Self::Deref { expr },
            ExprKind::Index(expr) => Self::Index { expr },
            ExprKind::HasConcept(expr) => Self::HasConcept { expr },
            ExprKind::Cast(expr) => Self::Cast { expr },
            ExprKind::TyCompare(expr) => Self::TyCompare { expr },
            ExprKind::Unary(expr) => Self::Unary { expr },
            ExprKind::Binary(expr) => Self::Binary { expr },
        }
    }
}
impl From<FFIExprKind> for ExprKind {
    fn from(value: FFIExprKind) -> Self {
        match value {
            FFIExprKind::Dummy => Self::Dummy,
            FFIExprKind::PrimitiveLit { expr } => Self::PrimitiveLit(expr),
            FFIExprKind::ArrayLit { expr } => Self::ArrayLit(expr),
            FFIExprKind::SetLit { expr } => Self::SetLit(expr),
            FFIExprKind::ProcCall { expr } => Self::ProcCall(expr),
            FFIExprKind::ActionCall { expr } => Self::ActionCall(expr),
            FFIExprKind::Instantiate { expr } => Self::Instantiate(expr),
            FFIExprKind::Name { expr } => Self::Name(expr),
            FFIExprKind::Prev { expr } => Self::Prev(expr),
            FFIExprKind::Field { expr } => Self::Field(expr),
            FFIExprKind::Deref { expr } => Self::Deref(expr),
            FFIExprKind::Index { expr } => Self::Index(expr),
            FFIExprKind::HasConcept { expr } => Self::HasConcept(expr),
            FFIExprKind::Cast { expr } => Self::Cast(expr),
            FFIExprKind::TyCompare { expr } => Self::TyCompare(expr),
            FFIExprKind::Unary { expr } => Self::Unary(expr),
            FFIExprKind::Binary { expr } => Self::Binary(expr),
        }
    }
}

uniffi::custom_type!(TyArg, FFITyArg, {
    remote,
    lower: |arg| arg.into(),
    try_lift: |arg| Ok(arg.into())
});
#[derive(Debug, Clone)]
pub enum FFITyArg {
    TyExpr {
        variance: Option<Variance>,
        ty_expr: TyExprId,
    },
    Wildcard {
        loc: Loc,
    },
}
impl From<TyArg> for FFITyArg {
    fn from(value: TyArg) -> Self {
        match value {
            TyArg::TyExpr(variance, ty_expr) => Self::TyExpr { variance, ty_expr },
            TyArg::Wildcard(loc) => Self::Wildcard { loc },
        }
    }
}
impl From<FFITyArg> for TyArg {
    fn from(value: FFITyArg) -> Self {
        match value {
            FFITyArg::TyExpr { variance, ty_expr } => Self::TyExpr(variance, ty_expr),
            FFITyArg::Wildcard { loc } => Self::Wildcard(loc),
        }
    }
}

uniffi::custom_type!(ConstructorArg, FFIConstructorArg, {
    remote,
    lower: |arg| arg.into(),
    try_lift: |arg| Ok(arg.into())
});
#[derive(Debug, Clone)]
pub enum FFIConstructorArg {
    State { loc: Loc, name: Name },
    Var { loc: Loc, name: Name, expr: ExprId },
}
impl From<ConstructorArg> for FFIConstructorArg {
    fn from(value: ConstructorArg) -> Self {
        match value {
            ConstructorArg::State(loc, name) => Self::State { loc, name },
            ConstructorArg::Var(loc, name, expr) => Self::Var { loc, name, expr },
        }
    }
}
impl From<FFIConstructorArg> for ConstructorArg {
    fn from(value: FFIConstructorArg) -> Self {
        match value {
            FFIConstructorArg::State { loc, name } => Self::State(loc, name),
            FFIConstructorArg::Var { loc, name, expr } => Self::Var(loc, name, expr),
        }
    }
}

uniffi::custom_type!(PrimitiveLit, FFIPrimitiveLit, {
    remote,
    lower: |lit| lit.into(),
    try_lift: |lit| Ok(lit.into())
});
#[derive(Debug, Clone)]
pub enum FFIPrimitiveLit {
    Int { lit: IntLit },
    Float { lit: FloatLit },
    String { lit: String },
    Char { lit: u32 },
    Bool { lit: bool },
    Null,
}
impl From<PrimitiveLit> for FFIPrimitiveLit {
    fn from(value: PrimitiveLit) -> Self {
        match value {
            PrimitiveLit::Int(lit) => Self::Int { lit },
            PrimitiveLit::Float(lit) => Self::Float { lit },
            PrimitiveLit::String(lit) => Self::String { lit },
            PrimitiveLit::Char(lit) => Self::Char { lit },
            PrimitiveLit::Bool(lit) => Self::Bool { lit },
            PrimitiveLit::Null => Self::Null,
        }
    }
}
impl From<FFIPrimitiveLit> for PrimitiveLit {
    fn from(value: FFIPrimitiveLit) -> Self {
        match value {
            FFIPrimitiveLit::Int { lit } => Self::Int(lit),
            FFIPrimitiveLit::Float { lit } => Self::Float(lit),
            FFIPrimitiveLit::String { lit } => Self::String(lit),
            FFIPrimitiveLit::Char { lit } => Self::Char(lit),
            FFIPrimitiveLit::Bool { lit } => Self::Bool(lit),
            FFIPrimitiveLit::Null => Self::Null,
        }
    }
}

uniffi::custom_type!(IntLit, FFIIntLit, {
    remote,
    lower: |lit| lit.into(),
    try_lift: |lit| Ok(lit.into())
});
#[derive(Debug, Clone, Copy)]
pub enum FFIIntLit {
    I8 { num: i8 },
    U8 { num: u8 },
    I16 { num: i16 },
    U16 { num: u16 },
    I32 { num: i32 },
    U32 { num: u32 },
    I64 { num: i64 },
    U64 { num: u64 },
}
impl From<IntLit> for FFIIntLit {
    fn from(value: IntLit) -> Self {
        match value {
            IntLit::I8(num) => Self::I8 { num },
            IntLit::U8(num) => Self::U8 { num },
            IntLit::I16(num) => Self::I16 { num },
            IntLit::U16(num) => Self::U16 { num },
            IntLit::I32(num) => Self::I32 { num },
            IntLit::U32(num) => Self::U32 { num },
            IntLit::I64(num) => Self::I64 { num },
            IntLit::U64(num) => Self::U64 { num },
        }
    }
}
impl From<FFIIntLit> for IntLit {
    fn from(value: FFIIntLit) -> Self {
        match value {
            FFIIntLit::I8 { num } => Self::I8(num),
            FFIIntLit::U8 { num } => Self::U8(num),
            FFIIntLit::I16 { num } => Self::I16(num),
            FFIIntLit::U16 { num } => Self::U16(num),
            FFIIntLit::I32 { num } => Self::I32(num),
            FFIIntLit::U32 { num } => Self::U32(num),
            FFIIntLit::I64 { num } => Self::I64(num),
            FFIIntLit::U64 { num } => Self::U64(num),
        }
    }
}

uniffi::custom_type!(FloatLit, FFIFloatLit, {
    remote,
    lower: |lit| lit.into(),
    try_lift: |lit| Ok(lit.into())
});
#[derive(Debug, Clone, Copy)]
pub enum FFIFloatLit {
    F32 { num: f32 },
    F64 { num: f64 },
}
impl From<FloatLit> for FFIFloatLit {
    fn from(value: FloatLit) -> Self {
        match value {
            FloatLit::F32(num) => Self::F32 { num },
            FloatLit::F64(num) => Self::F64 { num },
        }
    }
}
impl From<FFIFloatLit> for FloatLit {
    fn from(value: FFIFloatLit) -> Self {
        match value {
            FFIFloatLit::F32 { num } => Self::F32(num),
            FFIFloatLit::F64 { num } => Self::F64(num),
        }
    }
}
