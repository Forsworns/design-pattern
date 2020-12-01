use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref BY_LAZY_SINGLETON: Mutex<HashMap<u8, &'static str>> = Mutex::new({
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    });
}

#[cfg(test)]
#[test]
fn test_by_lazy() {
    assert_eq!("foo", *BY_LAZY_SINGLETON.lock().unwrap().get(&0).unwrap());
}
