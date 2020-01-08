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
    let mut cpu = IntCodeCPU::new();
    cpu.read_data_file("input/d15.txt")?;
    let mut cache = HashMap::<String, Meta>::new();
    let mut q = VecDeque::<(
        Point, /*from*/
        i64,   /*depth_of_from*/
        Point, /*to*/
    )>::new();
    q.push_back((Point::origin(), -1, Point::origin()));
    cache.insert(Point::origin().to_string(), Meta::new(cpu.snapshot(), 1, 0));
    // BFS with backtracking
    loop {
        if q.len() == 0 {
            println!("Failed to find target");
            return Ok(-1);
        }
        let current_move = q.pop_front().unwrap();
        cpu.load_snapshot(cache[&current_move.2.to_string()].snapshot.snapshot());
        let depth = current_move.1 + 1;
        let before_test_snap = cpu.snapshot();
        // North
        if test_pt(
            &current_move,
            &mut q,
            &mut cache,
            &mut cpu,
            &before_test_snap,
            depth,
            1,
        ) {
            return Ok(depth + 1);
        }

        // South
        if test_pt(
            &current_move,
            &mut q,
            &mut cache,
            &mut cpu,
            &before_test_snap,
            depth,
            2,
        ) {
            return Ok(depth + 1);
        }

        // West
        if test_pt(
            &current_move,
            &mut q,
            &mut cache,
            &mut cpu,
            &before_test_snap,
            depth,
            3,
        ) {
            return Ok(depth + 1);
        }

        // East
        if test_pt(
            &current_move,
            &mut q,
            &mut cache,
            &mut cpu,
            &before_test_snap,
            depth,
            4,
        ) {
            return Ok(depth + 1);
        }
    }
}

fn test_pt(
    current_move: &(Point, i64, Point),
    q: &mut VecDeque<(Point, i64, Point)>,
    cache: &mut HashMap<String, Meta>,
    cpu: &mut IntCodeCPU,
    before_test_snap: &IntCodeCPU,
    depth: i64,
    direction: i64,
) -> bool {
    let test_pt: Point;
    match direction {
        1 => {
            test_pt = Point::new(current_move.2.x, current_move.2.y - 1);
        }
        2 => {
            test_pt = Point::new(current_move.2.x, current_move.2.y + 1);
        }
        3 => {
            test_pt = Point::new(current_move.2.x - 1, current_move.2.y);
        }
        4 => {
            test_pt = Point::new(current_move.2.x + 1, current_move.2.y);
        }
        _ => panic!("Illegal direction"),
    }
    if !cache.contains_key(&test_pt.to_string()) {
        cpu.enqueue_input(direction);
        cpu.execute();
        let pixel = cpu.last_output.unwrap();
        cache.insert(test_pt.to_string(), Meta::new(cpu.snapshot(), pixel, depth));
        if pixel == 1 {
            // space
            q.push_front((current_move.2.clone(), current_move.1 + 1, test_pt.clone()));
        } else if pixel == 2 {
            // oxygen sys
            return true;
        }
        cpu.load_snapshot(before_test_snap.snapshot());
    }
    return false;
}
