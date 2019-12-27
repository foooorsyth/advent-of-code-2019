use crate::intcode;

const TARGET_PART2: i32 = 19690720;

pub fn part1() -> std::io::Result<i32> {
    return intcode::execute_program("input/d2.txt", &12, &2)
}

pub fn part2() -> std::io::Result<i32> {
    // brute force
    for n in 0..100 {
        for v in 0..100 {
            if intcode::execute_program("input/d2.txt", &n, &v).unwrap() == TARGET_PART2 {
                return Ok(100 * n + v)
            }
        }
    }
    panic!("wtf")
}
