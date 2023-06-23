use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar="dot.pest"]
struct DotParser;

#[derive(Debug)]
pub enum Type {
    Directed,
    Undirected,
}

#[derive(Debug)]
pub struct Dot {
    pub name: String,
    pub graph_type: Type,
    pub assigments: Vec<(String, String)>,
    pub edges: Vec<(String, String)>,
}

impl Dot {
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Dot {
        let source = std::fs::read_to_string(path).expect("Cannot read input file");

        let graph = DotParser::parse(Rule::graph, &source)
            .expect("unuccessfull parse")
            .next().unwrap();

        let mut inner_rules = graph.into_inner();
        let graph_type = inner_rules.next().unwrap().as_str();
        let graph_name = inner_rules.next().unwrap().as_str();

        let mut assigments = vec![];
        let mut edges = vec![];

        for statement in inner_rules {
            match statement.as_rule() {
                Rule::assigment => {
                    let mut inner_rules = statement.into_inner();
                    let name: &str = inner_rules.next().unwrap().as_str();
                    let value: &str = inner_rules.next().unwrap().as_str();
                    assigments.push((String::from(name), String::from(value)));
                },
                Rule::edge => {
                    let mut inner_rules = statement.into_inner();
                    let mut lhs: &str = inner_rules.next().expect("lhs").as_str();
                    let arrow: &str = inner_rules.next().expect("arrow").as_str();
                    let mut rhs: &str = inner_rules.next().expect("rhs").as_str();

                    if arrow == "<-" {
                        (lhs, rhs) = (rhs, lhs);
                    }

                    assert!(if graph_type == "digraph" { arrow != "--" } else { arrow != "->" && arrow != "<-" });
                    edges.push((String::from(lhs), String::from(rhs)));
                },
                _ => unreachable!(),
            }
        }

        return Self {
            name: String::from(graph_name),
            graph_type: match graph_type {
                "digraph" => Type::Directed,
                "graph" => Type::Undirected,
                _ => unreachable!(),
            },
            assigments,
            edges,
        }
    }
}

impl std::fmt::Display for Dot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "{} {} {{", match self.graph_type {
            Type::Directed => "digraph",
            Type::Undirected => "graph",
        }, self.name)?;

        for (name, value) in &self.assigments {
            writeln!(f, "  {name}={value};")?;
        }

        for (from, to) in &self.edges {
            let arrow = match self.graph_type {
                Type::Directed => "->",
                Type::Undirected => "--",
            };
            writeln!(f, "  {from} {arrow} {to};")?;
        }

        writeln!(f, "}}")
    }
}
