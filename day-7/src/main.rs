use petgraph::{csr::NodeIndex, graph::Graph};
use regex::{Captures, Regex};
use std::{
    env,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() -> Result<(), io::Error> {
    let filepath = &env::args().collect::<Vec<String>>()[1];
    let rules = read_lines(filepath)?.collect::<Result<Vec<String>, io::Error>>()?;
    let re = Regex::new(r"([a-z]+\s[a-z]+\sbag)").unwrap();

    let mut a = rules
        .iter()
        .map(|rule| re.captures_iter(&rule).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let graph = rules_to_graph(&mut a);

    Ok(())
}

fn rules_to_graph<'a>(rules: &'a mut Vec<Vec<Captures>>) -> Graph<&'a str, i32> {
    let mut graph = Graph::new();
    let mut gold: NodeIndex<_> = NodeIndex::from(0);

    for rule in rules {
        if rule.len() <= 1 {
            continue;
        }

        let base_str = rule.first().unwrap().get(0).unwrap().as_str();
        let base = graph.add_node(base_str);
        if base_str == "shiny gold bag" {
            gold = base;
        }

        for node in rule.iter().skip(1) {
            let node = graph.add_node(node.get(0).unwrap().as_str());
            graph.add_edge(base, node, 1);
        }
    }

    graph.reverse();
    println!("{:?}", graph.edges(gold).collect::<Vec<_>>());
    for neighbour in graph.neighbors(gold) {
        println!("{:?}", neighbour);
    }

    graph
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
