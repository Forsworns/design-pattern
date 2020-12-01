use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

static BY_ONCE_SINGLETON: Lazy<Mutex<HashMap<u8, &'static str>>> = Lazy::new(|| {
    Mutex::new({
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    })
});

#[cfg(test)]
#[test]
fn test_by_once() {
    assert_eq!("foo", *BY_ONCE_SINGLETON.lock().unwrap().get(&0).unwrap());
}
