/*
#[derive(Clone)]
pub struct Letter {
    text: String,
}

pub struct Envelope {
    letter: Option<Letter>,
}

pub struct PickupLorryHandle {
    done: bool,
}

impl Letter {
    pub fn new(text: String) -> Self {
        Letter { text: text }
    }
}

impl Envelope {
    pub fn wrap(&mut self, letter: &Letter) {
        self.letter = Some(letter.clone());
    }
}

pub fn buy_prestamped_envelope() -> Envelope {
    Envelope { letter: None }
}

impl PickupLorryHandle {
    pub fn pickup(&mut self, envelope: &Envelope) {
        // give letter
    }

    pub fn done(&mut self) {
        self.done = true;
        println!("sent");
    }
}

pub fn order_pickup() -> PickupLorryHandle {
    PickupLorryHandle { done: false, }
}

fn main() {
    let letter = Letter::new(String::from("Dear RustFest"));
    let mut envelope = buy_prestamped_envelope();
    envelope.wrap(&letter);
    let mut lorry = order_pickup();
    lorry.pickup(&envelope);
    lorry.done();
}
*/
#![allow(dead_code)]

pub struct Letter {
    text: String,
}
pub struct EmptyEnvelope {}
pub struct ClosedEnvelope { letter: Letter }
pub struct PickupLorryHandle { done: bool }
impl Letter {
    pub fn new(text: String) -> Self {
        Letter { text: text }
    }
}
impl EmptyEnvelope {
    pub fn wrap(self, letter: Letter) -> ClosedEnvelope {
        ClosedEnvelope { letter: letter }
    }
}
pub fn buy_prestamped_envelope() -> EmptyEnvelope {
    EmptyEnvelope { }
}
impl PickupLorryHandle {
    pub fn pickup(&mut self, _envelope: ClosedEnvelope) {
        // give letter
    }

    pub fn done(&mut self) {
        self.done = true;
        println!("sent");
    }
}
impl Drop for PickupLorryHandle {
    fn drop (&mut self) { println!("sent"); }
}
pub fn order_pickup() -> PickupLorryHandle {
    PickupLorryHandle { done: false, }
}
fn main() {
    let letter = Letter::new(String::from("Dear RustFest"));
    let envelope = buy_prestamped_envelope();
    let closed_envelope = envelope.wrap(letter);
    let mut lorry = order_pickup();
    lorry.pickup(closed_envelope);
}