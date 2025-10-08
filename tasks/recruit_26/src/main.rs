use std::process::Command;

/// preparation task
/// use the main function to display your rust toolchain
/// hint: achieve this goal with std::command::Process
fn main() -> Result<(), std::io::Error> {
    let mut cmd = Command::new("rustup");
    cmd.arg("toolchain").arg("list").status()?;
    Ok(())
}

// inline module
struct InlineModule;
impl InlineModule {
    pub fn greet() -> &'static str {
        "Hello From Rust"
    }
}

mod lib_str;
mod lib_greet;
mod test {
/// This is about code organization
/// Add use ans mod statement
/// to make the test functions work,
/// just like this 
/// ```rust
/// use std::collections::HashMap;
/// let map = HashMap::<i32, String>::new();
/// ```
    #[test]
    fn test_lib() {
        use super::InlineModule;
        use super::lib_greet::organization::*;

        // inline module
        assert_eq!("Hello From Rust", InlineModule::greet());

        // Outer module
        assert_eq!("Hello From Cargo", Organization::greet());
    }

/// This is tests for SelfString struct
/// your `self_format!` macro and other functions about SelfString
/// your SelfString should works like a simple string
/// # Example Code
/// ```rust
/// let var = 3;
/// let res_s_str = self_format!("nothing {}", var);
/// let res_s_str = SelfString::from("static str"); // from simple &'a static str
/// let res_s_str = SelfString::from(format!("string is"));
/// ```
    #[test]
    fn test_str() {
        use crate::self_format;
        use super::lib_str::string_proc::*;

        let var = 3;
        let res_s_str = self_format!("nothing {}", var);

        let res_str = format!("nothing {}", var);

        assert_eq!(res_s_str, res_str);
        assert_eq!(res_str, res_s_str);

        // from trait
        let mut f_str = SelfString::new(); 
        f_str = SelfString::from("from str");
        let f_string = SelfString::from("from string");

        assert_eq!(f_str, self_format!("from str"));
        assert_eq!(f_string, self_format!("from string"));
    }
}