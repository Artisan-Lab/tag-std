use ropey::Rope;
use safety_parser::configuration::{DefinedTag, get_tags};
use tower_lsp_server::lsp_types::Position;
use tree_sitter::{Parser, Tree};

type ByteRange = std::ops::Range<usize>;

#[cfg(test)]
mod tests;

#[derive(Debug)]
struct Attr {
    byte_range: ByteRange,
    start_pos: Position,
    end_pos: Position,
}

pub struct Rust {
    parser: Parser,
    /// Byte range for attributes for the text.
    attrs: Vec<Attr>,
    /// Source code as a rust file.
    text: String,
    /// Text rope.
    rope: Rope,
    tree: Option<Tree>,
    tags: Box<[DefinedTag]>,
}

impl Rust {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Rust {
            parser: init_tree_sitter(),
            attrs: Vec::new(),
            text: String::new(),
            rope: Rope::new(),
            tree: None,
            tags: get_tags(),
        }
    }

    pub fn update_node_tree(&mut self, text: String) -> String {
        self.tree = self.parser.parse(&text, None);
        self.rope = Rope::from_str(&text);
        self.text = text;
        let tree = self.tree.as_ref().unwrap();
        format!("text={:?}\ntree={tree:?}\nroot_node={}", self.text, tree.root_node())
    }

    fn push_attr(&mut self, node: tree_sitter::Node, v: &mut Vec<String>) {
        if node.grammar_name() == "attribute_item" {
            let range = node.byte_range();
            let attr = Attr {
                byte_range: range.clone(),
                start_pos: byte_to_pos(range.start, &self.rope),
                end_pos: byte_to_pos(range.end, &self.rope),
            };
            let src = &self.text[range];
            v.push(format!("src={src:?}\tattr={attr:?}"));
            self.attrs.push(attr);
        }
    }

    pub fn find_attrs(&mut self) -> Vec<String> {
        self.attrs.clear();
        let tree = self.tree.clone();
        let mut v = Vec::new();

        fn search(rust: &mut Rust, v: &mut Vec<String>, cursor: &mut tree_sitter::TreeCursor) {
            rust.push_attr(cursor.node(), v);
            // rust.print_node(cursor);
            if cursor.goto_first_child() {
                search(rust, v, cursor);
                cursor.goto_parent();
            }
            while cursor.goto_next_sibling() {
                search(rust, v, cursor);
            }
        }

        if let Some(tree) = tree {
            let mut cursor = tree.walk();
            if cursor.node().grammar_name() == "source_file" {
                cursor.goto_descendant(1);
                search(self, &mut v, &mut cursor);
            }
        }
        v
    }

    #[allow(dead_code)]
    fn print_node(&mut self, cursor: &mut tree_sitter::TreeCursor<'_>) {
        let node = cursor.node();
        let node_str = node.utf8_text(self.text.as_bytes()).unwrap();
        let node_name = node.grammar_name();
        let descendant_count = node.descendant_count();
        let node_kind = node.kind();
        println!(
            "depth={} descendant_index={} descendant_count={descendant_count} \
             node (name={node_name}, kind={node_kind})={node_str:?}",
            cursor.depth(),
            cursor.descendant_index(),
        );
    }

    /// Returns the attribute string if the cursor is in an attribute scope.
    pub fn get_attr_str(&self, pos: Position) -> Option<String> {
        self.get_attr_range(pos).map(|byte_range| self.text[byte_range].to_owned())
    }

    /// Returns the byte range for the document if the cursor is in an attribute scope.
    pub fn get_attr_range(&self, pos: Position) -> Option<ByteRange> {
        for attr in &self.attrs {
            if pos >= attr.start_pos && pos <= attr.end_pos {
                return Some(attr.byte_range.clone());
            }
        }
        None
    }

    pub fn for_each_tag<T>(&self, f: impl Fn(&DefinedTag) -> T) -> Vec<T> {
        self.tags.iter().map(f).collect()
    }
}

fn init_tree_sitter() -> Parser {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_rust::LANGUAGE.into()).expect("Error loading Rust grammar");
    parser
}

fn byte_to_pos(byte: usize, rope: &Rope) -> Position {
    let line = rope.byte_to_line(byte);
    let character = byte - rope.line_to_byte(line);
    Position { line: line as u32, character: character as u32 }
}
