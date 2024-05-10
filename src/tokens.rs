#[derive(Debug)]
pub enum Type {
    Int,
    Bool,
    Mult(Box<Type>, Box<Type>),
    Func(Box<Type>, Box<Type>),
    List(Box<Type>),
}

pub enum Expr {
    Var(String),
    Int(i32),
    Bool(bool),
    Times(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Equal(Box<Expr>, Box<Expr>),
    Less(Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Func(String, Box<Type>, Box<Expr>),
    Apply(Box<Expr>, Box<Expr>),
    Pair(Box<Expr>, Box<Expr>),
    First(Box<Expr>),
    Second(Box<Expr>),
    Recursion(String, Box<Type>, Box<Expr>),
    None(Type),
    Cons(Box<Expr>, Box<Expr>),
    Match(Box<Expr>, Box<Type>, Box<Expr>, String, String, Box<Expr>),
}

pub enum Commands {
    Expr(Expr),
    Fn(String, Expr),
    Exit,
}

pub trait Str {
    fn to_string(&self) -> String;
    fn to_string_with_precedence(&self, outer_precedence: i32) -> String;
}

impl Str for Type {
    fn to_string_with_precedence(&self, outer_precedence: i32) -> String {
        let (inner_precedence, result) = match self {
            Type::Int => (4, "Int".to_string()),
            Type::Bool => (4, "Bool".to_string()),
            Type::Mult(left, right) => {
                let left_str = left.to_string_with_precedence(2);
                let right_str = right.to_string_with_precedence(2);
                (2, format!("{} * {}", left_str, right_str))
            }
            Type::Func(arg, ret) => {
                let arg_str = arg.to_string_with_precedence(1);
                let ret_str = ret.to_string_with_precedence(0);
                (1, format!("{} -> {}", arg_str, ret_str))
            }
            Type::List(ty) => {
                let ty_str = ty.to_string_with_precedence(3);
                (3, format!("{} List", ty_str))
            }
        };

        if inner_precedence > outer_precedence {
            result
        } else {
            format!("({})", result)
        }
    }

    fn to_string(&self) -> String {
        self.to_string_with_precedence(-1)
    }
}
