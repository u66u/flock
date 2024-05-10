#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Bool,
    Mult(Box<Type>, Box<Type>),
    Func(Box<Type>, Box<Type>),
    List(Box<Type>),
}

#[derive(Debug, Clone)]
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

impl Type {
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

    pub fn to_string(&self) -> String {
        self.to_string_with_precedence(-1)
    }
}

impl Expr {
    fn to_string_with_precedence(&self, outer_precedence: i32) -> String {
        use Expr::*;
        let (inner_precedence, result) = match self {
            Var(x) => (10, x.clone()),
            Int(n) => (10, n.to_string()),
            Bool(b) => (10, b.to_string()),
            Pair(e1, e2) => (
                10,
                format!(
                    "({}, {})",
                    e1.to_string_with_precedence(0),
                    e2.to_string_with_precedence(0)
                ),
            ),
            None(ty) => (10, format!("[{}]", ty.to_string())),
            First(e) => (9, format!("fst {}", e.to_string_with_precedence(9))),
            Second(e) => (9, format!("snd {}", e.to_string_with_precedence(9))),
            Apply(e1, e2) => (
                9,
                format!(
                    "{} {}",
                    e1.to_string_with_precedence(8),
                    e2.to_string_with_precedence(9)
                ),
            ),
            Times(e1, e2) | Divide(e1, e2) | Mod(e1, e2) => {
                let symbol = match self {
                    Times(_, _) => "*",
                    Divide(_, _) => "/",
                    Mod(_, _) => "%",
                    _ => unreachable!(),
                };
                (
                    8,
                    format!(
                        "{} {} {}",
                        e1.to_string_with_precedence(7),
                        symbol,
                        e2.to_string_with_precedence(8)
                    ),
                )
            }
            Plus(e1, e2) | Minus(e1, e2) => {
                let symbol = if let Plus(_, _) = self { "+" } else { "-" };
                (
                    7,
                    format!(
                        "{} {} {}",
                        e1.to_string_with_precedence(6),
                        symbol,
                        e2.to_string_with_precedence(7)
                    ),
                )
            }
            Cons(e1, e2) => (
                6,
                format!(
                    "{} :: {}",
                    e1.to_string_with_precedence(6),
                    e2.to_string_with_precedence(5)
                ),
            ),
            Equal(e1, e2) | Less(e1, e2) => {
                let symbol = if let Equal(_, _) = self { "=" } else { "<" };
                (
                    5,
                    format!(
                        "{} {} {}",
                        e1.to_string_with_precedence(5),
                        symbol,
                        e2.to_string_with_precedence(5)
                    ),
                )
            }
            If(e1, e2, e3) => (
                4,
                format!(
                    "if {} then {} else {}",
                    e1.to_string_with_precedence(4),
                    e2.to_string_with_precedence(4),
                    e3.to_string_with_precedence(4)
                ),
            ),
            Match(e1, ty, e2, x, y, e3) => (
                3,
                format!(
                    "match {} with [{}] -> {} | {}::{} -> {}",
                    e1.to_string_with_precedence(3),
                    ty.to_string(),
                    e2.to_string_with_precedence(3),
                    x,
                    y,
                    e3.to_string_with_precedence(3)
                ),
            ),
            Func(x, ty, e) => (
                2,
                format!(
                    "fun {} : {} -> {}",
                    x,
                    ty.to_string(),
                    e.to_string_with_precedence(0)
                ),
            ),
            Recursion(x, ty, e) => (
                1,
                format!(
                    "rec {} : {} is {}",
                    x,
                    ty.to_string(),
                    e.to_string_with_precedence(0)
                ),
            ),
        };

        if inner_precedence > outer_precedence {
            result
        } else {
            format!("({})", result)
        }
    }

    pub fn to_string(&self) -> String {
        self.to_string_with_precedence(-1)
    }

    pub fn subst(substitutions: &Vec<(String, Expr)>, expr: &Expr) -> Expr {
        use Expr::*;
        match expr {
            Var(x) => substitutions
                .iter()
                .find(|(var_name, _)| var_name == x)
                .map(|(_, expr)| expr.clone())
                .unwrap_or_else(|| expr.clone()),
            Int(_) | Bool(_) | None(_) => expr.clone(),
            Times(e1, e2) => Self::Times(
                Box::new(Self::subst(substitutions, e1)),
                Box::new(Self::subst(substitutions, e2)),
            ),
            Divide(e1, e2) => Self::Divide(
                Box::new(Self::subst(substitutions, e1)),
                Box::new(Self::subst(substitutions, e2)),
            ),
            Mod(e1, e2) => Self::Mod(
                Box::new(Self::subst(substitutions, e1)),
                Box::new(Self::subst(substitutions, e2)),
            ),
            Plus(e1, e2) => Self::Plus(
                Box::new(Self::subst(substitutions, e1)),
                Box::new(Self::subst(substitutions, e2)),
            ),
            Minus(e1, e2) => Self::Minus(
                Box::new(Self::subst(substitutions, e1)),
                Box::new(Self::subst(substitutions, e2)),
            ),
            Equal(e1, e2) => Self::Equal(
                Box::new(Self::subst(substitutions, e1)),
                Box::new(Self::subst(substitutions, e2)),
            ),
            Less(e1, e2) => Self::Less(
                Box::new(Self::subst(substitutions, e1)),
                Box::new(Self::subst(substitutions, e2)),
            ),
            If(e1, e2, e3) => Self::If(
                Box::new(Self::subst(substitutions, e1)),
                Box::new(Self::subst(substitutions, e2)),
                Box::new(Self::subst(substitutions, e3)),
            ),
            Func(var, ty, e) => {
                let filtered_subs = substitutions
                    .iter()
                    .filter(|(var_name, _)| var_name != var)
                    .cloned()
                    .collect();
                Self::Func(
                    var.clone(),
                    ty.clone(),
                    Box::new(Self::subst(&filtered_subs, e)),
                )
            }
            Apply(e1, e2) => Self::Apply(
                Box::new(Self::subst(substitutions, e1)),
                Box::new(Self::subst(substitutions, e2)),
            ),
            Pair(e1, e2) => Self::Pair(
                Box::new(Self::subst(substitutions, e1)),
                Box::new(Self::subst(substitutions, e2)),
            ),
            First(e) => Self::First(Box::new(Self::subst(substitutions, e))),
            Second(e) => Self::Second(Box::new(Self::subst(substitutions, e))),
            Recursion(var, ty, e) => {
                let filtered_subs = substitutions
                    .iter()
                    .filter(|(var_name, _)| var_name != var)
                    .cloned()
                    .collect();
                Self::Recursion(
                    var.clone(),
                    ty.clone(),
                    Box::new(Self::subst(&filtered_subs, e)),
                )
            }
            Cons(e1, e2) => Self::Cons(
                Box::new(Self::subst(substitutions, e1)),
                Box::new(Self::subst(substitutions, e2)),
            ),
            Match(e1, ty, e2, x, y, e3) => {
                let filtered_subs = substitutions
                    .iter()
                    .filter(|(var_name, _)| var_name != x && var_name != y)
                    .cloned()
                    .collect();
                Self::Match(
                    Box::new(Self::subst(substitutions, e1)),
                    ty.clone(),
                    Box::new(Self::subst(substitutions, e2)),
                    x.clone(),
                    y.clone(),
                    Box::new(Self::subst(&filtered_subs, e3)),
                )
            }
        }
    }
}
