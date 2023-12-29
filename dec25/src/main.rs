use std::{collections::{HashMap, HashSet}, hash::Hash};

use input_curler::input_for;

type Label<'a> = Vec<&'a str>;

#[derive(Debug)]
struct Node<'a> {
    labels: Label<'a>,
    edges: Vec<(Label<'a>, usize)>
}
impl<'a> Node<'a> {
    fn new(labels: Vec<&'a str>) -> Self {
        Self {
            labels,
            edges: vec![]
        }
    }
}
impl<'a> PartialEq for Node<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.labels == other.labels
    }
}
impl<'a> Eq for Node<'a> {}
impl<'a> Hash for Node<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.labels.hash(state);
    }
}

type Network<'a> = HashMap<Label<'a>, Node<'a>>;

#[derive(Debug, PartialEq, Clone)]
struct Cut {
    side_one: Vec<String>,
    side_two: Vec<String>,
    size: usize
}
impl PartialOrd for Cut {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.size.partial_cmp(&other.size)
    }
}

fn main() {
    let data = input_for(25).unwrap();

    let mut network = parse_data(&data);

    let best_cut = stoer_wagner(&mut network);
    println!("Answer: {}", best_cut.side_one.len() * best_cut.side_two.len());
}

fn parse_data(data: &str) -> Network {
    let mut network = Network::new();

    for line in data.lines() {
        let (source, dest_part) = line.split_once(':').unwrap();
        let dests = dest_part.split_whitespace();

        for dest in dests {
            {
                let source_node = network.entry(vec![source]).or_insert(Node::new(vec![source]));
                source_node.edges.push((vec![dest], 1));
            }
            {
                let dest_node = network.entry(vec![dest]).or_insert(Node::new(vec![dest]));
                dest_node.edges.push((vec![source], 1));
            }
        }
    }

    network
}

fn stoer_wagner<'a>(network: &'a mut Network) -> Cut {
    let mut best_cut = Cut {
        side_one: vec![],
        side_two: vec![],
        size: usize::MAX
    };

    loop {
        if network.len() < 2 { break; }
        let cut = find_cut_and_reduce(network);
        // if cut < best_cut {
        //     best_cut = cut;
        // }
        if cut.size == 3 {
            best_cut = cut;
            break;
        }
    }

    best_cut
}

fn find_cut_and_reduce<'a>(network: &'a mut Network) -> Cut {

    let mut picked_nodes = HashSet::from([network.values().next().unwrap()]);
    let mut last_node: Option<&Node> = None;
    let mut penult_node: Option<&Node> = None;

    while picked_nodes.len() < network.len() {
        let all_edges = picked_nodes.iter().flat_map(|&node|
            &node.edges
        );

        let next_label = all_edges
            .filter(|edge| !picked_nodes.contains(network.get(&edge.0).unwrap()))
            .fold(HashMap::<&Label, usize>::new(), |mut acc, edge| {
                acc.entry(&edge.0).and_modify(|size| *size += edge.1).or_insert(edge.1);
                acc
            })
            .iter()
            .max_by_key(|edge_set| edge_set.1)
            .map(|edge_set| *edge_set.0)
            .unwrap();

        let next_node = network.get(next_label).unwrap();
        penult_node = last_node;
        last_node = Some(next_node);
        picked_nodes.insert(next_node);
    }

    let t = last_node.unwrap();
    let s = penult_node.unwrap_or(network.get(&t.edges[0].0).unwrap());

    let cut_size = t.edges.iter().map(|edge| edge.1).sum::<usize>();
    let t_labels = t.labels.clone();
    let s_labels = s.labels.clone();
    let mut side_two = network
        .iter()
        .filter(|&(k, _)| k != &t_labels)
        .flat_map(|(_, node)| node.labels.clone())
        .collect::<Vec<&str>>();
    side_two.sort();

    let combined_edges = s.edges.iter().chain(t.edges.iter())
        .filter(|edge| edge.0 != s_labels && edge.0 != t_labels)
        .fold(HashMap::<Label, usize>::new(), |mut acc, edge| {
            acc.entry(edge.0.clone()).and_modify(|size| *size += edge.1).or_insert(edge.1);
            acc
        })
        .into_iter()
        .collect::<Vec<(Label, usize)>>();
    let mut combined_labels = s.labels.clone();
    combined_labels.extend(t.labels.clone());
    combined_labels.sort();

    let combined_node = Node {
        labels: combined_labels.clone(),
        edges: combined_edges
    };

    network.remove(&s_labels);
    network.remove(&t_labels);
    network.insert(combined_labels.clone(), combined_node);

    for node in network.values_mut() {
        let st_edge_count = node.edges
            .iter()
            .filter(|edge| edge.0 == s_labels || edge.0 == t_labels)
            .map(|edge| edge.1)
            .sum::<usize>();
        if st_edge_count > 0 {
            node.edges.retain(|edge| edge.0 != s_labels && edge.0 != t_labels);
            node.edges.push((combined_labels.clone(), st_edge_count));
        }
    }

    Cut{
        side_one: t_labels.iter().map(|s| s.to_string()).collect(),
        side_two: side_two.iter().map(|s| s.to_string()).collect(),
        size: cut_size
    }
}
