fn main() {
    let (a, b) = ("a".to_string(), "b".to_string());
    println!("{}", bar(&a, &b));
    println!("{}", foo(&a, &b));
}

fn bar<'a>(a: &'a str, b: &'a str) -> impl std::fmt::Display + use<> {
    let string = format!("{} - {}", a, b);
    string
}

fn foo<'a>(a: &'a str, b: &'a str) -> impl std::fmt::Display + use<> {
    let a = format!("[{}]", a);
    let b = format!("[{}]", b);

    bar(&a, &b) // <-- compilation error
}

trait Trait {}

// argument position: anonymous type parameter
fn f1(arg: impl Trait) {}

// return position: abstract return type
fn f2() -> impl Trait {
    struct S;
    impl Trait for S {}
    S
}

// generic type parameter
fn with_generic_type<T: Trait>(arg: T) {}

// impl Trait in argument position
fn with_impl_trait(arg: impl Trait) {}

#[cfg(test)]

fn lifetime_test() {
    let (a, b) = ("a".to_string(), "b".to_string());
    assert_eq!(bar(&a, &b).to_string(), "[a] - [b]");
    assert_eq!(foo(&a, &b).to_string(), "a - b");
}
