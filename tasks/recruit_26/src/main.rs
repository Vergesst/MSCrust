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

    #[test]
    fn test_str() {
    }
}