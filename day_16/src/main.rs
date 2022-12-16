use std::collections::HashSet;
use std::{collections::HashMap, fs, io, ops::RangeInclusive};

use clap::Parser;
use petgraph::algo::{all_simple_paths, bellman_ford, dijkstra};
use tunnel::OpenedValve;

use crate::tunnel::Tunnel;

use petgraph::graph::{Node, NodeIndex};
use petgraph::{graph, Directed, Graph, Undirected};

mod tunnel;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "./challenges/challenge_16.txt")]
    path: String,
}

fn main() -> io::Result<()> {
    challenge_1_2().unwrap();
    Ok(())
}

fn challenge_1_2() -> io::Result<()> {
    let args = Args::parse();
    // File hosts must exist in current path before this produces output
    let input_str = fs::read_to_string(args.path)?.replace("\r\n", "\n");
    let mut tunnels: HashMap<String, Tunnel> = HashMap::new();
    let mut minutes_remaining = 20;
    let mut graph = Graph::<Tunnel, f32, Directed>::new();

    for c_line in input_str.split("\n") {
        let replaced_line = c_line
            .replace("Valve ", "")
            .replace(" has flow rate=", ";")
            .replace("tunnels", "tunnel")
            .replace("leads", "lead")
            .replace("valves", "valve")
            .replace("; tunnel lead to valve ", ";")
            .replace(", ", ",");
        let c_tunnel = Tunnel::new(&replaced_line);
        tunnels.insert(c_tunnel.name.clone(), c_tunnel);
    }

    let mut graph_indexes: HashMap<String, NodeIndex> = HashMap::new();
    // insert into graph
    for c_tunnel in tunnels.clone() {
        let index = graph.add_node(c_tunnel.1);
        graph_indexes.insert(c_tunnel.0, index);
    }

    // update edges
    for c_tunnel in tunnels {
        let c_index = graph_indexes.get(&c_tunnel.0).unwrap();
        for c_egde_name in c_tunnel.1.leads_to {
            let connect_to_index = graph_indexes.get(&c_egde_name).unwrap();
            graph.add_edge(*c_index, *connect_to_index, 1.0);
        }
    }

    /*let path = bellman_ford(&graph, *graph_indexes.get("AA").unwrap()).unwrap();
    println!("{:?}", path.distances);*/
    let start = *graph_indexes.get("AA").unwrap();
    for c_graph_index in graph_indexes.clone() {
        // PATH from AA to AA does not make much sense!
        if c_graph_index.0 == String::from("AA") {
            continue;
        }
        let ways = all_simple_paths::<Vec<_>, _>(&graph, start, c_graph_index.1, 0, Some(30))
            .collect::<Vec<_>>();
        println!("{}: {:?}", c_graph_index.0, ways);
    }

    let max_iterations = 10;
    let start_at = String::from("AA");
    let points = go_through_nodes(
        &graph,
        &graph_indexes,
        max_iterations,
        start_at.clone(),
        start_at,
        0,
        0,
        HashSet::new(),
        HashSet::new(),
    );

    //println!("{:?}", graph);

    //graph.neighbors(*graph_indexes.get("AA").unwrap());
    //part 1
    println!("Points 1:\t{}", points);

    //part 2
    println!("Points 2:\t{:?}", "points");

    Ok(())
}

fn go_through_nodes(
    graph: &Graph<Tunnel, f32, Directed>,
    graph_indexes: &HashMap<String, NodeIndex>,
    max_iterations: u32,
    current_node_name: String,
    last_node_name: String,
    current_iteration: u32,
    max_pressure: u32,
    opened_valves: HashSet<OpenedValve>, // Name, (value, opened_at)
    visited: HashSet<NodeIndex>,
) -> u32 {
    if current_iteration >= max_iterations {
        // break
        return max_pressure;
    }
    let current_index = graph_indexes.get(&current_node_name).unwrap();
    let mut max_pressure = max_pressure;
    let mut new_visited = visited.clone();
    let mut new_opened_valves = opened_valves.clone();
    let mut sortet_neighbors = graph.neighbors(*current_index).collect::<Vec<_>>();
    sortet_neighbors.sort();
    'outer: for c_neighbour_index in sortet_neighbors {
        let c_tunnel = graph.node_weight(c_neighbour_index).unwrap();
        /*if new_visited.contains(&c_neighbour_index) {
            //Don't go back the same way
            continue;
        }*/
        new_visited.insert(c_neighbour_index);
        //skipped tube
        let points = go_through_nodes(
            graph,
            graph_indexes,
            max_iterations,
            c_tunnel.name.clone(),
            current_node_name.clone(), // will be last in next iteration
            current_iteration + 1,
            max_pressure,
            new_opened_valves.clone(),
            new_visited.clone(),
        );
        max_pressure = max_pressure.max(points);
        // opened tube
        if !new_opened_valves.contains(&OpenedValve::default_values(c_neighbour_index))
            && c_tunnel.flow_rate != 0
            && current_iteration + 2 <= max_iterations
        {
            let mut contained = false;
            'innter: for c_opened_valve in new_opened_valves.clone() {
                if c_neighbour_index == c_opened_valve.index {
                    contained = true;
                    continue 'innter;
                }
            }
            if !contained {
                if !new_opened_valves.contains(&OpenedValve::default_values(c_neighbour_index)) {
                    new_opened_valves.insert(OpenedValve::new(
                        c_neighbour_index,
                        c_tunnel.flow_rate,
                        current_iteration + 1,
                    ));
                }
                let points_2 = go_through_nodes(
                    graph,
                    graph_indexes,
                    max_iterations,
                    c_tunnel.name.clone(),
                    current_node_name.clone(), // will be last in next iteration
                    current_iteration + 2,
                    max_pressure,
                    new_opened_valves.clone(),
                    new_visited.clone(),
                );

                max_pressure = max_pressure.max(points_2);
            }
        }
    }
    let mut output = 0;
    for c_valve in opened_valves.clone() {
        output += c_valve.flow_rate * (max_iterations - c_valve.opened_at);
    }
    if output == 1651 {
        println!("{:?}", opened_valves);
    }
    output = output.max(max_pressure);
    return output.max(max_pressure);
}
