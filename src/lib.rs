pub mod graph {
    use graph_items::node::Node;
    use graph_items::edge::Edge;

    pub struct Graph<'a> {
        pub nodes: Vec<Node<'a>>,
        pub edges: Vec<Edge<'a>>,
        pub attrs: Vec<(String, String)>,
    }

    impl Graph<'_> {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: vec![],
            }
        }

        pub fn node(self, label: &str) -> Node {
            Node { label }
        }

        pub fn with_nodes(mut self, nodes: Vec<Node>) -> Self {
            self
        }
    }

    pub mod graph_items {
        pub mod node {
            #[derive(Debug, Eq, PartialEq)]
            pub struct Node<'a> {
                pub label: &'a str,
            }

            impl Node<'_> {
                pub fn new(label: &str) -> Node {
                    Node { label }
                }
            }
        }

        pub mod edge {
            #[derive(Debug, Eq, PartialEq)]
            pub struct Edge<'a> {
                pub from: &'a str,
                pub to: &'a str,
            }

            impl Edge<'_> {
                pub fn new<'a>(from: &'a str, to: &'a str) -> Edge<'a> {
                    Edge { from, to }
                }
            }
        }
    }
}
