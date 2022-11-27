mod common;
use common::*;
use mini_frunk::transmogrify::{transmogrify_from, Plucker, Sculptor, Transmogrifier};
use mini_frunk::{hlist, HList};

#[test]
fn pluck_basic() {
    let hlist = hlist![1, "hello", true, 42f32];
    let (t, r): (f32, _) = hlist.pluck();
    assert_eq!(t, 42f32);
    assert_eq!(r, hlist![1, "hello", true])
}

#[test]
fn sculpt_basic() {
    let hlist = hlist![9000, "joe", 41f32, true];
    let (reshaped, remainder): (HList![f32, i32, &str], _) = hlist.sculpt();
    assert_eq!(reshaped, hlist![41f32, 9000, "joe"]);
    assert_eq!(remainder, hlist![true]);
}

#[test]
fn aligned_labelled_transmogrify_from() {
    let person = Person {
        first_name: "Moe",
        last_name: "Ali",
        age: 30,
    };
    let jumbled_person: JumbledPerson = transmogrify_from(person);
    assert_eq!(jumbled_person.first_name, "Moe");
    assert_eq!(jumbled_person.last_name, "Ali");
    assert_eq!(jumbled_person.age, 30);
}

#[test]
fn non_aligned_transmogrify_from() {
    let long_person = LongPerson {
        first_name: "Moe",
        last_name: "Ali",
        bank_title: "Rust & Ferris",
        account_balance: 1000000.42,
    };
    let short_person: ShortPerson = transmogrify_from(long_person);
    assert_eq!(short_person.first_name, "Moe");
    assert_eq!(short_person.last_name, "Ali");
}

#[test]
fn aligned_labelled_transmogrify() {
    let person = Person {
        first_name: "Moe",
        last_name: "Ali",
        age: 30,
    };
    let jumbled_person: JumbledPerson = person.transmogrify();
    assert_eq!(jumbled_person.first_name, "Moe");
    assert_eq!(jumbled_person.last_name, "Ali");
    assert_eq!(jumbled_person.age, 30);
}

#[test]
fn non_aligned_transmogrify() {
    let long_person = LongPerson {
        first_name: "Moe",
        last_name: "Ali",
        bank_title: "Rust & Ferris",
        account_balance: 1000000.42,
    };
    let short_person: ShortPerson = long_person.transmogrify();
    assert_eq!(short_person.first_name, "Moe");
    assert_eq!(short_person.last_name, "Ali");
}

#[cfg(feature = "compilation-fail-transmogrify")]
#[test]
fn compilation_fail_transmogrify() {
    use mini_frunk::{hlist, HList};
    let h = hlist![9000, "joe", 41f32, true];
    let (reshaped, remainder): (HList![f32, i32, &str], _) = h.sculpt();
    assert_eq!(reshaped, hlist![41f32, 9000, "joe"]);
    assert_eq!(remainder, hlist![true]);
    let (reshaped, _): (HList![char], _) = h.sculpt();
}
