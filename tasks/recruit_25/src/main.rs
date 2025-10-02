/// preparation task
/// use the main function to display your rust toolchain
/// hint: achieve this goal with std::command::Process
use std::process::Command;

fn main() -> Result<(), std::io::Error> {
    let mut cmd = Command::new("rustup");
    cmd.arg("toolchain").arg("list").status()?;

    Ok(())
}

mod test {

/// Add use ans mod statement
/// to make the test functions work,
/// just like this 
/// ```rust
/// use std::collections::HashMap;
/// let map = HashMap::<i32, String>::new();
/// ```
    #[test]
    fn test_lib1() {}
}