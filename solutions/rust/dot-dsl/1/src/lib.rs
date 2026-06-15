pub mod graph {
    use std::collections::HashMap;
    pub struct Graph {
        pub edges : Vec<self::graph_items::edge::Edge>,
        pub nodes : Vec<self::graph_items::node::Node>,
        pub attrs : HashMap<&'static str, &'static str>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                edges: Vec::new(),
                nodes: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_attrs(mut self, attrs: impl AsRef<[(&'static str, &'static str)]>) -> Self {
            for (k,v) in attrs.as_ref() {
                self.attrs.insert(*k, *v);
            }
            self
        }

        pub fn  with_nodes(mut self, nodes: impl AsRef<[self::graph_items::node::Node]> ) -> Self {
            for node in nodes.as_ref() {
                self.nodes.push(node.clone());
            }
            self
        }

        pub fn with_edges(mut self, edges: impl AsRef<[self::graph_items::edge::Edge]> ) -> Self {
            for edge in edges.as_ref() {
                self.edges.push(edge.clone());
            }
            self
        }

        pub fn node(&self, name: &str) -> Option<&self::graph_items::node::Node> {
            self.nodes.iter().find(|&node| node.name() == name)
        }

    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;
            #[derive(Debug, Clone, PartialEq)]
            pub struct Edge {
                nodes: (String, String),
                attrs: HashMap<&'static str, &'static str>,
            }

            impl Edge {
                pub fn new(node1: &str, node2: &str) -> Self {
                    Self {
                        nodes: (node1.to_string(), node2.to_string()),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: impl AsRef<[(&'static str, &'static str)]>) -> Self {
                    for (k,v) in attrs.as_ref() {
                        self.attrs.insert(*k, *v);
                    }
                    self
                }

                pub fn attr(&self, key : &str) -> Option<&str> {
                    self.attrs.get(key).copied()
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Node {
                name: String,
                attrs: HashMap<&'static str, &'static str>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Self {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: impl AsRef<[(&'static str, &'static str)]>) -> Self {
                    for (k,v) in attrs.as_ref() {
                        self.attrs.insert(*k, *v);
                    }
                    self
                }

                pub fn attr(&self, key : &str) -> Option<&str> {
                    self.attrs.get(key).copied()
                }

                pub fn name(&self) -> &str {
                    self.name.as_str()
                }
            }
        }
        
    }
}
