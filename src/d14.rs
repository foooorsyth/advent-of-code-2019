use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

pub fn part1() -> Result<i32> {
    let reactions = parse("input/d14.txt").unwrap();
    let (qtys, _) = calc_qtys(&reactions);
    Ok(qtys[&"ORE".to_owned()])
}

fn calc_qtys(
    reactions: &HashMap<String, Reaction>,
) -> (HashMap<String, i32>, HashMap<String, i32>) {
    let mut qtys = HashMap::<String, i32>::new();
    let mut surpluses = HashMap::<String, i32>::new();
    let mut q = VecDeque::<Chemical>::new();
    qtys.insert("FUEL".to_owned(), 1);
    q.push_back(Chemical::new("FUEL".to_owned(), 1));
    surpluses.insert("FUEL".to_owned(), 0);
    loop {
        if q.len() == 0 {
            return (qtys, surpluses);
        }
        let current = q.pop_front().unwrap();
        if current.key == "ORE".to_owned() {
            continue;
        }
        let rx = &reactions[&current.key.clone()];
        if !surpluses.contains_key(&current.key) {
            surpluses.insert(current.key.clone(), 0);
        }
        let mut multiple = current.qty / rx.rhs.qty;
        let remainder = current.qty % rx.rhs.qty;
        if remainder != 0 {
            if surpluses[&current.key] >= remainder {
                *surpluses.get_mut(&current.key).unwrap() -= remainder;
            } else {
                multiple += 1;
                let surplus = multiple * rx.rhs.qty - current.qty;
                *surpluses.get_mut(&current.key).unwrap() += surplus;
            }
        }
        for lhs_chem in &rx.lhs {
            let lhs_qty_needed = lhs_chem.qty * multiple;
            if qtys.contains_key(&lhs_chem.key) {
                *qtys.get_mut(&lhs_chem.key).unwrap() += lhs_qty_needed;
            } else {
                qtys.insert(lhs_chem.key.clone(), lhs_qty_needed);
            }
            if lhs_qty_needed > 0 {
                q.push_back(Chemical::new(lhs_chem.key.clone(), lhs_qty_needed));
            }
        }
    }
}

#[derive(Debug)]
struct Reaction {
    lhs: Vec<Chemical>,
    rhs: Chemical,
}

impl Reaction {
    fn empty() -> Reaction {
        Reaction {
            lhs: Vec::new(),
            rhs: Chemical::empty(),
        }
    }
}

#[derive(Debug)]
struct Chemical {
    key: String,
    qty: i32,
}

impl Chemical {
    fn new(key: String, qty: i32) -> Chemical {
        Chemical { key: key, qty: qty }
    }
    fn empty() -> Chemical {
        Chemical {
            key: "".to_owned(),
            qty: 0,
        }
    }
}

fn parse(input: &'static str) -> Result<HashMap<String, Reaction>> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);
    let mut res = HashMap::<String, Reaction>::new();
    for (_, line) in reader.lines().enumerate() {
        let mut rx = Reaction::empty();
        let mut rhs_chem = Chemical::empty();
        let line = line.unwrap().to_string();
        let lhs_rhs: Vec<String> = line.split("=").map(|s| s.to_string()).collect();
        let lhs = lhs_rhs[0].clone();
        let rhs = lhs_rhs[1].clone();
        let rhs_split: Vec<String> = rhs.split(" ").map(|s| s.to_string()).collect();
        rhs_chem.key = rhs_split[2].clone();
        rhs_chem.qty = rhs_split[1].parse::<i32>().unwrap();
        rx.rhs = rhs_chem;
        let lhs_split: Vec<String> = lhs.split(", ").map(|s| s.to_string()).collect();
        for i in 0..lhs_split.len() {
            let chem_split: Vec<String> = lhs_split[i].split(" ").map(|s| s.to_string()).collect();
            let mut lhs_chem = Chemical::empty();
            lhs_chem.key = chem_split[1].clone();
            lhs_chem.qty = chem_split[0].clone().parse::<i32>().unwrap();
            rx.lhs.push(lhs_chem);
        }
        res.insert(rx.rhs.key.clone(), rx);
    }
    return Ok(res);
}
