pub mod semantic;

pub use semantic::{
    analyze_code,
    analyze_program,
    FunctionSignature,
    PrimitiveType,
    SemanticAnalyzer,
    SemanticError,
    Symbol,
    SymbolKind,
};
