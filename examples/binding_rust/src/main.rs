use tree_sitter::{InputEdit, Parser, Point};

fn main() {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_rust::language())
        .expect("Error loading Rust grammar");

    let source_code = "fn test() {}";
    println!("source_code = {}\n", source_code);

    let mut tree = parser.parse(source_code, None).unwrap();
    println!("tree =\n{:?}\n", tree);
    println!(
        "tree.root_node().to_sexp() =\n{:?}\n",
        tree.root_node().to_sexp()
    );

    let root_node = tree.root_node();

    assert_eq!(root_node.kind(), "source_file");
    assert_eq!(root_node.start_position().column, 0);
    assert_eq!(root_node.end_position().column, 12);
    println!("root_node.kind() = {}", root_node.kind());
    println!("root_node.start_position().column = {}", root_node.start_position().column);
    println!("root_node.end_position().column = {}", root_node.end_position().column);

    let new_source_code = "fn test(a: u32) {}";
    println!("new_source_code = {}\n", new_source_code);

    tree.edit(&InputEdit {
        start_byte: 8,
        old_end_byte: 8,
        new_end_byte: 14,
        start_position: Point::new(0, 8),
        old_end_position: Point::new(0, 8),
        new_end_position: Point::new(0, 14),
    });

    let new_tree = parser.parse(new_source_code, Some(&tree)).unwrap();
    println!("new_tree =\n{:?}\n", new_tree);
    println!(
        "new_tree.root_node().to_sexp() =\n{:?}\n",
        new_tree.root_node().to_sexp()
    );

    let lines = &["pub fn foo() {", "  1", "}"];

    let tree = parser
        .parse_with(
            &mut |_byte: usize, position: Point| -> &[u8] {
                let row = position.row as usize;
                let column = position.column as usize;

                if row < lines.len() {
                    if column < lines[row].as_bytes().len() {
                        &lines[row].as_bytes()[column..]
                    } else {
                        b"\n"
                    }
                } else {
                    &[]
                }
            },
            None,
        )
        .unwrap();

    let sexp = "(source_file (function_item (visibility_modifier) name: (identifier) parameters: (parameters) body: (block (integer_literal))))";
    assert_eq!(tree.root_node().to_sexp(), sexp);

    println!(
        "tree.root_node().to_sexp() =\n{:?}\n",
        tree.root_node().to_sexp()
    );
}
