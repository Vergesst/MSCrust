use core::fmt;
use std::ops::Deref;

macro_rules! self_format {
    ($template:expr,$($arg:tt,)*) => {
        
    };
}

/// This struct should derive Copy and display trait
/// For comparing and something else 
#[derive(PartialEq, Eq, Debug)]
pub struct SelfString {
    inner: Vec<u8>
}

impl SelfString {
    fn new() -> Self {
        SelfString {
            inner: Vec::new()
        }
    }

    pub fn as_str(&self) -> &str {
        unsafe {
            std::str::from_utf8_unchecked(
                self.inner.as_slice()
            )
        }
    }
}

impl From<String> for SelfString {
    fn from(origin: String) -> Self {
        SelfString { inner: origin.into_bytes() }
    }
}

impl From<&str> for SelfString {
    fn from(origin: &str) -> Self {
        SelfString::from(String::from(origin))
    }
}

impl Deref for SelfString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl fmt::Display for SelfString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

mod test {
    #[test]
    fn macro_test() {
        use super::SelfString;

        let res = SelfString::from("this");
        assert_eq!(res, SelfString::from(format!("this")));
    }
}


