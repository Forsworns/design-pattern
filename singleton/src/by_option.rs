struct ByOption {
    content: Option<u8>,
}

impl ByOption {
    fn get(&mut self) -> u8 {
        let s = unsafe { std::ptr::replace(&mut self.content, None) };
        s.unwrap()
    }
}

static mut BY_OPTION_SINGLETON: ByOption = ByOption { content: Some(8) };

#[cfg(test)]
#[test]
#[should_panic]
fn test_by_option() {
    unsafe {
        assert_eq!(8, BY_OPTION_SINGLETON.get());
        assert_eq!(8, BY_OPTION_SINGLETON.get());
    }
}
