use builder::Builder;

#[test]
fn builder_test() {
    let bd = Builder {};
    // use `cargo test -- --nocapture`
    println!("{}", bd.europe(233.0).get_detail());
    println!("{}", bd.luxury(233.0).get_detail());
    println!("{}", bd.modern(233.0).get_detail());
}
