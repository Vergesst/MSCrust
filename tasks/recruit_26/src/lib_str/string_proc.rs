use core::fmt;
use std::ops::Deref;

#[macro_export]
macro_rules! self_format {
    ($($arg:tt)*) => {
        $crate::lib_str::string_proc::SelfString::from(format!($($arg)*))
    }
}

/// This struct should derive Copy and display trait
/// for comparing and something else
/// And this struct will work as a simplified String
/// with from<String> and from<str> 
#[derive(PartialEq, Eq, Debug)]
pub struct SelfString {
    inner: Vec<u8>
}

impl SelfString {
    pub fn new() -> Self {
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

// for comparison
impl PartialEq<String> for SelfString {
    fn eq(&self, other: &String) -> bool {
        self.inner.as_slice() == other.as_bytes()
    }
}

impl PartialEq<SelfString> for String {
    fn eq(&self, other: &SelfString) -> bool {
        self.as_bytes() == other.as_bytes()
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
    fn from_test() {
        use super::SelfString;

        // build from slice --- &str
        let res = SelfString::from("this");
        // build from String
        assert_eq!(res, SelfString::from(format!("this")));
    }

    #[test]
    fn format_test() {
        let var = 3;
        let res_str = format!("nothing {}", var);

        // macro --- self_format!
        let res_sstr = self_format!("nothing {}", var);
        
        // test for `impl PartialEq<String> for SelfString`
        assert_eq!(res_sstr, res_str);
        // test for `impl PartialEq<SelfString> for String`
        assert_eq!(res_str, res_sstr);
    }
}


