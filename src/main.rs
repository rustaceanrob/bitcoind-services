use bitcoin::p2p::ServiceFlags;
use serde::Deserialize;
use std::{collections::BTreeMap, fs::File, io::Read};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Node {
    time: u64,
    services: u64,
    address: String,
    port: u16,
    network: String,
}

fn parse_nodes(json_str: &str) -> Result<Vec<Node>, serde_json::Error> {
    serde_json::from_str(json_str)
}

/// Groups nodes by (network, services) and returns counts
fn group_nodes(
    nodes: &[Node],
    only_compact_filters: bool,
) -> BTreeMap<(String, ServiceFlags), usize> {
    let mut groups: BTreeMap<(String, ServiceFlags), usize> = BTreeMap::new();
    for node in nodes {
        let services = ServiceFlags::from(node.services);
        if only_compact_filters && !services.has(ServiceFlags::COMPACT_FILTERS) {
            continue;
        }
        *groups.entry((node.network.clone(), services)).or_insert(0) += 1;
    }
    groups
}

/// Renders a horizontal bar chart in the terminal
fn print_bar_chart(groups: &BTreeMap<(String, ServiceFlags), usize>) {
    let max_count = groups.values().copied().max().unwrap_or(1);
    let max_bar_width = 10;

    // Find longest label for alignment
    let labels: Vec<String> = groups
        .keys()
        .map(|(net, svc)| format!("{} / flags={}", net, svc))
        .collect();
    let max_label_len = labels
        .iter()
        .map(|l| l.replace("ServiceFlags", "").len())
        .max()
        .unwrap_or(0);

    println!();
    println!(
        "{:<width$} {:<5}  Bar",
        "Group",
        "Count",
        width = max_label_len
    );
    println!("{}", "-".repeat(max_label_len + 5 + max_bar_width + 5));

    for ((net, svc), count) in groups {
        let label = format!(
            "{} / flags={}",
            net,
            svc.to_string().replace("ServiceFlags", "")
        );
        let bar_len = (*count as f64 / max_count as f64 * max_bar_width as f64).ceil() as usize;
        let bar = "█".repeat(bar_len);

        println!(
            "{:<width$} {:<5}  {} ",
            label,
            count,
            bar,
            width = max_label_len
        );
    }
    println!();
}

fn main() {
    let mut compact_filters_only = false;
    let mut args = std::env::args();
    args.next().unwrap();
    if let Some(_arg) = args.next() {
        compact_filters_only = true;
    }
    let mut nodes = File::open("nodes.json").expect("Have you ran `parse.sh`?");
    let mut input = String::new();
    nodes.read_to_string(&mut input).unwrap();
    match parse_nodes(input.as_str()) {
        Ok(nodes) => {
            println!("Parsed {} nodes", nodes.len());
            let groups = group_nodes(&nodes, compact_filters_only);
            print_bar_chart(&groups);
        }
        Err(e) => eprintln!("Failed to parse: {}", e),
    }
}
