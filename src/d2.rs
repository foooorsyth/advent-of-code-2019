use crate::intcode;
use std::collections::HashMap;

const TARGET_PART2: i32 = 19690720;

pub fn part1() -> std::io::Result<i32> {
    let mut ow = HashMap::new();
    ow.insert(1, 12);
    ow.insert(2, 2);
    return intcode::execute_with_overwrite("input/d2.txt", (true, 0), ow);
}

pub fn part2() -> std::io::Result<i32> {
    // brute force
    for n in 0..100 {
        for v in 0..100 {
            let mut ow = HashMap::new();
            ow.insert(1, n);
            ow.insert(2, v);
            if intcode::execute_with_overwrite("input/d2.txt", (true, 0), ow).unwrap()
                == TARGET_PART2
            {
                return Ok(100 * n + v);
            }
        }
    }
    panic!("wtf")
}
