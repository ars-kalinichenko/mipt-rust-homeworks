use mini_frunk::hlist::{HCons, HNil};
use mini_frunk::{hlist, hlist_pat, HList};
use static_assertions::assert_type_eq_all as assert_type_eq;

#[test]
#[allow(non_snake_case)]
fn HList_macro() {
    assert_type_eq!(HNil, HList![]);
    assert_type_eq!(
        HCons<u32, HCons<String, HCons<i32, HNil>>>,
        HList![u32, String, i32],
    );
}

#[test]
fn hlist_macro() {
    let expected = HCons {
        head: 1,
        tail: HCons {
            head: "hello",
            tail: HCons {
                head: vec!["world"],
                tail: HNil,
            },
        },
    };
    assert_eq!(HNil, hlist![]);
    assert_eq!(expected, hlist![1, "hello", vec!["world"]]);
}

#[test]
fn hlist_pat_macro() {
    let h: HList!(&str, &str, i32, bool) = hlist!["Joe", "Blow", 30, true];
    let hlist_pat!(f_name, l_name, age, is_admin) = h;
    assert_eq!(f_name, "Joe");
    assert_eq!(l_name, "Blow");
    assert_eq!(age, 30);
    assert_eq!(is_admin, true);
}
