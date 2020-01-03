use crate::intcode::IntCodeCPU;
use std::collections::HashSet;

const LOW: i32 = 256310;
const HI: i32 = 732736;

pub fn part1() -> i32 {
    return common(false);
}

pub fn part2() -> i32 {
    return common(true);
}

fn common(p2_criteria: bool) -> i32 {
    let (true_low, true_hi) = compact_range(&LOW, &HI);
    let mut count = 0;
    let mut cache: HashSet<i32> = HashSet::new();
    let p5_s = IntCodeCPU::dig(&(true_low as i64), &5) as i32;
    for gap in 0..=4 {
        for i in p5_s..=9 {
            for j in i..=9 {
                for k in j..=9 {
                    for l in k..=9 {
                        for m in l..=9 {
                            let perm = construct(&gap, &i, &j, &k, &l, &m);
                            if perm >= true_low && perm <= true_hi && !cache.contains(&perm) {
                                if p2_criteria {
                                    if check_p2_criteria(&perm, &gap) {
                                        cache.insert(perm);
                                        count += 1;
                                    }
                                } else {
                                    cache.insert(perm);
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return count;
}

fn check_p2_criteria(perm: &i32, gap: &i32) -> bool {
    let target = IntCodeCPU::dig(&(*perm as i64), &gap);
    if *gap - 1 >= 0 && IntCodeCPU::dig(&(*perm as i64), &(*gap - 1)) == target {
        return false;
    }

    if *gap + 2 <= 5 && IntCodeCPU::dig(&(*perm as i64), &(*gap + 2)) == target {
        return false;
    }

    return true;
}

fn construct(gap: &i32, i: &i32, j: &i32, k: &i32, l: &i32, m: &i32) -> i32 {
    match gap {
        4 => {
            return i * 10i32.pow(5)
                + i * 10i32.pow(4)
                + j * 10i32.pow(3)
                + k * 10i32.pow(2)
                + l * 10i32.pow(1)
                + m
        }
        3 => {
            return i * 10i32.pow(5)
                + j * 10i32.pow(4)
                + j * 10i32.pow(3)
                + k * 10i32.pow(2)
                + l * 10i32.pow(1)
                + m
        }
        2 => {
            return i * 10i32.pow(5)
                + j * 10i32.pow(4)
                + k * 10i32.pow(3)
                + k * 10i32.pow(2)
                + l * 10i32.pow(1)
                + m
        }
        1 => {
            return i * 10i32.pow(5)
                + j * 10i32.pow(4)
                + k * 10i32.pow(3)
                + l * 10i32.pow(2)
                + l * 10i32.pow(1)
                + m
        }
        0 => {
            return i * 10i32.pow(5)
                + j * 10i32.pow(4)
                + k * 10i32.pow(3)
                + l * 10i32.pow(2)
                + m * 10i32.pow(1)
                + m
        }
        &_ => panic!("wtf"),
    }
}

fn ex_sum(val: &i32, pwr: &i32) -> i32 {
    return val - (IntCodeCPU::dig(&(*val as i64), pwr) as i32) * 10i32.pow(*pwr as u32);
}

pub fn compact_range(low: &i32, hi: &i32) -> (i32, i32) {
    let mut low_dig_min = 0;
    let mut low_new = *low;
    let mut low_bt = false;

    let mut hi_dig_min = 0;
    let mut hi_new = *hi;
    let mut hi_bt = false;

    for pwr in (0..=5).rev() {
        compact_range_aux(&mut low_new, &pwr, &mut low_dig_min, &mut low_bt);
        compact_range_aux(&mut hi_new, &pwr, &mut hi_dig_min, &mut hi_bt);
    }
    return (low_new, hi_new);
}

fn compact_range_aux(adj_val: &mut i32, pwr: &i32, dig_min: &mut i32, back_tracked: &mut bool) {
    let dig_curr = IntCodeCPU::dig(&(*adj_val as i64), pwr) as i32;
    if dig_curr < *dig_min {
        let mut tmp = *dig_min * 10i32.pow(*pwr as u32) + ex_sum(adj_val, &pwr);
        while tmp > HI {
            *back_tracked = true;
            let tmp_dig = IntCodeCPU::dig(&(*adj_val as i64), &(*pwr + 1)) as i32;
            tmp = (tmp_dig - 1) * 10i32.pow((*pwr + 1) as u32) + ex_sum(adj_val, &(*pwr + 1));
            if tmp_dig == *dig_min {
                *dig_min -= 1;
            }
            tmp = 9 * 10i32.pow(*pwr as u32) + ex_sum(&tmp, &pwr);
            *dig_min = 9;
        }
        *adj_val = tmp;
    } else {
        if !*back_tracked {
            *dig_min = dig_curr;
        } else {
            *adj_val = 9 * 10i32.pow(*pwr as u32) + ex_sum(adj_val, &pwr);
        }
    }
}
