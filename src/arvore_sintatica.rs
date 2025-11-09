use std::{
    cmp::Ordering::*,
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Rem},
};

pub enum OperadorUnario {
    Negacao,
    Pinguinacao,
}

impl OperadorUnario {
    fn precedencia(&self) -> u8 {
        3
    }
}

pub enum OperadorBinario {
    Adicao,
    Multiplicacao,
    Divisao,
    RestoDivisao,
}

impl OperadorBinario {
    fn precedencia(&self) -> u8 {
        if let OperadorBinario::Adicao = self {
            1
        } else if let OperadorBinario::RestoDivisao = self {
            3
        } else {
            2
        }
    }
}

pub enum Expressao {
    Valor(i64),
    OperadorBinario {
        p: Box<Expressao>,
        op: OperadorBinario,
        s: Box<Expressao>,
    },
    OperadorUnario(OperadorUnario, Box<Expressao>),
}

impl Expressao {
    fn precedencia(&self) -> u8 {
        match self {
            Expressao::OperadorBinario { op, .. } => op.precedencia(),
            Expressao::Valor(_) => 10,
            Expressao::OperadorUnario(op, _) => op.precedencia(),
        }
    }
}

impl Display for OperadorUnario {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperadorUnario::Pinguinacao => f.write_str("🐧"),
            OperadorUnario::Negacao => f.write_str("-"),
        }
    }
}

impl Display for OperadorBinario {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperadorBinario::Adicao => f.write_str("+"),
            OperadorBinario::Multiplicacao => f.write_str("*"),
            OperadorBinario::Divisao => f.write_str("/"),
            OperadorBinario::RestoDivisao => f.write_str("%"),
        }
    }
}

impl Display for Expressao {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expressao::Valor(i) => write!(f, "{i}"),
            Expressao::OperadorUnario(op, exp) => write!(f, "{op}{exp}"),
            Expressao::OperadorBinario { p, op, s } => {
                let prec_esquerda = p.precedencia();
                let prec_direita = s.precedencia();

                let prec_atual = op.precedencia();

                let cmp_esq = prec_atual.cmp(&prec_esquerda);
                let cmp_dir = prec_atual.cmp(&prec_direita);

                match cmp_esq {
                    Less | Equal => write!(f, "{p}"),
                    Greater => write!(f, "({p})"),
                }?;

                write!(f, " {op} ")?;

                match cmp_dir {
                    Less | Equal => write!(f, "{s}"),
                    Greater => write!(f, "({s})"),
                }
            }
        }
    }
}

impl Add for &Box<Expressao> {
    type Output = Option<i64>;
    fn add(self, rhs: Self) -> Self::Output {
        self.avaliar()
            .zip(rhs.avaliar())
            .map(|(a, b)| a.checked_add(b))
            .flatten()
    }
}

impl Mul for &Box<Expressao> {
    type Output = Option<i64>;

    fn mul(self, rhs: Self) -> Self::Output {
        self.avaliar()
            .zip(rhs.avaliar())
            .map(|(a, b)| a.checked_mul(b))
            .flatten()
    }
}

impl Div for &Box<Expressao> {
    type Output = Option<i64>;

    fn div(self, rhs: Self) -> Self::Output {
        self.avaliar()
            .zip(rhs.avaliar())
            .map(|(a, b)| a.checked_div(b))
            .flatten()
    }
}

impl Rem for &Box<Expressao> {
    type Output = Option<i64>;

    fn rem(self, rhs: Self) -> Self::Output {
        self.avaliar()
            .zip(rhs.avaliar())
            .map(|(a, b)| a.checked_rem(b))
            .flatten()
    }
}

impl Neg for &Box<Expressao> {
    type Output = Option<i64>;

    fn neg(self) -> Self::Output {
        self.avaliar().map(|v| -v)
    }
}

impl Expressao {
    pub fn new_num(i: i64) -> Self {
        Expressao::Valor(i)
    }

    pub fn new_bin_op(op: OperadorBinario, primeiro: Expressao, segundo: Expressao) -> Self {
        Expressao::OperadorBinario {
            p: Box::new(primeiro),
            op,
            s: Box::new(segundo),
        }
    }

    pub fn new_un_op(op: OperadorUnario, primeiro: Expressao) -> Self {
        Expressao::OperadorUnario(op, Box::new(primeiro))
    }

    pub fn avaliar(&self) -> Option<i64> {
        match self {
            Self::OperadorBinario { p, op: operador, s } => match operador {
                OperadorBinario::Adicao => p + s,
                OperadorBinario::Multiplicacao => p * s,
                OperadorBinario::Divisao => p / s,
                OperadorBinario::RestoDivisao => p % s,
            },
            Self::OperadorUnario(op, val) => match op {
                OperadorUnario::Negacao => -val,
                OperadorUnario::Pinguinacao => val.avaliar(),
            },

            Self::Valor(v) => Some(v).copied(),
        }
    }

    pub fn imprimir(&self) {
        println!("{}", self)
    }

    pub fn imprimir_arvore(&self) {
        print!("   ");
        self.print_node("".to_string(), true);
    }

    fn print_node(&self, prefix: String, is_last: bool) {
        let conector = if prefix.is_empty() {
            ""
        } else if is_last {
            "└── "
        } else {
            "├── "
        };

        match self {
            Expressao::Valor(n) => println!("{prefix}{conector}{n}"),
            Expressao::OperadorUnario(op, _) => println!("{prefix}{conector}{op}"),
            Expressao::OperadorBinario { op, .. } => println!("{prefix}{conector}{op}"),
        }

        let prefix = prefix + if is_last { "   " } else { "|   " };

        match self {
            Expressao::Valor(_) => {}
            Expressao::OperadorUnario(OperadorUnario::Pinguinacao, n) => {
                n.print_node(prefix + " ", true)
            }
            Expressao::OperadorUnario(_, n) => n.print_node(prefix, true),
            Expressao::OperadorBinario { p, s, .. } => {
                p.print_node(prefix.clone(), false);
                s.print_node(prefix, true);
            }
        }
    }
}

#[test]
fn soma_exp() {
    let a = Expressao::new_num(10);
    let b = Expressao::new_num(20);

    assert_eq!(a.avaliar(), Some(10));
    assert_eq!(b.avaliar(), Some(20));

    let s = Expressao::new_bin_op(OperadorBinario::Adicao, a, b);

    assert_eq!(s.avaliar(), Some(30))
}

#[test]
fn mul_exp() {
    let a = Expressao::new_num(10);
    let b = Expressao::new_num(20);
    let c = Expressao::new_un_op(OperadorUnario::Negacao, b);

    assert_eq!(a.avaliar(), Some(10));
    assert_eq!(c.avaliar(), Some(-20));

    let mult = Expressao::new_bin_op(OperadorBinario::Multiplicacao, a, c);

    assert_eq!(mult.avaliar(), Some(-200))
}
