# 16. Naming Conventions

## Standard

**PascalCase for all identifiers** (following industry standards for enterprise-grade languages)

| Category | Convention | Example |
|----------|------------|---------|
| Types/Structs | PascalCase | `Expr`, `Token`, `Parser`, `Config` |
| Enums | PascalCase | `ExprKind`, `TokenKind`, `Result`, `Option` |
| Enum Values | PascalCase | `ExprKind.Var`, `TokenKind.If`, `Some`, `None` |
| Functions/Methods | PascalCase | `ParseExpr`, `LexToken`, `CalculateSum` |
| Variables | PascalCase | `Tokens`, `Input`, `Environment`, `Result` |
| Constants | PascalCase | `MaxDepth`, `DefaultSize`, `BufferCapacity` |
| Modules/Namespaces | PascalCase | `Lexer`, `Parser`, `TypeCheck`, `CodeGen` |
| Traits | PascalCase | `Display`, `Iterator`, `Serializable` |
| Parameters | PascalCase | `InputValue`, `OutputBuffer`, `ConfigPath` |
| Lifetimes | Lowercase | `'a`, `'ctx`, `'stream` |
| Refinement Type Aliases | PascalCase | `Nat`, `Positive`, `NonEmptyString`, `Email` |

## File Naming

| File Type | Convention | Example |
|-----------|------------|---------|
| Source Files | PascalCase | `Lexer.xin`, `Parser.xin`, `Config.xin` |
| Module Directories | PascalCase | `Lexer/`, `Parser/`, `TypeSystem/` |
| Test Files | PascalCase with Test suffix | `LexerTest.xin`, `ParserTest.xin` |

## Examples

```xin
// Good - Types and Structs
struct UserAccount { }
struct ParserResult { }
enum TokenKind { }

// Bad - Not PascalCase
struct user_account { }
struct parser_result { }

// Good - Functions
fn ParseExpression() -> Result<Expr, Error> { }
fn CalculateTotal(Items: Vec<Item>) -> i64 { }

// Bad - snake_case or mixed
fn parse_expression() -> Result<Expr, Error> { }
fn CalcTotal(items: Vec<Item>) -> i64 { }

// Good - Variables
let UserName: String = "John";
let MaximumBufferSize: usize = 1024;

// Bad
let user_name: String = "John";
let max_buffer_size: usize = 1024;

// Good - Constants
const MaximumRetryCount: i32 = 3;
const DefaultTimeoutMs: u64 = 5000;

// Bad
const MAX_RETRY_COUNT: i32 = 3;
const default_timeout_ms: u64 = 5000;

// Good - Modules
mod Lexer;
mod Parser;
mod TypeCheck;

// Bad
mod lexer;
mod type_check;

// Good - Parameters
fn CreateUser(Name: String, Email: String) -> User { }
fn ProcessData(InputBuffer: []u8, OutputBuffer: &mut [u8]) -> Result<(), Error> { }

// Bad
fn create_user(name: String, email: String) -> User { }
fn process_data(buf: []u8, out: &mut [u8]) -> Result<(), Error> { }
```

## Acronyms

| Acronym | Usage | Example |
|---------|-------|---------|
| URL | Keep as uppercase | `ParseUrl()` |
| HTTP | Keep as uppercase | `HttpClient` |
| JSON | Keep as uppercase | `JsonParser` |
| ID | Keep as uppercase | `UserId` |
| IO | Keep as uppercase | `IoError` |
