#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Float(f64),
    Int(i64),
    Bool(bool),
    None,
}
