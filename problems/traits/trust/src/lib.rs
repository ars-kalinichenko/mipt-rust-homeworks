#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoundOutcome {
    BothCooperated,
    LeftCheated,
    RightCheated,
    BothCheated,
}

pub struct Game {
    // TODO: your code goes here.
}

impl Game {
    // pub fn new(left: Box<???>, right: Box<???>) -> Self {
    // TODO: your code goes here.
    // }

    pub fn left_score(&self) -> i32 {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn right_score(&self) -> i32 {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn play_round(&mut self) -> RoundOutcome {
        // TODO: your code goes here.
        unimplemented!()
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CheatingAgent {}

// TODO: your code goes here.

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CooperatingAgent {}

// TODO: your code goes here.

////////////////////////////////////////////////////////////////////////////////

pub struct GrudgerAgent {}

// TODO: your code goes here.

////////////////////////////////////////////////////////////////////////////////

pub struct CopycatAgent {}

// TODO: your code goes here.

////////////////////////////////////////////////////////////////////////////////

pub struct DetectiveAgent {
    // TODO: your code goes here.
}

// TODO: your code goes here.
