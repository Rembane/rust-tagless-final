// https://doc.rust-lang.org/stable/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
//
// Symantics is ballin'
pub trait Symantics {
    type Item;
    fn value(v: i32) -> Self::Item;
    fn add(a: Self::Item, b: Self::Item) -> Self::Item;
    //    fn lam(a: Self::Item)
}

// Apa is not a bepa
// It is a pretty printer!
struct Apa(String);

impl Symantics for Apa {
    type Item = Apa;
    fn value(v: i32) -> Apa {
        Apa(format!("{}", v))
    }
    fn add(a: Self::Item, b: Self::Item) -> Self::Item {
        Apa(format!("{} + {}", a.0, b.0))
    }
}

fn view(v: Apa) -> String {
    v.0
}

// An evaluator!
struct Eval {
    inner: i32,
}

impl Symantics for Eval {
    type Item = Eval;
    fn value(v: i32) -> Eval {
        Eval { inner: v }
    }
    fn add(a: Self::Item, b: Self::Item) -> Self::Item {
        Eval {
            inner: a.inner + b.inner,
        }
    }
}

fn eval(v: Eval) -> i32 {
    v.inner
}

fn main() {
    println!("{}", view(Apa::add(Apa::value(3), Apa::value(4))));
    println!(
        "Evaluated: {}",
        eval(Eval::add(Eval::value(3), Eval::value(4)))
    )
}
