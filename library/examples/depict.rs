use {
    kutil::{cli::depict::*, std::iter::*},
    std::io::*,
};

// It's pretty easy to implement Depict manually

// See: examples/depict_derive.rs for how to automagically derive Depict

struct Node {
    name: String,
    children: Vec<Node>,
}

impl Node {
    fn new(name: &str) -> Self {
        Self::new_with(name, Default::default())
    }

    fn new_with(name: &str, children: Vec<Node>) -> Self {
        Self { name: name.into(), children }
    }
}

impl Depict for Node {
    // You just need to implement this function

    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> Result<()>
    where
        WriteT: Write,
    {
        // We'll use the provided theme
        context.theme.write_string(writer, &self.name)?;

        // The context helps us follow the rules and build a recursive, nested horizontal tree
        for (child, last) in IterateWithLast::new(&self.children) {
            context.indent_into_branch(writer, last)?;
            child.depict(writer, &context.child().increase_indentation_branch(last))?;
        }

        Ok(())
    }
}

pub fn main() {
    let node = Node::new_with(
        "root",
        vec![
            Node::new_with(
                "first",
                vec![
                    Node::new_with("child1", vec![Node::new("grandchild1"), Node::new("grandchild2")]),
                    Node::new_with("child2", vec![Node::new("grandchild3"), Node::new("grandchild4")]),
                ],
            ),
            Node::new("second"),
            Node::new("third"),
        ],
    );

    node.print_default_depiction();
}
