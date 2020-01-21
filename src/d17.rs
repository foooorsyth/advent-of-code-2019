use crate::intcode::IntCodeCPU;
use crate::shared::{atoi, in_bounds, itoa, Point};
use std::cmp::{min, Ordering};
use std::collections::{BinaryHeap, HashMap, HashSet};

pub fn part1() -> std::io::Result<i64> {
    let mut cpu = IntCodeCPU::new();
    cpu.read_data_file("input/d17.txt")?;
    cpu.execute();
    let (img, w, h) = clean_img(&cpu);
    let intersecs = intersections(&img, w, h);
    let res: i64 = intersecs
        .iter()
        .fold(0, |acc, x| acc + (x.x as i64) * (x.y as i64));
    return Ok(res);
}

pub fn part2() -> std::io::Result<i64> {
    let mut cpu = IntCodeCPU::new();
    cpu.read_data_file("input/d17.txt")?;
    cpu.set_mem_at(0, 2);
    cpu.execute();
    let (img, w, h) = clean_img(&cpu);
    cpu.output.clear();
    let full_sequence = walk(&img, w, h);
    let sorted_substrs = longest_repeated(&full_sequence, 3, 12);
    let (parts, current_sequence) = compress(&full_sequence, &sorted_substrs, 0, 0, 0);
    let condensed = condense(&current_sequence, &parts);
    let main_routine = asciify(&condensed);
    let a = asciify(&parts[0]);
    let b = asciify(&parts[1]);
    let c = asciify(&parts[2]);
    for input in main_routine {
        cpu.enqueue_input(input);
    }
    cpu.execute();
    for input in a {
        cpu.enqueue_input(input);
    }
    cpu.execute();
    for input in b {
        cpu.enqueue_input(input);
    }
    cpu.execute();
    for input in c {
        cpu.enqueue_input(input);
    }
    cpu.execute();
    cpu.enqueue_input(atoi('n'));
    cpu.enqueue_input(10);
    cpu.execute();
    Ok(cpu.last_output.unwrap())
}

// condenses AAABBBCCC to ABC, given lengths of A, B, and C
fn condense(sequence: &String, parts: &Vec<String>) -> String {
    let mut res = "".to_owned();
    let seq_vec: Vec<char> = sequence.chars().collect();
    let mut i = 0;
    while i < seq_vec.len() {
        let c = seq_vec[i];
        let part_idx = atoi(c) - 65;
        let part = &parts[part_idx as usize];
        res.push(c);
        i += part.len();
    }
    res
}

//dynamic programming LZ-like compression
fn compress(
    sequence: &String,
    sorted_substrs: &Vec<Substring>,
    a: usize,
    b: usize,
    c: usize,
) -> (Vec<String>, String) {
    let mut current_sequence = sequence.clone();
    let max_parts = 3;
    let part_names = ['A', 'B', 'C'];
    let mut parts: Vec<String> = Vec::with_capacity(max_parts);
    let mut start_outer = 0;
    let mut part = 0;
    'outer_loop: while start_outer < current_sequence.len() {
        let mut found_one = false;
        let range: std::ops::Range<usize>;
        match part {
            0 => {
                range = a..sorted_substrs.len();
            }
            1 => {
                range = b..sorted_substrs.len();
            }
            2 => {
                range = c..sorted_substrs.len();
            }
            _ => panic!("wtf"),
        }
        'substr_loop: for substr_idx in range {
            let substr = &sorted_substrs[substr_idx];
            if start_outer + substr.len >= current_sequence.len()
                || current_sequence[start_outer..(start_outer + substr.len)] != substr.val
                || asciify(&substr.val.clone()).len() > 21
            {
                continue;
            }
            found_one = true;
            for char_idx in start_outer..(start_outer + substr.len) {
                let mut bytes = current_sequence.into_bytes();
                bytes[char_idx] = part_names[part] as u8;
                unsafe { current_sequence = String::from_utf8_unchecked(bytes) }
            }
            let mut start_inner = start_outer + substr.len;
            start_outer += substr.len;
            let mut chain = true;
            while start_inner <= current_sequence.len() - substr.len {
                if current_sequence[start_inner..(start_inner + substr.len)] == substr.val {
                    for char_idx in start_inner..(start_inner + substr.len) {
                        let mut bytes = current_sequence.into_bytes();
                        bytes[char_idx] = part_names[part] as u8;
                        unsafe { current_sequence = String::from_utf8_unchecked(bytes) }
                    }
                    start_inner += substr.len;
                    if chain {
                        start_outer += substr.len;
                    }
                } else {
                    chain = false;
                    start_inner += 1;
                }
            }
            parts.push(substr.val.clone());
            part += 1;
            if part == max_parts {
                break 'outer_loop;
            }
            break 'substr_loop;
        }
        if !found_one {
            start_outer += 1
        }
    }
    for ch in current_sequence.chars() {
        if !(ch == 'A' || ch == 'B' || ch == 'C') {
            if a < sorted_substrs.len() - 1 {
                return compress(sequence, sorted_substrs, a + 1, b, c);
            } else if b < sorted_substrs.len() - 1 {
                return compress(sequence, sorted_substrs, a, b + 1, c);
            } else if c < sorted_substrs.len() - 1 {
                return compress(sequence, sorted_substrs, a, b, c + 1);
            } else {
                panic!("couldn't do it");
            }
        }
    }
    return (parts, current_sequence);
}

