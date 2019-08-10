// Symantics is a ball trait
pub trait Symantics<T> {
    type Item<T>;
    fn value(v: i32) -> Self::Item<i32>;
    fn add(a: Self::Item, b: Self::Item) -> Self::Item;
    fn lam(a: Self::Item)
}

// Apa is not a bepa
struct Apa<T> {
    _marker: std::marker::PhantomData<T>,
    inner: String,
}

impl<T> Symantics<T> for Apa<T>
where
    T: 'static + Copy,
{
    type Item = Apa<T>;
    fn value(v: i32) -> Apa<T> {
        Apa {
            inner: format!("{}", v),
            _marker: std::marker::PhantomData,
        }
    }
    fn add(a: Self::Item, b: Self::Item) -> Self::Item {
        Apa {
            inner: format!("{} + {}", a.inner, b.inner),
            _marker: std::marker::PhantomData,
        }
    }
}

fn view<T>(v: Apa<T>) -> String {
    v.inner
}

fn main() {
    println!(
        "{}",
        view(Apa::<i32>::add(Apa::<i32>::value(3), Apa::<i32>::value(4)))
    )
}
