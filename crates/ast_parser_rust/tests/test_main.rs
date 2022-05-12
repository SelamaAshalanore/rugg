#[cfg(test)]
mod tests {
    use staticanalyzer::code_to_dot_digraph;

    #[test]
    fn parse_simple_r_code() {
        let code: &str = r#"
        fn main() {
            println!("Hello, world!");
        }
        "#;
        assert_eq!(
            code_to_dot_digraph(code), 
r#"digraph ast {
    main[label="main"];
}
"#
        )
    }

    
    #[test]
    fn parse_class_with_fn() {
        let code: &str = r#"
            pub struct Mock;
            impl Mock {
                pub fn mock_fn() {}
            }
        "#;
        assert_eq!(
            code_to_dot_digraph(code), 
r#"digraph ast {
    Mock[label="{Mock|mock_fn()}"][shape="record"];
}
"#
        )
    }

    #[test]
    fn parse_class_with_nested_impl() {
        let code: &str = r#"
            pub struct Mock;
            impl Mock {
                pub fn mock_fn() { f1(f2()) }    
            }
            fn f1(i: usize) {}
            fn f2() -> usize { 0 }
        "#;
        assert_eq!(
            code_to_dot_digraph(code), 
r#"digraph ast {
    Mock[label="{Mock|mock_fn()}"][shape="record"];
    f1[label="f1"];
    f2[label="f2"];
    f2 -> Mock[label=""][style="dashed"][arrowhead="vee"];
    f1 -> Mock[label=""][style="dashed"][arrowhead="vee"];
}
"#
        )
    }

    #[test]
    fn test_fn_dependency() {
        let code: &str = r#"
            fn main() {
                hello();
            }
            fn hello() {}
        "#;
        assert_eq!(
            code_to_dot_digraph(code), 
r#"digraph ast {
    main[label="main"];
    hello[label="hello"];
    main -> hello[label=""][style="dashed"][arrowhead="vee"];
}
"#
        )
    }

    #[test]
    fn test_fns_dependency() {
        let code: &str = r#"
            fn main() {
                f1();
                f2();
            }
            fn f1() {}
            fn f2() {}
        "#;
        assert_eq!(
            code_to_dot_digraph(code), 
r#"digraph ast {
    main[label="main"];
    f1[label="f1"];
    f2[label="f2"];
    main -> f2[label=""][style="dashed"][arrowhead="vee"];
    main -> f1[label=""][style="dashed"][arrowhead="vee"];
}
"#
        )
    }

    #[test]
    fn test_nested_fn_call_dependency() {
        let code: &str = r#"
            fn main() {
                f1(f2());
            }
            fn f1(i: usize) {}
            fn f2() -> usize { 0 }
        "#;
        assert_eq!(
            code_to_dot_digraph(code), 
r#"digraph ast {
    main[label="main"];
    f1[label="f1"];
    f2[label="f2"];
    main -> f2[label=""][style="dashed"][arrowhead="vee"];
    main -> f1[label=""][style="dashed"][arrowhead="vee"];
}
"#
        )
    }
}