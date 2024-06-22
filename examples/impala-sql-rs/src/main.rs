use tree_sitter::Parser;

fn main() {
    let code = "SELECT * FROM table_name";

    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_impala::language())
        .expect("Error loading Impala grammar");
    let tree = parser.parse(code, None).unwrap();

    assert!(!tree.root_node().has_error());
    println!("{:?}", tree.root_node().to_sexp());
}
