pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[crabtime::function]
pub fn gen_struct(name: String, fields: Vec<String>, types: Vec<String>) {
    let mut field_defs = Vec::new();
    let mut fn_args = Vec::new();
    let mut assigns = Vec::new();

    for (fname, ftype) in fields.iter().zip(types.iter()) {
        field_defs.push(format!("    {}: {},", fname, ftype));
        fn_args.push(format!("{}: {}", fname, ftype));
        assigns.push(format!("{},", fname));
    }

    let struct_fields = field_defs.join("\n");
    let new_args = fn_args.join(", ");
    let assignments = assigns.join("\n");

    crabtime::output! {
        struct {{name}} {
            {{struct_fields}}
        }

        impl {{name}} {
            pub fn new({{new_args}}) -> Self {
                Self {
                    {{assignments}}
                }
            }
        }
    }
}

#[crabtime::function]
pub fn gen_hello() -> &'static str {
    "pub fn hello() { println!(\"Hello\"); }"
}

gen_struct!(User, ["name", "age"], ["String", "u32"]);
gen_hello!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn nicehello() {
        hello();
    }

    #[test]
    fn test_user_struct() {
        let user = User::new("Alice".into(), 30);
        assert_eq!(user.name, "Alice");
        assert_eq!(user.age, 30);
    }
}
