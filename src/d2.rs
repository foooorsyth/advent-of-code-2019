use std::fs;

pub fn part1() -> std::io::Result<i32> {
    let input_string: String = fs::read_to_string("input/d2.txt")?;
    let mut input_vec: Vec<i32> = input_string.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    input_vec[1] = 12;
    input_vec[2] = 2;
    for i in (0..input_vec.len()).step_by(4) {
        if !mutate(&mut input_vec, i) {
            return Ok(input_vec[0])
        }
    }
    panic!("wtf")
}

fn mutate(data: &mut Vec<i32>, index: usize) -> bool {
    let assignment = data[index + 3] as usize;
    let lhs = data[index + 1] as usize;
    let rhs = data[index + 2] as usize;
    match data[index] {
        1 => {
            data[assignment] = data[lhs] + data[rhs];
            true
        }
        2 => {
            data[assignment] = data[lhs] * data[rhs];
            true
        }
        99 => false,
        _ => { panic!("wtf") }
    }
}