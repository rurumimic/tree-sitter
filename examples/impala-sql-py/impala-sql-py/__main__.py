import tree_sitter_impala as tsimpala
from tree_sitter import Language, Parser

source_code = "select * from table_name;"

def main():
    PY_LANGUAGE = Language(tsimpala.language())
    parser = Parser(PY_LANGUAGE)

    tree = parser.parse(bytes(source_code, "utf8"))

    root_node = tree.root_node
    print(root_node)

if __name__ == "__main__":
    main()

