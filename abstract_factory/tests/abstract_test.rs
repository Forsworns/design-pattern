use abstract_factory::AbstractFactory;

#[test]
fn adapter_test() {
    let adapter_fact = AbstractFactory::get_factory("adapter").unwrap();
    let egm = adapter_fact.create("EGM").unwrap();
    egm.get("".into());
    let iir = adapter_fact.create("IIR").unwrap();
    iir.get("".into());
}
