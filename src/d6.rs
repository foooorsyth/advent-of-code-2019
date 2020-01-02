use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::usize;

pub fn part1() -> std::io::Result<i32> {
    let (mut storage, lut) = build_tree("input/d6.txt")?;
    return Ok(count_orbits(&mut storage, &lut));
}

pub fn part2() -> std::io::Result<i32> {
    let (storage, lut) = build_tree("input/d6.txt")?;
    let you_node = &storage[lut[&"YOU".to_owned()]];
    let san_node = &storage[lut[&"SAN".to_owned()]];
    let sp = shortest_path(
        &storage,
        &lut,
        you_node.parent.clone(),
        san_node.parent.clone(),
    );
    return Ok(sp.unwrap() as i32);
}

#[allow(dead_code)]
struct UTreeNode {
    parent: String,
    children: Vec<String>,
    value: String,
    visited: bool,
}

impl UTreeNode {
    pub fn new(parent: String, val: String) -> UTreeNode {
        return UTreeNode {
            parent: parent,
            value: val,
            children: Vec::new(),
            visited: false,
        };
    }
}

fn shortest_path(
    storage: &Vec<UTreeNode>,
    lut: &HashMap<String, usize>,
    start: String,
    end: String,
) -> Option<usize> {
    // Dijkstra
    let mut dist: Vec<usize> = (0..storage.len()).map(|_| usize::MAX).collect();
    let mut heap = BinaryHeap::<String>::new();
    dist[lut[&start]] = 0;
    heap.push(start.clone());
    while let Some(node_id) = heap.pop() {
        let idx = lut[&node_id];
        let cost = dist[idx];
        let node = &storage[idx];
        if node_id == end {
            return Some(cost);
        }
        if cost > dist[lut[&node_id]] {
            continue;
        }

        if node.parent != "" {
            check_dist(lut, &mut dist, &mut heap, &node.parent, cost);
        }

        for child_id in &node.children {
            check_dist(lut, &mut dist, &mut heap, child_id, cost);
        }
    }
    return None;
}

fn check_dist(
    lut: &HashMap<String, usize>,
    dist: &mut Vec<usize>,
    heap: &mut BinaryHeap<String>,
    id: &String,
    cost: usize,
) {
    let child_cost = cost + 1;
    let child_idx = lut[&id.clone()];
    if child_cost < dist[child_idx] {
        heap.push(id.clone());
        dist[child_idx] = child_cost;
    }
}

fn count_orbits(storage: &mut Vec<UTreeNode>, lut: &HashMap<String, usize>) -> i32 {
    // iterative DFS with depth summation
    let mut count: i32 = 0;
    let mut stack: Vec<(String, i32)> = Vec::new();
    stack.push(("COM".to_owned(), 0));
    while stack.len() != 0 {
        let (node_id, depth) = &stack.pop().unwrap();
        let mut node = &mut storage.get_mut(lut[node_id]).unwrap();
        if !node.visited {
            node.visited = true;
            count += depth;
            for child_id in &node.children {
                stack.push((child_id.to_string(), depth + 1));
            }
        }
    }
    return count;
}

fn build_tree(
    data_file: &'static str,
) -> std::io::Result<(Vec<UTreeNode>, HashMap<String, usize>)> {
    let file = File::open(data_file)?;
    let reader = BufReader::new(file);
    let mut storage: Vec<UTreeNode> = Vec::new();
    let mut lut: HashMap<String, usize> = HashMap::new();
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap().to_string();
        let orbit: Vec<String> = line.split(")").map(|s| s.to_string()).collect();
        let orbited_key = orbit[0].to_string();
        let orbiter_key = orbit[1].to_string();
        {
            // fighting the BC
            if lut.contains_key(&orbited_key) {
                let orbited_node_idx = lut.get(&orbited_key).unwrap();
                storage[*orbited_node_idx]
                    .children
                    .push(orbiter_key.clone());
            } else {
                let mut orbited_node = UTreeNode::new("".to_owned(), orbited_key.clone());
                orbited_node.children.push(orbiter_key.clone());
                storage.push(orbited_node);
                let orbited_node_idx = storage.len() - 1;
                lut.insert(orbited_key.clone(), orbited_node_idx);
            }
        }
        if lut.contains_key(&orbiter_key) {
            let orbiter_node_idx = lut.get(&orbiter_key).unwrap();
            storage[*orbiter_node_idx].parent = orbited_key.clone();
        } else {
            let orbiter_node = UTreeNode::new(orbited_key.clone(), orbiter_key.clone());
            storage.push(orbiter_node);
            let orbiter_node_idx = storage.len() - 1;
            lut.insert(orbiter_key.clone(), orbiter_node_idx);
        }
    }
    return Ok((storage, lut));
}
