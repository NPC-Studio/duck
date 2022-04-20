use super::*;
use colored::Colorize;
use hashbrown::HashMap;
use itertools::Itertools;
use parking_lot::Mutex;

#[macro_export]
macro_rules! array {
    ($ty:expr) => {
        Ty::Array(Box::new($ty))
    };
}

#[macro_export]
macro_rules! adt {
    ($solver:expr => { $($var:ident: $should_be:expr), * $(,)? }) => {
        $solver.new_adt(AdtState::Extendable, vec![
            $((
                crate::parse::Identifier::lazy(stringify!($var).to_string()),
                $should_be,
            ), )*
        ])
    };
    ($($var:ident: $should_be:expr), * $(,)?) => {
        {
            let fields = vec![
                $((
                    crate::parse::Identifier::lazy(stringify!($var).to_string()),
                    $should_be,
                ), )*
            ];
            Ty::Adt(SOLVER.lock().new_adt(AdtState::Extendable, fields))
        }
    };
}

#[macro_export]
macro_rules! function {
    (() => $return_type:expr) => {
        crate::solve::Ty::Func(crate::solve::Func::Def(crate::solve::Def {
            binding: None,
            parameters: vec![],
            return_type: Box::new($return_type),
        }))
    };
    (($($arg:expr), * $(,)?) => $return_type:expr) => {
        crate::solve::Ty::Func(crate::solve::Func::Def(crate::solve::Def {
            binding: None,
            parameters:  vec![$($arg)*],
            return_type: Box::new($return_type),
        }))
    };
}

lazy_static! {
    static ref PRINTER: Mutex<Printer> = Mutex::new(Printer {
        aliases: HashMap::default(),
        expr_strings: HashMap::default(),
        alias_characters: vec![
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
            'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
            'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'Ƃ', 'Ɔ', 'ƈ', 'Ɖ', 'Ƌ', 'Ǝ', 'Ə', 'Ɛ', 'Ɣ', 'ƕ', 'Ɩ',
            'Ɨ', 'ƚ', 'ƛ', 'Ɯ', 'ƞ', 'Ɵ', 'ơ', 'Ƣ', 'Ƥ', 'ƥ', 'ƨ', 'Ʃ', 'ƫ', 'Ƭ', 'Ʊ', 'Ʋ', 'ƶ', 'ƹ', 'ƾ', 'ƿ',
        ],
        iter: 0,
    });
}

pub struct Printer {
    aliases: HashMap<Var, char>,
    expr_strings: HashMap<Var, String>,
    alias_characters: Vec<char>,
    iter: usize,
}
impl Printer {
    pub fn flush() {
        let mut printer = PRINTER.lock();
        printer.aliases.clear();
        printer.expr_strings.clear();
        printer.iter = 0;
    }

    pub fn give_expr_alias(var: Var, name: String) {
        if !PRINTER.lock().aliases.contains_key(&var) {
            println!("{}        {}   :   {}", "ALIAS".bright_red(), Printer::var(&var), name);
            // PRINTER.lock().expr_strings.insert(var, name);
        }
    }

    #[must_use]
    pub fn var(var: &Var) -> String {
        let mut printer = PRINTER.lock();
        let var = *var;
        if let Some(expr_string) = printer.expr_strings.get(&var) {
            expr_string.clone()
        } else {
            let entry = if let Some(entry) = printer.aliases.get(&var) {
                entry.to_string()
            } else {
                let v = printer.alias_characters[printer.iter];
                printer.iter = if printer.iter + 1 >= printer.alias_characters.len() {
                    0
                } else {
                    printer.iter + 1
                };
                printer.aliases.insert(var, v);
                v.to_string()
            };
            entry
        }
        .bright_black()
        .bold()
        .to_string()
    }

    #[must_use]
    pub fn ty(ty: &Ty, solver: &Solver) -> String {
        let s = match ty {
            Ty::Uninitialized => "<null>".into(),
            Ty::Any => "any".into(),
            Ty::Undefined => "undefined".into(),
            Ty::Noone => "noone".into(),
            Ty::Bool => "bool".into(),
            Ty::Real => "real".into(),
            Ty::Str => "string".into(),
            Ty::Array(inner) => format!("[{}]", Self::ty(inner, solver)),
            Ty::Var(var) => Self::var(var),
            Ty::Adt(adt_id) => {
                let adt = solver.get_adt(*adt_id);
                if adt.fields.is_empty() {
                    "{}".into()
                } else {
                    format!(
                        "{}{{ {} }}",
                        match adt.state {
                            AdtState::Inferred => "?",
                            AdtState::Extendable => "mut ",
                            AdtState::Concrete => "",
                        },
                        adt.fields
                            .iter()
                            .map(|(name, field)| format!("{}: {}", name, Printer::ty(&field.ty, solver)))
                            .join(", ")
                    )
                }
            }
            Ty::Func(function) => match function {
                Func::Def(Def {
                    parameters,
                    return_type,
                    ..
                }) => format!(
                    "fn ({}) -> {}",
                    parameters.iter().map(|ty| Printer::ty(ty, solver)).join(", "),
                    Printer::ty(return_type, solver)
                ),
                Func::Call(Call {
                    parameters,
                    return_type,
                }) => format!(
                    "({}) -> {}",
                    parameters.iter().map(|ty| Printer::ty(ty, solver)).join(", "),
                    Printer::ty(return_type, solver)
                ),
            },
        };
        s.blue().bold().to_string()
    }

    #[must_use]
    pub fn query(a: &crate::parse::Expr) -> String {
        format!("{}        {a}: {}", "QUERY".bright_red(), Printer::var(&a.var()))
    }

    #[must_use]
    pub fn ty_unification(a: &Ty, b: &Ty, solver: &Solver) -> String {
        format!(
            "{}      {}   ≟   {}",
            "UNIFY T".bright_yellow(),
            Printer::ty(a, solver),
            Printer::ty(b, solver),
        )
    }

    #[must_use]
    pub fn var_unification(var: &Var, ty: &Ty, solver: &Solver) -> String {
        format!(
            "{}      {}   ≟   {}",
            "UNIFY M".bright_yellow(),
            Printer::var(var),
            Printer::ty(ty, solver),
        )
    }

    #[must_use]
    pub fn substitution(var: &Var, ty: &Ty, solver: &Solver) -> String {
        format!(
            "{}          {}   →   {}",
            "SUB".bright_green(),
            Printer::var(var),
            Printer::ty(ty, solver),
        )
    }
}

#[macro_export]
macro_rules! duck_error {
    ($($arg:tt)*) => {
        Err(crate::duck_error_unwrapped!($($arg)*))
    };
}

#[macro_export]
macro_rules! duck_error_unwrapped {
    ($($arg:tt)*) => {
        codespan_reporting::diagnostic::Diagnostic::error().with_message(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! duck_bug {
    ($($msg_arg:expr), * $(,)?) => {
        Err(codespan_reporting::diagnostic::Diagnostic::bug().with_message(format!($($msg_arg, )*)))
    };
}