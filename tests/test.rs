use std::panic::catch_unwind;
use tracing_asserts as ta;

#[derive(PartialOrd, PartialEq, Debug)]
enum DummyType {
    Foo,
    Bar,
    Baz,
}

#[test]
fn test_assert_lt() {
    ta::assert_lt!(3, 4);
    ta::assert_lt!(4.0, 4.5);
    ta::assert_lt!("a string", "b string");
    ta::assert_lt!(
        DummyType::Foo,
        DummyType::Bar,
        "Message with {}",
        "cool formatting"
    );

    let a = &DummyType::Foo;
    let b = &DummyType::Baz;
    ta::assert_lt!(a, b);

    assert!(catch_unwind(|| ta::assert_lt!(5, 3)).is_err());
    assert!(catch_unwind(|| ta::assert_lt!(5, 5)).is_err());
    assert!(catch_unwind(|| ta::assert_lt!(DummyType::Bar, DummyType::Foo)).is_err());
}

#[test]
fn test_assert_gt() {
    ta::assert_gt!(4, 3);
    ta::assert_gt!(4.5, 4.0);
    ta::assert_gt!("b string", "a string");
    ta::assert_gt!(
        DummyType::Bar,
        DummyType::Foo,
        "Message with {}",
        "cool formatting"
    );

    let a = &DummyType::Foo;
    let b = &DummyType::Baz;
    ta::assert_gt!(b, a);

    assert!(catch_unwind(|| ta::assert_gt!(3, 5)).is_err());
    assert!(catch_unwind(|| ta::assert_gt!(5, 5)).is_err());
    assert!(catch_unwind(|| ta::assert_gt!(DummyType::Foo, DummyType::Bar)).is_err());
}

#[test]
fn test_assert_le() {
    ta::assert_le!(3, 4);
    ta::assert_le!(4, 4);
    ta::assert_le!(4.0, 4.5);
    ta::assert_le!("a string", "a string");
    ta::assert_le!("a string", "b string");
    ta::assert_le!(DummyType::Foo, DummyType::Bar, "Message");
    ta::assert_le!(
        DummyType::Foo,
        DummyType::Foo,
        "Message with {}",
        "cool formatting"
    );

    let a = &DummyType::Foo;
    let b = &DummyType::Baz;
    ta::assert_le!(a, a);
    ta::assert_le!(a, b);

    assert!(catch_unwind(|| ta::assert_le!(5, 3)).is_err());
    assert!(catch_unwind(|| ta::assert_le!(DummyType::Bar, DummyType::Foo)).is_err());
}

#[test]
fn test_assert_ge() {
    ta::assert_ge!(4, 3);
    ta::assert_ge!(4, 4);
    ta::assert_ge!(4.5, 4.0);
    ta::assert_ge!(5.0, 5.0);
    ta::assert_ge!("a string", "a string");
    ta::assert_ge!("b string", "a string");
    ta::assert_ge!(DummyType::Bar, DummyType::Bar, "Example");
    ta::assert_ge!(
        DummyType::Bar,
        DummyType::Foo,
        "Message with {}",
        "cool formatting",
    );

    let a = &DummyType::Foo;
    let b = &DummyType::Baz;
    ta::assert_ge!(a, a);
    ta::assert_ge!(b, a);

    assert!(catch_unwind(|| ta::assert_ge!(3, 5)).is_err());
    assert!(catch_unwind(|| ta::assert_ge!(DummyType::Foo, DummyType::Bar)).is_err());
}

#[test]
fn test_assert_eq() {
    ta::assert_eq!(4, 4);
    ta::assert_eq!(4, 4);
    ta::assert_eq!(4.0, 4.0);
    ta::assert_eq!(5.1, 5.1);
    ta::assert_eq!("a string", "a string");
    ta::assert_eq!(DummyType::Bar, DummyType::Bar, "Example");
    ta::assert_eq!(
        DummyType::Bar,
        DummyType::Bar,
        "Message with {}",
        "cool formatting",
    );

    let a = &DummyType::Foo;
    let b = &DummyType::Baz;
    ta::assert_eq!(a, a);
    ta::assert_eq!(b, b);

    assert!(catch_unwind(|| ta::assert_eq!(3, 5)).is_err());
    assert!(catch_unwind(|| ta::assert_eq!(3.0, f64::NAN)).is_err());
    assert!(catch_unwind(|| ta::assert_eq!(DummyType::Foo, DummyType::Bar)).is_err());
}

#[test]
fn test_assert_ne() {
    ta::assert_ne!(4, 5);
    ta::assert_ne!(2, 1);
    ta::assert_ne!(4.0, 4.2);
    ta::assert_ne!(5.1, f64::NAN);
    ta::assert_ne!("a string", "b string");
    ta::assert_ne!(DummyType::Foo, DummyType::Bar, "Example");
    ta::assert_ne!(
        DummyType::Bar,
        DummyType::Foo,
        "Message with {}",
        "cool formatting",
    );

    let a = &DummyType::Foo;
    let b = &DummyType::Baz;
    ta::assert_ne!(a, b);
    ta::assert_ne!(b, a);

    assert!(catch_unwind(|| ta::assert_ne!(3, 3)).is_err());
    assert!(catch_unwind(|| ta::assert_ne!(DummyType::Foo, DummyType::Foo)).is_err());
}

#[test]
#[should_panic]
fn test_unreachable() {
    ta::unreachable!()
}

#[test]
#[should_panic]
fn test_assert_fail() {
    ta::assert!({ !true })
}

#[test]
fn test_assert() {
    ta::assert!(true, "help");
    ta::assert!(true, "help");
    ta::assert!(true, "help {}", "me");
}
