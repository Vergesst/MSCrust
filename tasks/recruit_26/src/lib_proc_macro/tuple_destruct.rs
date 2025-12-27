#[macro_export]
macro_rules! num_str {
    ("") => {
        compile_error!("empty string (\"\") is not a valid input")
    };
    ($inner:expr) => {
        {
            let inner: ::std::string::String = $inner.into();
            if inner.chars().all(|c| c.is_ascii_digit()) {
                $crate::lib_proc_macro::tuple_destruct::NumString{inner}
            } else {
                // compile_error!("should never contains non-digit character")
                $crate::lib_proc_macro::tuple_destruct::NumString{inner}

            }
        }
    };
}

#[derive(Debug)]
pub struct NumString {
    inner: String
}

impl NumString {
    fn new(inner: String) -> Result<Self, String> {
        if !inner.is_empty() && inner.chars().all(|c| c.is_ascii_digit()) {
            Ok(NumString {inner})
        } else {
            Err(format!("INVALID FORMAT: {} is not a valid number string", inner))
        }
    }
}

#[test]
fn test_num_str() {
    let a = NumString::new("as".to_string());
    let b = num_str!("123");
    print!("{:?}", b);
    let _ = format!("");
}

// a fixed struct
pub struct FixedStruct {
    name: String,
    age: i32,
    phone_num: NumString,
}