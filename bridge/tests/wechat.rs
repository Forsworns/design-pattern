use bridge::{WeChat, Pay, Face, Cypher, FingerPrint};

// use `cargo test -- --nocapture`
#[test]
fn face() {
    let mode = Face{};
    let pay = WeChat::new(mode);
    pay.transfer("12306", "123", 100);
}

#[test]
fn cypher() {
    let mode = Cypher{};
    let pay = WeChat::new(mode);
    pay.transfer("12306", "123", 100);
}

#[test]
fn finger_print() {
    let mode = FingerPrint{};
    let pay = WeChat::new(mode);
    pay.transfer("12306", "123", 100);
}