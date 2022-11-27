mod common;
use common::*;
use mini_frunk::generic::{convert_from, from_generic, into_generic};
use mini_frunk::hlist;

#[test]
fn person_into_generic() {
    let person: Person = from_generic(hlist!("Humpty", "Drumpty", 3));
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
fn person_from_generic() {
    let person = Person {
        first_name: "Humpty",
        last_name: "Drumpty",
        age: 3,
    };
    let hlist = into_generic(person);
    assert_eq!(hlist, hlist!("Humpty", "Drumpty", 3));
}

#[test]
fn strategist_to_president() {
    let strategist = Strategist {
        first_name: "Steve",
        last_name: "Cannon",
        age: 3,
    };
    let president: President = convert_from(strategist);
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
    let president: President = convert_from(strategist.clone());
    let beginning: Strategist = convert_from(president);
    assert_eq!(beginning, strategist)
}

#[test]
fn mixed_conversions_round_trip() {
    let saved_user = SavedUser {
        first_name: "Humpty",
        last_name: "Drumpty",
        age: 3,
    };
    let api_user: ApiUser = convert_from(saved_user.clone());
    let beginning: SavedUser = convert_from(api_user);
    assert_eq!(beginning, saved_user)
}

#[test]
fn jumbled_fields() {
    let person: Person = from_generic(hlist!("Humpty", "Drumpty", 3));
    let jumbled_person: JumbledPerson = convert_from(person.clone());
    assert_eq!(person.first_name, jumbled_person.last_name);
}

#[cfg(feature = "compilation-fail-generic")]
#[test]
fn compilation_fail_generic() {
    let person = Person {
        first_name: "Humpty",
        last_name: "Drumpty",
        age: 3,
    };
    let _short_person: ShortPerson = convert_from(person);
}
