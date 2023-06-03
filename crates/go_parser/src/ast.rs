#[derive(Debug, Clone)]
pub enum Node {
    ConstDecl(ConstDecl),
    Declaration(Declaration),
    ExpressionStmt(ExpressionStmt),
    FunctionDecl(FunctionDecl),
    PrimaryExpr(PrimaryExpr),
    ShortVarDecl(ShortVarDecl),
    SimpleStmt(SimpleStmt),
    SourceFile(SourceFile),
    Statement(Statement),
    TopLevelDecl(TopLevelDecl),
    VarDecl(VarDecl),
}

pub type Identifier = String;

#[derive(Debug, Clone)]
pub struct Type {
    pub type_name: TypeName,
}

#[derive(Debug, Clone)]
pub struct TypeName {
    pub identifier: Identifier,
}

#[derive(Debug, Clone)]
pub struct Declaration {
    pub const_decl: Option<ConstDecl>,
    pub type_decl: Option<TypeDecl>,
    pub var_decl: Option<VarDecl>,
}

#[derive(Debug, Clone)]
pub struct TopLevelDecl {
    pub declaration: Option<Declaration>,
    pub function_decl: Option<FunctionDecl>,
    pub method_decl: Option<MethodDecl>,
}

#[derive(Debug, Clone)]
pub struct FunctionDecl {
    pub function_name: FunctionName,
    pub signature: Signature,
    pub function_body: FunctionBody,
}

#[derive(Debug, Clone)]
pub struct FunctionName {
    pub identifier: Identifier,
}

#[derive(Debug, Clone)]
pub struct MethodDecl {
    pub receiver: Receiver,
    pub method_name: MethodName,
    pub signature: Signature,
    pub function_body: FunctionBody,
}

#[derive(Debug, Clone)]
pub struct Receiver {
    pub parameters: Parameters,
}

#[derive(Debug, Clone)]
pub struct Signature {
    pub parameters: Parameters,
}

#[derive(Debug, Clone)]
pub struct Parameters {
    pub parameter_list: ParameterList,
}

#[derive(Debug, Clone)]
pub struct ParameterList {
    pub parameter_decls: Vec<ParameterDecl>,
}

#[derive(Debug, Clone)]
pub struct ParameterDecl {
    pub _type: Type,
}

#[derive(Debug, Clone)]
pub struct FunctionBody {
    pub block: Block,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statement_list: StatementList,
}

#[derive(Debug, Clone)]
pub struct StatementList {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct Statement {
    pub declaration: Option<Declaration>,
    pub simple_stmt: Option<SimpleStmt>,
    pub if_stmt: Option<IfStmt>,
}

#[derive(Debug, Clone)]
pub struct SimpleStmt {
    pub expression_stmt: Option<ExpressionStmt>,
    pub short_var_decl: Option<ShortVarDecl>,
}

#[derive(Debug, Clone)]
pub struct ExpressionStmt {
    pub expression: Expression,
}

#[derive(Debug, Clone)]
pub struct ShortVarDecl {
    pub identifier_list: Vec<Identifier>,
    pub expression_list: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct IfStmt {
    pub expression: Expression,
    pub block: Block,
}

#[derive(Debug, Clone)]
pub struct ConstDecl {
    pub const_spec: ConstSpec,
}

#[derive(Debug, Clone)]
pub struct ConstSpec {
    pub identifier_list: Vec<Identifier>,
    pub expression_list: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct TypeDecl {
    pub type_spec: TypeSpec,
}

#[derive(Debug, Clone)]
pub struct TypeSpec {
    pub type_def: TypeDef,
}

#[derive(Debug, Clone)]
pub struct TypeDef {
    pub identifier: Identifier,
    pub _type: Type,
}

#[derive(Debug, Clone)]
pub struct VarDecl {
    pub var_spec: VarSpec,
}

#[derive(Debug, Clone)]
pub struct VarSpec {
    pub identifier_list: Vec<Identifier>,
    pub expression_list: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub unary_expr: Option<UnaryExpr>,
    pub expression1: Option<Box<Expression>>,
    pub expression2: Option<Box<Expression>>,
}

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub primary_expr: PrimaryExpr,
}

#[derive(Debug, Clone)]
pub struct PrimaryExpr {
    pub operand: Option<Operand>,
    pub method_expr: Option<MethodExpr>,
    pub selector: Option<Selector>,
    pub arguments: Option<Arguments>,
}

#[derive(Debug, Clone)]
pub struct Operand {
    pub loc: usize,
    pub literal: Option<Literal>,
    pub operand_name: Option<OperandName>,
}

#[derive(Debug, Clone)]
pub struct Literal {
    pub basic_lit: BasicLit,
}

#[derive(Debug, Clone)]
pub struct BasicLit {
    pub string_lit: StringLit,
}

#[derive(Debug, Clone)]
pub struct StringLit {
    pub raw_string_lit: Option<RawStringLit>,
    pub interpreted_string_lit: Option<InterpretedStringLit>,
}

pub type RawStringLit = String;

pub type InterpretedStringLit = String;

#[derive(Debug, Clone)]
pub struct OperandName {
    pub identifier: Identifier,
}

#[derive(Debug, Clone)]
pub struct Arguments {
    pub expression_list: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct MethodExpr {
    pub receiver_type: ReceiverType,
    pub method_name: MethodName,
}

#[derive(Debug, Clone)]
pub struct ReceiverType {
    pub _type: Type,
}

pub type MethodName = Identifier;

#[derive(Debug, Clone)]
pub struct Selector {
    pub identifier: Identifier,
}

#[derive(Debug, Clone)]
pub struct SourceFile {
    pub package_clause: PackageClause,
    pub import_decl: ImportDecl,
    pub top_level_decls: Vec<TopLevelDecl>,
}

#[derive(Debug, Clone)]
pub struct PackageClause {
    pub package_name: PackageName,
}

pub type PackageName = Identifier;

#[derive(Debug, Clone)]
pub struct ImportDecl {
    pub import_spec: Vec<ImportSpec>,
}

#[derive(Debug, Clone)]
pub struct ImportSpec {
    pub import_path: ImportPath,
}

#[derive(Debug, Clone)]
pub struct ImportPath {
    pub string_lit: StringLit,
}
