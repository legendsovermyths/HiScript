#[derive(Debug)]
pub enum Literal {
    String(String),
    Float(f64),
    Int(i64),
}
