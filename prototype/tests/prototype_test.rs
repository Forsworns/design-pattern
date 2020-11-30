use prototype::QuestionBankController;

#[test]
fn prototype_test() {
    // use `cargo tets -- --nocapture`
    let mut c = QuestionBankController::new();
    println!("{}", c.create_paper("小明", "123456789"));
    println!("{}", c.create_paper("小红", "987654321"));
}
