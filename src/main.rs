// https://doc.rust-lang.org/stable/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
//
// Symantics is ballin'
pub trait Symantics {
    fn value(v: i32) -> Self;
    fn add(a: Self, b: Self) -> Self;
    //    fn lam(a: Self::Item)
}

// Apa is not a bepa
// It is a pretty printer!
struct Apa(String);

impl Symantics for Apa {
    fn value(v: i32) -> Self {
        Apa(format!("{}", v))
    }
    fn add(a: Self, b: Self) -> Self {
        Apa(format!("{} + {}", a.0, b.0))
    }
}

fn view(v: Apa) -> String {
    v.0
}

// An evaluator!
struct Eval (i32);

impl Symantics for Eval {
    fn value(v: i32) -> Eval {
        Eval (v)
    }
    fn add(a: Eval, b: Eval) -> Eval {
        Eval ( a.0 + b.0)
    }
}

fn eval(v: Eval) -> i32 {
    v.0
}

fn main() {
    println!("{}", view(Apa::value(5)));
    println!("{}", view(Apa::add(Apa::value(3), Apa::value(4))));
    println!(
        "Evaluated: {}",
        eval(Eval::add(Eval::value(3), Eval::value(4)))
    )
}
