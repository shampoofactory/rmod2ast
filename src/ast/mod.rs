use std::{iter::Enumerate, ops::Sub};

// PIM

// PIM syntax 1
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Ident<'a>(pub &'a str);

// PIM syntax 2
// Refactored.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Number<'a> {
    Oct(&'a str),
    Dec(&'a str),
    Hex(&'a str),
    Real(&'a str),
}

// PIM syntax 10
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct QStr<'a>(pub &'a str);

// PIM syntax 11
#[derive(Debug, PartialEq, Eq)]
pub struct Qualident<'a>(pub Vec<Ident<'a>>);

// PIM syntax 12
#[derive(Debug, PartialEq, Eq)]
pub struct ConstantDeclaration<'a>(pub Ident<'a>, pub ConstExpression<'a>);

// PIM syntax 13
#[derive(Debug, PartialEq, Eq)]
pub struct ConstExpression<'a>(
    pub SimpleConstExpr<'a>,
    pub Option<(Relation, SimpleConstExpr<'a>)>,
);

// PIM syntax 14
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Relation {
    EQ,
    HASH,
    NE,
    LT,
    LTE,
    GT,
    GTE,
    IN,
}

// PIM syntax 15
#[derive(Debug, PartialEq, Eq)]
pub struct SimpleConstExpr<'a>(
    pub AddSub,
    pub ConstTerm<'a>,
    pub Vec<(AddOperator, ConstTerm<'a>)>,
);

// PIM syntax 16
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AddOperator {
    ADD,
    SUB,
    OR,
}

// PIM syntax 17
#[derive(Debug, PartialEq, Eq)]
pub struct ConstTerm<'a>(pub ConstFactor<'a>, pub Vec<(MulOperator, ConstFactor<'a>)>);

// PIM syntax 18
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MulOperator {
    MUL,
    DIV,
    MOD,
    AND,
}

// PIM syntax 19
#[derive(Debug, PartialEq, Eq)]
pub enum ConstFactor<'a> {
    Qualident(Qualident<'a>),
    Num(Number<'a>),
    QStr(QStr<'a>),
    Set(Set<'a>),
    ConstantExpression(Box<ConstExpression<'a>>),
    Not(Box<ConstFactor<'a>>),
}

// PIM syntax 21
#[derive(Debug, PartialEq, Eq)]
pub struct Set<'a>(pub Option<Qualident<'a>>, pub Vec<Element<'a>>);

// PIM syntax 22
#[derive(Debug, PartialEq, Eq)]
pub enum Element<'a> {
    Item(ConstExpression<'a>),
    Range(ConstExpression<'a>, ConstExpression<'a>),
}

// PIM syntax 23
#[derive(Debug, PartialEq, Eq)]
pub struct TypeDeclaration<'a>(pub Ident<'a>, pub Type<'a>);

// PIM syntax 24
#[derive(Debug, PartialEq, Eq)]
pub enum Type<'a> {
    SimpleType(SimpleType<'a>),
    ArrayType(ArrayType<'a>),
    RecordType(RecordType<'a>),
    SetType(SetType<'a>),
    PointerType(PointerType<'a>),
    ProcedureType(ProcedureType<'a>),
}

// PIM syntax 27
#[derive(Debug, PartialEq, Eq)]
pub enum SimpleType<'a> {
    Qualident(Qualident<'a>),
    Enumeration(Enumeration<'a>),
    SubrangeType(SubrangeType<'a>),
}

// PIM syntax 27
#[derive(Debug, PartialEq, Eq)]
pub struct Enumeration<'a>(pub IdentList<'a>);

// PIM syntax 28
#[derive(Debug, PartialEq, Eq)]
pub struct IdentList<'a>(pub Vec<Ident<'a>>);

// PIM syntax 29
#[derive(Debug, PartialEq, Eq)]
pub struct SubrangeType<'a>(pub ConstExpression<'a>, pub ConstExpression<'a>);

// PIM syntax 30
#[derive(Debug, PartialEq, Eq)]
pub struct ArrayType<'a>(pub Vec<SimpleType<'a>>, pub Box<Type<'a>>);

// PIM syntax 31
#[derive(Debug, PartialEq, Eq)]
pub struct RecordType<'a>(pub FieldListSequence<'a>);

// PIM syntax 32
#[derive(Debug, PartialEq, Eq, Default)]
pub struct FieldListSequence<'a>(pub Vec<FieldList<'a>>);

// PIM syntax 33
#[derive(Debug, PartialEq, Eq)]
pub enum FieldList<'a> {
    Ident(IdentList<'a>, Type<'a>),
    Case(Option<Ident<'a>>, Qualident<'a>, Vec<Variant<'a>>, FieldListSequence<'a>),
    Empty,
}

// PIM syntax 36
#[derive(Debug, PartialEq, Eq)]
pub struct Variant<'a>(pub CaseLabelList<'a>, pub FieldListSequence<'a>);

// PIM syntax 37
#[derive(Debug, PartialEq, Eq)]
pub struct CaseLabelList<'a>(pub Vec<CaseLabels<'a>>);

// PIM syntax 38
#[derive(Debug, PartialEq, Eq)]
pub struct CaseLabels<'a>(pub ConstExpression<'a>, pub Option<ConstExpression<'a>>);

// PIM syntax 39
#[derive(Debug, PartialEq, Eq)]
pub struct SetType<'a>(pub SimpleType<'a>);

// PIM syntax 40
#[derive(Debug, PartialEq, Eq)]
pub struct PointerType<'a>(pub Box<Type<'a>>);

// PIM syntax 41
#[derive(Debug, PartialEq, Eq)]
pub struct ProcedureType<'a>(pub Option<FormalTypeList<'a>>);

// PIM syntax 42
#[derive(Debug, PartialEq, Eq)]
pub struct FormalTypeList<'a>(pub Vec<(Option<Var>, FormalType<'a>)>, pub Option<Qualident<'a>>);

// PIM syntax 44
#[derive(Debug, PartialEq, Eq)]
pub struct VariableDeclaration<'a>(pub IdentList<'a>, pub Type<'a>);

// PIM syntax 45
// Refactored.
#[derive(Debug, PartialEq, Eq)]
pub struct Designator<'a>(pub Qualident<'a>, pub Vec<DesignatorElement<'a>>);

#[derive(Debug, PartialEq, Eq)]
pub enum DesignatorElement<'a> {
    Ident(Ident<'a>),
    ExpList(ExpList<'a>),
    Arrow,
}

// PIM syntax 46
#[derive(Debug, PartialEq, Eq)]
pub struct ExpList<'a>(pub Vec<Expression<'a>>);

// PIM syntax 47
#[derive(Debug, PartialEq, Eq)]
pub struct Expression<'a>(pub SimpleExpression<'a>, pub Option<(Relation, SimpleExpression<'a>)>);

// PIM syntax 48
#[derive(Debug, PartialEq, Eq)]
pub struct SimpleExpression<'a>(pub AddSub, pub Box<Term<'a>>, pub Vec<(AddOperator, Term<'a>)>);

// PIM syntax 49
#[derive(Debug, PartialEq, Eq)]
pub struct Term<'a>(pub Factor<'a>, pub Vec<(MulOperator, Factor<'a>)>);

// PIM syntax 50
#[derive(Debug, PartialEq, Eq)]
pub enum Factor<'a> {
    Num(Number<'a>),
    QStr(QStr<'a>),
    Set(Set<'a>),
    Designator(Designator<'a>, Option<ActualParameters<'a>>),
    Expression(Expression<'a>),
    Not(Box<Factor<'a>>),
}

// PIM syntax 52
#[derive(Debug, PartialEq, Eq)]
pub struct ActualParameters<'a>(pub Option<ExpList<'a>>);

// PIM syntax 53
#[derive(Debug, PartialEq, Eq)]
pub enum Statement<'a> {
    Assignment(Assignment<'a>),
    ProcedureCall(ProcedureCall<'a>),
    IfStatement(IfStatement<'a>),
    CaseStatement(CaseStatement<'a>),
    WhileStatement(WhileStatement<'a>),
    RepeatStatement(RepeatStatement<'a>),
    LoopStatement(LoopStatement<'a>),
    ForStatement(ForStatement<'a>),
    WithStatement(WithStatement<'a>),
    EXIT,
    RETURN(Option<Expression<'a>>),
    Empty,
}

// PIM syntax 57
#[derive(Debug, PartialEq, Eq)]
pub struct Assignment<'a>(pub Designator<'a>, pub Expression<'a>);

// PIM syntax 58
#[derive(Debug, PartialEq, Eq)]
pub struct ProcedureCall<'a>(pub Designator<'a>, pub Option<ActualParameters<'a>>);

// PIM syntax 59
#[derive(Debug, PartialEq, Eq)]
pub struct StatementSequence<'a>(pub Vec<Statement<'a>>);

// PIM syntax 60
#[derive(Debug, PartialEq, Eq)]
pub struct IfStatement<'a>(
    pub Vec<(Expression<'a>, StatementSequence<'a>)>,
    pub Option<StatementSequence<'a>>,
);

// PIM syntax 62
#[derive(Debug, PartialEq, Eq)]
pub struct CaseStatement<'a>(
    pub Expression<'a>,
    pub Vec<Case<'a>>,
    pub Option<StatementSequence<'a>>,
);

// PIM syntax 65
#[derive(Debug, PartialEq, Eq)]
pub struct Case<'a>(pub CaseLabelList<'a>, pub StatementSequence<'a>);

// PIM syntax 66
#[derive(Debug, PartialEq, Eq)]
pub struct WhileStatement<'a>(pub Expression<'a>, pub StatementSequence<'a>);

// PIM syntax 67
#[derive(Debug, PartialEq, Eq)]
pub struct RepeatStatement<'a>(pub StatementSequence<'a>, pub Expression<'a>);

// PIM syntax 68
#[derive(Debug, PartialEq, Eq)]
pub struct ForStatement<'a>(
    pub Ident<'a>,
    pub Expression<'a>,
    pub Expression<'a>,
    pub Option<ConstExpression<'a>>,
    pub StatementSequence<'a>,
);

// PIM syntax 70
#[derive(Debug, PartialEq, Eq)]
pub struct LoopStatement<'a>(pub StatementSequence<'a>);

// PIM syntax 71
#[derive(Debug, PartialEq, Eq)]
pub struct WithStatement<'a>(pub Designator<'a>, pub StatementSequence<'a>);

// PIM syntax 72
#[derive(Debug, PartialEq, Eq)]
pub struct ProcedureDeclaration<'a>(pub ProcedureHeading<'a>, pub Block<'a>, pub Ident<'a>);

// PIM syntax 73
#[derive(Debug, PartialEq, Eq)]
pub struct ProcedureHeading<'a>(pub Ident<'a>, pub Option<FormalParameters<'a>>);

// PIM syntax 74
#[derive(Debug, PartialEq, Eq)]
pub struct Block<'a>(pub Vec<Declaration<'a>>, pub Option<StatementSequence<'a>>);

// PIM syntax 75
#[derive(Debug, PartialEq, Eq)]
pub enum Declaration<'a> {
    Const(Vec<ConstantDeclaration<'a>>),
    Type(Vec<TypeDeclaration<'a>>),
    Var(Vec<VariableDeclaration<'a>>),
    Procedure(ProcedureDeclaration<'a>),
    Module(ModuleDeclaration<'a>),
}

// PIM syntax 79
#[derive(Debug, PartialEq, Eq)]
pub struct FormalParameters<'a>(pub Vec<FPSection<'a>>, pub Option<Qualident<'a>>);

// PIM syntax 81
#[derive(Debug, PartialEq, Eq)]
pub struct FPSection<'a>(pub Option<Var>, pub IdentList<'a>, pub FormalType<'a>);

// PIM syntax 82
#[derive(Debug, PartialEq, Eq)]
pub struct FormalType<'a>(pub Option<ArrayOf>, pub Qualident<'a>);

// PIM syntax 83
#[derive(Debug, PartialEq, Eq)]
pub struct ModuleDeclaration<'a>(
    pub Ident<'a>,
    pub Option<Priority<'a>>,
    pub Vec<Import<'a>>,
    pub Option<Export<'a>>,
    pub Block<'a>,
    pub Ident<'a>,
);

// PIM syntax 85
#[derive(Debug, PartialEq, Eq)]
pub struct Priority<'a>(pub ConstExpression<'a>);

// PIM syntax 86
#[derive(Debug, PartialEq, Eq)]
pub struct Export<'a>(pub Option<Qualified>, pub IdentList<'a>);

// PIM syntax 87
#[derive(Debug, PartialEq, Eq)]
pub struct Import<'a>(pub Option<Ident<'a>>, pub IdentList<'a>);

// PIM syntax 88
#[derive(Debug, PartialEq, Eq)]
pub struct DefinitionModule<'a>(
    pub Ident<'a>,
    pub Vec<Import<'a>>,
    pub Option<Export<'a>>,
    pub Vec<Definition<'a>>,
    pub Ident<'a>,
);

// PIM syntax 90
#[derive(Debug, PartialEq, Eq)]
pub enum Definition<'a> {
    Const(Vec<ConstantDeclaration<'a>>),
    Type(Vec<(Ident<'a>, Option<Type<'a>>)>),
    Var(Vec<VariableDeclaration<'a>>),
    ProcedureHeading(ProcedureHeading<'a>),
}

// PIM syntax 94
#[derive(Debug, PartialEq, Eq)]
pub struct ProgramModule<'a>(
    pub Ident<'a>,
    pub Option<Priority<'a>>,
    pub Vec<Import<'a>>,
    pub Block<'a>,
    pub Ident<'a>,
);

// PIM syntax 96
#[derive(Debug, PartialEq, Eq)]
pub enum CompilationUnit<'a> {
    DefinitionModule(DefinitionModule<'a>),
    ProgramModule(Option<Implementation>, ProgramModule<'a>),
}

// Additional structures
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AddSub {
    ADD,
    SUB,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ArrayOf;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Implementation;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Qualified;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Var;
