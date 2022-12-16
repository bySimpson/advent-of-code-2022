use petgraph::graph::{Node, NodeIndex};
use std::collections::HashSet;

use petgraph::stable_graph::IndexType;

#[derive(Debug, Clone, PartialEq)]
pub struct Tunnel {
    pub name: String,
    pub leads_to: HashSet<String>,
    pub flow_rate: u32,
}

impl Tunnel {
    pub fn new(input_line: &str) -> Self {
        let split = input_line.split(";").collect::<Vec<&str>>();
        let name = split[0].to_string();
        let flow_rate = split[1].parse::<u32>().unwrap();
        let mut leads_to = HashSet::new();
        for c_tunnel in split[2].split(",") {
            leads_to.insert(c_tunnel.to_string());
        }

        Self {
            name,
            leads_to,
            flow_rate,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq)]
pub struct OpenedValve {
    pub flow_rate: u32,
    pub opened_at: u32,
    pub index: NodeIndex,
}

impl PartialEq for OpenedValve {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl OpenedValve {
    pub fn default_values(index: NodeIndex) -> Self {
        Self {
            flow_rate: 0,
            opened_at: 0,
            index,
        }
    }

    pub fn new(index: NodeIndex, flow_rate: u32, opened_at: u32) -> Self {
        Self {
            flow_rate,
            opened_at,
            index,
        }
    }
}
