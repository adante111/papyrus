use super::*;
use crate::code::*;

#[test]
fn test_unclosed_delimiter() {
    assert_eq!(parse_program("fn foo() {"), InputResult::More);
    assert_eq!(parse_program("("), InputResult::More);
    assert_eq!(parse_program("{"), InputResult::More);
    assert_eq!(parse_program("let a = ("), InputResult::More);
    assert_eq!(parse_program("let a = {"), InputResult::More);
    assert_eq!(parse_program("let a = foo("), InputResult::More);
    assert_eq!(parse_program("let a = \""), InputResult::More);
}

#[cfg(feature = "format")] // have to turn formatting on to check this
#[test]
fn test_items() {
    assert_eq!(
        parse_program("fn b() {}"),
        InputResult::Program(Input {
            items: vec![("fn b() {}".to_string(), false)],
            stmts: vec![],
            crates: vec![]
        })
    ); // Item::Fn
    assert_eq!(
        parse_program("#[derive(Debug)]\nstruct A {\n\tu: u32\n}"),
        InputResult::Program(Input {
            items: vec![(
                r##"#[derive(Debug)]
struct A {
    u: u32,
}"##
                .to_string(),
                false
            )],
            stmts: vec![],
            crates: vec![]
        })
    ); // Item::Struct
    assert_eq!(
        parse_program("extern crate rand as r;"),
        InputResult::Program(Input {
            items: vec![],
            stmts: vec![],
            crates: vec![CrateType::parse_str(&"extern crate rand as r;").unwrap()]
        })
    ); // Item::ExternCrate
    assert_eq!(
        parse_program("impl Eq for MyStruct {}"),
        InputResult::Program(Input {
            items: vec![("impl Eq for MyStruct {}".to_string(), false)],
            stmts: vec![],
            crates: vec![]
        })
    ); // Item::Impl
    assert_eq!(
        parse_program("use something::a::crate;"),
        InputResult::Program(Input {
            items: vec![("use something::a::crate;".to_string(), false)],
            stmts: vec![],
            crates: vec![]
        })
    ); // Item::Use
    assert_eq!(
        parse_program("#![feature(test)]"),
        InputResult::Program(Input {
            items: vec![("#![feature(test)]".to_string(), true)],
            stmts: vec![],
            crates: vec![]
        })
    ); // Item::Use
}

#[cfg(feature = "format")] // have to turn formatting on to check this
#[test]
fn test_exprs() {
    // Expr::Binary
    assert_eq!(
        parse_program("2+2"),
        InputResult::Program(Input {
            items: vec![],
            stmts: vec![Statement {
                expr: "2 + 2".to_string(),
                semi: false
            }],
            crates: vec![]
        })
    );
    assert_eq!(
        parse_program("2+2;"),
        InputResult::Program(Input {
            items: vec![],
            stmts: vec![Statement {
                expr: "2 + 2".to_string(),
                semi: true
            }],
            crates: vec![]
        })
    );
    // Expr::Macro
    assert_eq!(
        parse_program("println!(\"hello\")"),
        InputResult::Program(Input {
            items: vec![],
            stmts: vec![Statement {
                expr: "println!(\"hello\")".to_string(),
                semi: false
            }],
            crates: vec![]
        })
    );
    assert_eq!(
        parse_program("println!(\"hello\");"),
        InputResult::Program(Input {
            items: vec![],
            stmts: vec![Statement {
                expr: "println!(\"hello\")".to_string(),
                semi: true
            }],
            crates: vec![]
        })
    );
    // Expr::Tuple
    assert_eq!(
        parse_program("()"),
        InputResult::Program(Input {
            items: vec![],
            stmts: vec![Statement {
                expr: "()".to_string(),
                semi: false
            }],
            crates: vec![]
        })
    );
    assert_eq!(
        parse_program("();"),
        InputResult::Program(Input {
            items: vec![],
            stmts: vec![Statement {
                expr: "()".to_string(),
                semi: true
            }],
            crates: vec![]
        })
    );
    // Expr::Call
    assert_eq!(
        parse_program("f()"),
        InputResult::Program(Input {
            items: vec![],
            stmts: vec![Statement {
                expr: "f()".to_string(),
                semi: false
            }],
            crates: vec![]
        })
    );
    assert_eq!(
        parse_program("f();"),
        InputResult::Program(Input {
            items: vec![],
            stmts: vec![Statement {
                expr: "f()".to_string(),
                semi: true
            }],
            crates: vec![]
        })
    );
    // LET
    assert_eq!(
        parse_program("let a = 1;"),
        InputResult::Program(Input {
            items: vec![],
            stmts: vec![Statement {
                expr: "let a = 1".to_string(),
                semi: true
            }],
            crates: vec![]
        })
    );
    // Expr::ForLoop
    assert_eq!(
        parse_program("for i in 0..3 {}"),
        InputResult::Program(Input {
            items: vec![],
            stmts: vec![Statement {
                expr: "for i in 0..3 {}".to_string(),
                semi: false
            }],
            crates: vec![]
        })
    );
    // Expr::Path
    assert_eq!(
        parse_program("b"),
        InputResult::Program(Input {
            items: vec![],
            stmts: vec![Statement {
                expr: "b".to_string(),
                semi: false
            }],
            crates: vec![]
        })
    );
    assert_eq!(
        parse_program("b;"),
        InputResult::Program(Input {
            items: vec![],
            stmts: vec![Statement {
                expr: "b".to_string(),
                semi: true
            }],
            crates: vec![]
        })
    );
    // Expr::MethodCall
    assert_eq!(
        parse_program("std::env::current_dir()"),
        InputResult::Program(Input {
            items: vec![],
            stmts: vec![Statement {
                expr: "std::env::current_dir()".to_string(),
                semi: false
            }],
            crates: vec![]
        })
    );
    assert_eq!(
        parse_program("std::env::current_dir();"),
        InputResult::Program(Input {
            items: vec![],
            stmts: vec![Statement {
                expr: "std::env::current_dir()".to_string(),
                semi: true
            }],
            crates: vec![]
        })
    );
}

#[test]
fn test_determine_result() {
    assert_eq!(
        determine_result(":help", ":help", false),
        InputResult::Command("help".to_string())
    );
    assert_eq!(
        determine_result(":another", ":another", false),
        InputResult::Command("another".to_string())
    );
    assert_eq!(
        determine_result(":help cmd", ":help cmd", false),
        InputResult::Command("help cmd".to_string())
    );
    assert_eq!(determine_result("", "", false), InputResult::Empty);
    assert_eq!(
        determine_result("2+2", "2+2", false),
        InputResult::Program(Input {
            items: Vec::new(),
            stmts: vec![Statement {
                expr: "2 + 2".to_string(),
                semi: false,
            }],
            crates: Vec::new()
        })
    );
    assert_eq!(
        determine_result("let a = 1;", "let a = 1;", false),
        InputResult::More
    );
    assert_eq!(determine_result("{", "{", false), InputResult::More);
}

#[test]
fn fail_parse_program() {
    assert_eq!(
        parse_program("extern crate "),
        InputResult::InputError("unexpected end of input, expected identifier".to_string())
    );
    assert_eq!(
        parse_program("let a = 1"),
        InputResult::InputError("expected `;`".to_string())
    );
}
