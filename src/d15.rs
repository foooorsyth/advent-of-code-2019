use crate::intcode::IntCodeCPU;
use crate::shared::Point;
use std::collections::{HashMap, VecDeque};

#[allow(dead_code)]
struct Meta {
    snapshot: IntCodeCPU,
    val: i64,
    depth: i64,
}

impl Meta {
    fn new(snapshot: IntCodeCPU, val: i64, depth: i64) -> Meta {
        return Meta {
            snapshot: snapshot,
            val: val,
            depth: depth,
        };
    }
}

pub fn part1() -> std::io::Result<i64> {
    let (_, depth, _) = build_map(true).unwrap();
    return Ok(depth);
}

pub fn part2() -> std::io::Result<i64> {
    let (mut map_cache, _, oxy_sys_pos) = build_map(false).unwrap();
    let mut q = VecDeque::<(i64 /*depth_of_to*/, Point /*to*/)>::new();
    q.push_back((0, oxy_sys_pos.clone()));
    let mut depth_max = 0;
    // Flood fill
    loop {
        if q.len() == 0 {
            return Ok(depth_max);
        }
        let current_move = q.pop_front().unwrap();
        if current_move.0 > depth_max {
            depth_max = current_move.0;
        }
        map_cache.get_mut(&current_move.1.to_string()).unwrap().val = 2; // mark visited
        let n_test = Point::new(current_move.1.x, current_move.1.y - 1);
        if map_cache.contains_key(&n_test.to_string()) && map_cache[&n_test.to_string()].val == 1 {
            q.push_back((current_move.0 + 1, n_test));
        }
        let s_test = Point::new(current_move.1.x, current_move.1.y + 1);
        if map_cache.contains_key(&s_test.to_string()) && map_cache[&s_test.to_string()].val == 1 {
            q.push_back((current_move.0 + 1, s_test));
        }
        let w_test = Point::new(current_move.1.x - 1, current_move.1.y);
        if map_cache.contains_key(&w_test.to_string()) && map_cache[&w_test.to_string()].val == 1 {
            q.push_back((current_move.0 + 1, w_test));
        }
        let e_test = Point::new(current_move.1.x + 1, current_move.1.y);
        if map_cache.contains_key(&e_test.to_string()) && map_cache[&e_test.to_string()].val == 1 {
            q.push_back((current_move.0 + 1, e_test));
        }
    }
}

fn build_map(greedy_exit: bool) -> std::io::Result<(HashMap<String, Meta>, i64, Point)> {
    let mut cpu = IntCodeCPU::new();
    cpu.read_data_file("input/d15.txt")?;
    let mut cache = HashMap::<String, Meta>::new();
    let mut q = VecDeque::<(i64 /*depth_of_to*/, Point /*to*/)>::new();
    q.push_back((0, Point::origin()));
    let mut oxy_sys_pos = Point::origin();
    cache.insert(Point::origin().to_string(), Meta::new(cpu.snapshot(), 1, 0));
    // BFS with backtracking
    loop {
        if q.len() == 0 {
            return Ok((cache, -1, oxy_sys_pos));
        }
        let current_move = q.pop_front().unwrap();
        cpu.load_snapshot(cache[&current_move.1.to_string()].snapshot.snapshot());
        let before_test_snap = cpu.snapshot();
        for dir in 1..=4 {
            if test_pt(
                &current_move,
                &mut q,
                &mut cache,
                &mut cpu,
                &before_test_snap,
                dir,
                greedy_exit,
                &mut oxy_sys_pos,
            ) {
                return Ok((cache, current_move.0 + 1, oxy_sys_pos));
            }
        }
    }
}

fn test_pt(
    current_move: &(i64, Point),
    q: &mut VecDeque<(i64, Point)>,
    cache: &mut HashMap<String, Meta>,
    cpu: &mut IntCodeCPU,
    before_test_snap: &IntCodeCPU,
    direction: i64,
    greedy_exit: bool,
    oxy_sys_pos: &mut Point,
) -> bool {
    let test_pt: Point;
    match direction {
        1 => {
            test_pt = Point::new(current_move.1.x, current_move.1.y - 1);
        }
        2 => {
            test_pt = Point::new(current_move.1.x, current_move.1.y + 1);
        }
        3 => {
            test_pt = Point::new(current_move.1.x - 1, current_move.1.y);
        }
        4 => {
            test_pt = Point::new(current_move.1.x + 1, current_move.1.y);
        }
        _ => panic!("Illegal direction"),
    }
    if !cache.contains_key(&test_pt.to_string()) {
        cpu.enqueue_input(direction);
        cpu.execute();
        let pixel = cpu.last_output.unwrap();
        cache.insert(
            test_pt.to_string(),
            Meta::new(cpu.snapshot(), pixel, current_move.0),
        );
        if pixel == 1 {
            // space
            q.push_front((current_move.0 + 1, test_pt.clone()));
        } else if pixel == 2 {
            // oxygen sys
            *oxy_sys_pos = test_pt.clone();
            if greedy_exit {
                return true;
            }
        }
        cpu.load_snapshot(before_test_snap.snapshot());
    }
    return false;
}
