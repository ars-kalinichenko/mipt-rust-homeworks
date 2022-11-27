mod common;
use common::*;

// Note the symbols import! You shouldn't use one symbol names.
use mini_frunk::labelled::{from_labelled_generic, into_labelled_generic, labelled_convert_from};
use mini_frunk::{field, field::symbols::*, hlist};

#[test]
fn person_from_labelled_generic() {
    let hlist = hlist![
        field!((f, i, r, s, t, __, n, a, m, e), "Humpty"),
        field!((l, a, s, t, __, n, a, m, e), "Drumpty"),
        field!((a, g, e), 3),
    ];
    let person: Person = from_labelled_generic(hlist);
    assert_eq!(
        person,
        Person {
            first_name: "Humpty",
            last_name: "Drumpty",
            age: 3,
        }
    );
}

#[test]
fn struct_into_labelled_generic() {
    let person = Person {
        first_name: "Humpty",
        last_name: "Drumpty",
        age: 3,
    };
    let hlist = into_labelled_generic(person);
    assert_eq!(
        hlist,
        hlist![
            field!((f, i, r, s, t, __, n, a, m, e), "Humpty"),
            field!((l, a, s, t, __, n, a, m, e), "Drumpty"),
            field!((a, g, e), 3),
        ]
    );
}

#[test]
fn strategist_to_president() {
    let strategist = Strategist {
        first_name: "Steve",
        last_name: "Cannon",
        age: 3,
    };
    let president: President = labelled_convert_from(strategist);
    assert_eq!(
        president,
        President {
            first_name: "Steve",
            last_name: "Cannon",
            age: 3,
        }
    )
}

#[test]
fn round_trip() {
    let strategist = Strategist {
        first_name: "Steve",
        last_name: "Cannon",
        age: 3,
    };
    let president: President = labelled_convert_from(strategist.clone());
    let beginning: Strategist = labelled_convert_from(president);
    assert_eq!(beginning, strategist)
}

#[cfg(feature = "compilation-fail-labelled")]
#[test]
fn compilation_fail_labelled() {
    let person = Person {
        first_name: "Humpty",
        last_name: "Drumpty",
        age: 3,
    };
    let _jumbled_person: JumbledPerson = labelled_convert_from(person);
}