fn asciify(sequence: &String) -> Vec<i64> {
    let seq_vec: Vec<char> = sequence.chars().collect();
    let mut res = Vec::<i64>::new();
    let mut two_dig = false;
    for i in 0..seq_vec.len() {
        let c = seq_vec[i];
        let i_c = atoi(c);
        if two_dig {
            res.push(i_c);
            res.push(atoi(','));
            two_dig = false;
            continue;
        }
        if i < seq_vec.len() - 1 && !is_letter(i_c) {
            let next = seq_vec[i + 1];
            let i_next = atoi(next);
            if !is_letter(i_next) {
                // next is a number, so this is a two digit number
                two_dig = true;
            }
        }
        res.push(i_c);
        if !two_dig && i != seq_vec.len() - 1 {
            res.push(atoi(','));
        }
    }
    res.push(10);
    res
}

fn is_letter(val: i64) -> bool {
    match val {
        76 | 82 | 65 | 66 | 67 => true,
        _ => false,
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Substring {
    val: String,
    start: usize,
    len: usize,
}

impl Ord for Substring {
    fn cmp(&self, other: &Substring) -> Ordering {
        self.len.cmp(&other.len)
    }
}

impl PartialOrd for Substring {
    fn partial_cmp(&self, other: &Substring) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn longest_repeated(input: &String, min_len: usize, max_len: usize) -> Vec<Substring> {
    let mut max_q: BinaryHeap<Substring> = BinaryHeap::new();
    let mut all: HashMap<String, Substring> = HashMap::new();
    let mut dupes: HashSet<String> = HashSet::new();
    for start in 0..(input.len() - min_len) {
        for end in (start + min_len)..min(input.len() - min_len, start + max_len) {
            let sub = input[start..end].to_owned().clone();
            let contains = all.contains_key(&sub);
            let start_end_conditions = (sub.starts_with("L") || sub.starts_with("R"))
                && !sub.ends_with("L")
                && !sub.ends_with("R");
            if !contains && start_end_conditions {
                all.insert(
                    sub.clone(),
                    Substring {
                        val: sub.clone(),
                        start,
                        len: end - start,
                    },
                );
            } else if contains && start_end_conditions && !dupes.contains(&sub) {
                dupes.insert(sub.clone());
                let cached = &all[&sub];
                max_q.push(Substring {
                    val: cached.val.clone(),
                    start: cached.start,
                    len: cached.len,
                });
            }
        }
    }
    let mut sorted_subs = Vec::<Substring>::new();
    while let Some(substr) = max_q.pop() {
        sorted_subs.push(substr);
    }
    return sorted_subs;
}

fn clean_img(cpu: &IntCodeCPU) -> (Vec<i64>, usize, usize) {
    let mut img = cpu.output.clone();
    let w: usize = img.iter().position(|&x| x == 10).unwrap();
    img.retain(|&x| x != 10);
    let h: usize = img.len() / w;
    (img, w, h)
}

fn walk(img: &Vec<i64>, w: usize, h: usize) -> String {
    let mut full_sequence = Vec::<(char, i64)>::new();
    let start_pos = img.iter().position(|x| *x == 94).unwrap();
    let y = start_pos / w;
    let x = start_pos - y * w;
    let mut pos = Point::new(x as i32, y as i32);
    let mut dir = 0;
    let r = Point {
        x: pos.x + 1,
        y: pos.y,
    };
    let d = Point {
        x: pos.x,
        y: pos.y + 1,
    };
    let l = Point {
        x: pos.x - 1,
        y: pos.y,
    };
    let u = Point {
        x: pos.x,
        y: pos.y - 1,
    };
    if in_bounds(w, h, &r) && img[w * (r.y as usize) + (r.x as usize)] == 35 {
        dir = 1;
    }
    if in_bounds(w, h, &l) && img[w * (l.y as usize) + (l.x as usize)] == 35 {
        dir = 3;
    }
    if in_bounds(w, h, &d) && img[w * (d.y as usize) + (d.x as usize)] == 35 {
        dir = 2;
    }
    if in_bounds(w, h, &u) && img[w * (u.y as usize) + (u.x as usize)] == 35 {
        dir = 0;
    }
    match dir {
        0 => {}
        1 => {
            full_sequence.push(('R', 0));
        }
        2 => {
            full_sequence.push(('L', 0));
            full_sequence.push(('L', 0));
        }
        3 => {
            full_sequence.push(('L', 0));
        }
        _ => panic!("wtf"),
    }
    while let Some(step_char) = step(&img, w, h, &mut pos, &mut dir) {
        if step_char == 'F' {
            let len = full_sequence.len();
            let last = full_sequence[len - 1];
            full_sequence[len - 1] = (last.0, last.1 + 1);
        } else {
            full_sequence.push((step_char, 0));
        }
    }
    let mut str_rep = "".to_owned();
    for entry in full_sequence {
        str_rep.push(entry.0);
        if entry.1 > 9 {
            let tens = IntCodeCPU::dig(&entry.1, &1);
            let ones = IntCodeCPU::dig(&entry.1, &0);
            str_rep.push(itoa(tens + 48));
            str_rep.push(itoa(ones + 48));
        } else {
            str_rep.push(itoa(entry.1 + 48));
        }
    }
    return str_rep;
}

fn step(img: &Vec<i64>, w: usize, h: usize, pos: &mut Point, dir: &mut i32) -> Option<char> {
    let u = Point {
        x: pos.x,
        y: pos.y - 1,
    };
    let r = Point {
        x: pos.x + 1,
        y: pos.y,
    };
    let d = Point {
        x: pos.x,
        y: pos.y + 1,
    };
    let l = Point {
        x: pos.x - 1,
        y: pos.y,
    };
    let forward: Point;
    match dir {
        // up
        0 => {
            forward = u;
        }
        // right
        1 | -3 => {
            forward = r;
        }
        // down
        2 | -2 => {
            forward = d;
        }
        // left
        3 | -1 => {
            forward = l;
        }
        _ => panic!("wtf"),
    }
    // bounds check
    if in_bounds(w, h, &forward) && img[w * (forward.y as usize) + (forward.x as usize)] == 35 {
        *pos = forward.clone();
        return Some('F');
    } else {
        match dir {
            // up, check l/r
            0 => {
                if in_bounds(w, h, &l) && img[w * (l.y as usize) + (l.x as usize)] == 35 {
                    *dir = 3;
                    return Some('L');
                }
                if in_bounds(w, h, &r) && img[w * (r.y as usize) + (r.x as usize)] == 35 {
                    *dir = 1;
                    return Some('R');
                }
                return None;
            }
            // right, check u/d
            1 => {
                if in_bounds(w, h, &u) && img[w * (u.y as usize) + (u.x as usize)] == 35 {
                    *dir = 0;
                    return Some('L');
                }
                if in_bounds(w, h, &d) && img[w * (d.y as usize) + (d.x as usize)] == 35 {
                    *dir = 2;
                    return Some('R');
                }
                return None;
            }
            2 => {
                //down, check l/r
                if in_bounds(w, h, &l) && img[w * (l.y as usize) + (l.x as usize)] == 35 {
                    *dir = 3;
                    return Some('R');
                }
                if in_bounds(w, h, &r) && img[w * (r.y as usize) + (r.x as usize)] == 35 {
                    *dir = 1;
                    return Some('L');
                }
                return None;
            }
            // left
            3 => {
                if in_bounds(w, h, &u) && img[w * (u.y as usize) + (u.x as usize)] == 35 {
                    *dir = 0;
                    return Some('R');
                }
                if in_bounds(w, h, &d) && img[w * (d.y as usize) + (d.x as usize)] == 35 {
                    *dir = 2;
                    return Some('L');
                }
                return None;
            }
            _ => panic!("wtf"),
        }
    }
}

fn intersections(img: &Vec<i64>, w: usize, h: usize) -> Vec<Point> {
    let mut res = Vec::new();
    for y in 1..(h - 1) {
        for x in 1..(w - 1) {
            if img[w * y + x] == 35
                && img[w * y + (x - 1)] == 35
                && img[w * y + (x + 1)] == 35
                && img[w * (y - 1) + x] == 35
                && img[w * (y + 1) + x] == 35
            {
                res.push(Point::new(x as i32, y as i32))
            }
        }
    }
    res
}
