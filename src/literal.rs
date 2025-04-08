use crate::error::ErrorMessage;
#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Float(f64),
    Int(i64),
    Bool(bool),
    None,
}
impl Literal {
    pub fn print(&self) {
        match self {
            Literal::String(val) => println!("{}", val),
            Literal::Int(val) => println!("{}", val),
            Literal::Float(val) => println!("{}", val),
            Literal::Bool(val) => println!("{}", val),
            Literal::None => println!("Null"),
        }
    }
}
macro_rules! impl_op {
    ($trait_name:ident, $method_name:ident, $op:tt, $op_name:expr, with_string) => {
        pub trait $trait_name<RHS = Self> {
            type Output;
            type Error;
            fn $method_name(self, rhs: RHS) -> Result<Self::Output, Self::Error>;
        }

        impl $trait_name for Literal {
            type Output = Literal;
            type Error = ErrorMessage;

            fn $method_name(self, rhs: Literal) -> Result<Literal, ErrorMessage> {
                match (self, rhs) {
                    (Literal::Int(a), Literal::Int(b)) => Ok(Literal::Int(a $op b)),
                    (Literal::Float(a), Literal::Float(b)) => Ok(Literal::Float(a $op b)),
                    (Literal::Int(a), Literal::Float(b)) => Ok(Literal::Float(a as f64 $op b)),
                    (Literal::Float(a), Literal::Int(b)) => Ok(Literal::Float(a $op b as f64)),
                    (Literal::String(a), Literal::String(b)) => Ok(Literal::String(a $op &b)),
                    (l, r) => Err(ErrorMessage::new(&format!(
                        "{} not supported between {:?} and {:?}",
                        $op_name, l, r
                    ))),
                }
            }
        }
    };

    ($trait_name:ident, $method_name:ident, $op:tt, $op_name:expr, with_float) => {
        pub trait $trait_name<RHS = Self> {
            type Output;
            type Error;
            fn $method_name(self, rhs: RHS) -> Result<Self::Output, Self::Error>;
        }

        impl $trait_name for Literal {
            type Output = Literal;
            type Error = ErrorMessage;

             fn $method_name(self, rhs: Literal) -> Result<Literal, ErrorMessage> {
                match (self, rhs) {
                    (Literal::Int(a), Literal::Int(b)) => Ok(Literal::Int(a $op b)),
                    (Literal::Float(a), Literal::Float(b)) => Ok(Literal::Float(a $op b)),
                    (Literal::Int(a), Literal::Float(b)) => Ok(Literal::Float(a as f64 $op b)),
                    (Literal::Float(a), Literal::Int(b)) => Ok(Literal::Float(a $op b as f64)),
                    (l, r) => Err(ErrorMessage::new(&format!(
                        "{} not supported between {:?} and {:?}",
                        $op_name, l, r
                    ))),
                }
            }
        }
    };

    ($trait_name:ident, $method_name:ident, $op:tt, $op_name:expr, with_int) => {
        pub trait $trait_name<RHS = Self> {
            type Output;
            type Error;
            fn $method_name(self, rhs: RHS) -> Result<Self::Output, Self::Error>;
        }

        impl $trait_name for Literal {
            type Output = Literal;
            type Error = ErrorMessage;

            fn $method_name(self, rhs: Literal) -> Result<Literal, ErrorMessage> {
                match (self, rhs) {
                    (Literal::Int(a), Literal::Int(b)) => Ok(Literal::Int(a $op b)),
                    (l, r) => Err(ErrorMessage::new(&format!(
                        "{} not supported between {:?} and {:?}",
                        $op_name, l, r
                    ))),
                }
            }
        }
    };

    ($trait_name:ident, $method_name: ident, $op: tt, $op_name: expr, ret_bool)=>{
        pub trait $trait_name<RHS = Self>{
            type Output;
            type Error;
            fn $method_name(self, rhs: RHS) -> Result<Self::Output, Self::Error>;
        }

        impl $trait_name for Literal {
            type Output = Literal;
            type Error = ErrorMessage;

            fn $method_name(self, rhs: Literal) -> Result<Literal, ErrorMessage> {
                match (self, rhs) {
                    (Literal::Int(a), Literal::Int(b)) => Ok(Literal::Bool(a $op b)),
                    (Literal::Float(a), Literal::Float(b)) => Ok(Literal::Bool(a $op b)),
                    (Literal::Int(a), Literal::Float(b)) => Ok(Literal::Bool((a as f64) $op b)),
                    (Literal::Float(a), Literal::Int(b)) => Ok(Literal::Bool(a $op b as f64)),
                    (Literal::String(a), Literal::String(b)) => Ok(Literal::Bool(a $op b)),
                    (l, r) => Err(ErrorMessage::new(&format!(
                        "{} not supported between {:?} and {:?}",
                        $op_name, l, r
                    ))),
                }
            }
        }
    }
}
impl_op!(Add, add, +, "Addition", with_string);
impl_op!(Sub, sub, -,"Subtraction", with_float);
impl_op!(Div, div, /, "Division", with_float);
impl_op!(Mul, mul, *, "Multiplication", with_float);
impl_op!(Mod, modulo, %, "Modulo", with_float);
impl_op!(LeftShift, left_shift, << ,"Left Shift", with_int);
impl_op!(RightShift, right_shify, >>, "Right Shift", with_int);
impl_op!(BitAnd, bit_and, &, "Bit And", with_int);
impl_op!(BitOr, bit_or, |, "Bit Or", with_int);
impl_op!(Xor, xor, ^, "Xor", with_int);
impl_op!(NotEqual, not_equal, != ,"Not Equal", ret_bool);
impl_op!(EqualTo, equal_to, ==, "Equal to", ret_bool);
impl_op!(Greater, greater, >, "Greater", ret_bool);
impl_op!(GreaterOrEqual, greater_or_equal, >=, "Greater Or Equal", ret_bool);
impl_op!(Lesser, lesser, <, "Lesser", ret_bool);
impl_op!(LesserOrEqual,lesser_or_equal, <= ,"Lesser or Equal", ret_bool);
