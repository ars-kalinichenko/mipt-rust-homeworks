use context::Context;

////////////////////////////////////////////////////////////////////////////////

trait SayHi {
    fn say_hi(&self) -> &str;
}

struct Greeter {}

impl SayHi for Greeter {
    fn say_hi(&self) -> &str {
        "hi!"
    }
}

////////////////////////////////////////////////////////////////////////////////

#[test]
fn singletones() {
    let mut cx = Context::new();

    cx.insert_singleton(64i64);
    cx.insert_singleton(32i32);
    assert_eq!(*cx.get_singleton::<i64>(), 64);
    assert_eq!(*cx.get_singleton::<i32>(), 32);

    cx.insert_singleton(Box::new(Greeter {}) as Box<dyn SayHi>);
    assert_eq!(cx.get_singleton::<Box<dyn SayHi>>().say_hi(), "hi!");

    cx.insert_singleton::<Box<[u8]>>(Box::new(*b"binary data"));
    assert_eq!(
        cx.get_singleton::<Box<[u8]>>() as &[u8],
        b"binary data" as &[u8]
    );

    cx.insert_singleton("hello, world!");
    assert_eq!(*cx.get_singleton::<&'static str>(), "hello, world!");
    cx.insert_singleton("foo bar");
    assert_eq!(*cx.get_singleton::<&'static str>(), "foo bar");
}

#[test]
fn key() {
    let mut cx = Context::new();

    cx.insert("x", 128i32);
    cx.insert("y", 255i32);
    assert_eq!(*cx.get::<i32>("x"), 128);
    assert_eq!(*cx.get::<i32>("y"), 255);

    cx.insert_singleton(372i32);
    assert_eq!(*cx.get_singleton::<i32>(), 372);

    cx.insert("z", 100i32);
    assert_eq!(*cx.get::<i32>("z"), 100);
    assert_eq!(*cx.get::<i32>("x"), 128);
    assert_eq!(*cx.get::<i32>("y"), 255);

    cx.insert("my_str", "my favourite str");
    assert_eq!(*cx.get::<&'static str>("my_str"), "my favourite str");

    assert_eq!(*cx.get_singleton::<i32>(), 372);

    let key = "foo".to_string();
    cx.insert(key.as_ref(), true);
    assert_eq!(*cx.get::<bool>(&key), true);
}

#[test]
#[should_panic]
fn get_missing() {
    let cx = Context::new();
    cx.get::<Greeter>("greeter");
}

#[test]
#[should_panic]
fn get_missing_singletone() {
    let cx = Context::new();
    cx.get_singleton::<Greeter>();
}

#[test]
#[should_panic]
fn wrong_type() {
    let mut cx = Context::new();
    cx.insert("greeter", Greeter {});
    cx.get::<usize>("greeter");
}