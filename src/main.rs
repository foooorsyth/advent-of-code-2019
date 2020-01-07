mod d1;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod d8;
mod d9;
mod intcode;
mod shared;

use std::env;

#[cfg(test)]
mod tests {
    use crate::d1;
    use crate::d10;
    use crate::d11;
    use crate::d12;
    use crate::d13;
    use crate::d14;
    use crate::d2;
    use crate::d3;
    use crate::d4;
    use crate::d5;
    use crate::d6;
    use crate::d7;
    use crate::d8;
    use crate::d9;

    #[test]
    fn test_d1_p1() {
        assert_eq!(d1::part1().unwrap(), 3382136);
    }

    #[test]
    fn test_d1_p2() {
        assert_eq!(d1::part2().unwrap(), 5070314);
    }

    #[test]
    fn test_d2_p1() {
        assert_eq!(d2::part1().unwrap(), 4462686);
    }

    #[test]
    fn test_d2_p2() {
        assert_eq!(d2::part2().unwrap(), 5936);
    }

    #[test]
    fn test_d3_p1() {
        assert_eq!(d3::part1().unwrap(), 1211);
    }

    #[test]
    fn test_d3_p2() {
        assert_eq!(d3::part2().unwrap(), 101386);
    }

    #[test]
    fn test_d4_p1() {
        assert_eq!(d4::part1(), 979);
    }

    #[test]
    fn test_d4_p2() {
        assert_eq!(d4::part2(), 635);
    }

    #[test]
    fn test_d5_p1() {
        assert_eq!(d5::part1().unwrap(), 9025675);
    }

    #[test]
    fn test_d5_p2() {
        assert_eq!(d5::part2().unwrap(), 11981754);
    }

    #[test]
    fn test_d6_p1() {
        assert_eq!(d6::part1().unwrap(), 145250);
    }

    #[test]
    fn test_d6_p2() {
        assert_eq!(d6::part2().unwrap(), 274);
    }

    #[test]
    fn test_d7_p1() {
        assert_eq!(d7::part1().unwrap(), 262086);
    }

    #[test]
    fn test_d7_p2() {
        assert_eq!(d7::part2().unwrap(), 5371621);
    }

    #[test]
    fn test_d8_p1() {
        assert_eq!(d8::part1().unwrap(), 1806);
    }

    #[test]
    fn test_d9_p1() {
        assert_eq!(d9::part1().unwrap(), 4261108180);
    }

    #[test]
    fn test_d9_p2() {
        assert_eq!(d9::part2().unwrap(), 77944);
    }

    #[test]
    fn test_d10_p1() {
        assert_eq!(d10::part1().unwrap(), 269);
    }

    #[test]
    fn test_d10_p2() {
        assert_eq!(d10::part2().unwrap(), 612);
    }

    #[test]
    fn test_d11_p1() {
        assert_eq!(d11::part1().unwrap(), 2021);
    }

    #[test]
    fn test_d12_p1() {
        assert_eq!(d12::part1().unwrap(), 12644);
    }

    #[test]
    fn test_d12_p2() {
        assert_eq!(d12::part2().unwrap(), 290314621566528);
    }

    #[test]
    fn test_d13_p1() {
        assert_eq!(d13::part1().unwrap(), 270);
    }

    #[test]
    fn test_d14_p1() {
        assert_eq!(d14::part1().unwrap(), 899155);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Provide day. eg. cargo run -- 12");
        return;
    }
    match args[1].parse::<i32>().unwrap() {
        1 => {
            let res_d1_p1 = d1::part1().unwrap();
            println!("d1_p1: {}", res_d1_p1);
            let res_d1_p2 = d1::part2().unwrap();
            println!("d1_p2: {}", res_d1_p2);
        }
        2 => {
            let res_d2_p1 = d2::part1().unwrap();
            println!("d2_p1: {}", res_d2_p1);
            let res_d2_p2 = d2::part2().unwrap();
            println!("d2_p2: {}", res_d2_p2);
        }
        3 => {
            let res_d3_p1 = d3::part1().unwrap();
            println!("d3_p1: {}", res_d3_p1);
            let res_d3_p2 = d3::part2().unwrap();
            println!("d3_p1: {}", res_d3_p2);
        }
        4 => {
            let res_d4_p1 = d4::part1();
            println!("d4_p1: {}", res_d4_p1);
            let res_d4_p2 = d4::part2();
            println!("d4_p2: {}", res_d4_p2);
        }
        5 => {
            let res_d5_p1 = d5::part1().unwrap();
            println!("d5_p1: {}", res_d5_p1);
            let res_d5_p2 = d5::part2().unwrap();
            println!("d5_p2: {}", res_d5_p2);
        }
        6 => {
            let res_d6_p1 = d6::part1().unwrap();
            println!("d6_p1: {}", res_d6_p1);
            let res_d6_p2 = d6::part2().unwrap();
            println!("d6_p2: {}", res_d6_p2);
        }
        7 => {
            let res_d7_p1 = d7::part1().unwrap();
            println!("d7_p1: {}", res_d7_p1);
            let res_d7_p2 = d7::part2().unwrap();
            println!("d7_p2: {}", res_d7_p2);
        }
        8 => {
            let res_d8_p1 = d8::part1().unwrap();
            println!("d8_p1: {}", res_d8_p1);
            println!("d8_p2:");
            d8::part2().unwrap();
        }
        9 => {
            let res_d9_p1 = d9::part1().unwrap();
            println!("d9_p1: {}", res_d9_p1);
            let res_d9_p2 = d9::part2().unwrap();
            println!("d9_p2: {}", res_d9_p2);
        }
        10 => {
            let res_d10_p1 = d10::part1().unwrap();
            println!("d10_p1: {}", res_d10_p1);
            let res_d10_p2 = d10::part2().unwrap();
            println!("d10_p2: {}", res_d10_p2);
        }
        11 => {
            let res_d11_p1 = d11::part1().unwrap();
            println!("d11_p1: {}", res_d11_p1);
            println!("d11_p2:");
            d11::part2().unwrap();
        }
        12 => {
            let res_d12_p1 = d12::part1().unwrap();
            println!("d12_p1: {}", res_d12_p1);
            let res_d12_p2 = d12::part2().unwrap();
            println!("d12_p2: {}", res_d12_p2);
        }
        13 => {
            let res_d13_p1 = d13::part1().unwrap();
            println!("d13_p1: {}", res_d13_p1);
            let res_d13_p2 = d13::part2().unwrap();
            println!("d13_p2: {}", res_d13_p2);
        }
        14 => {
            let res_d14_p1 = d14::part1().unwrap();
            println!("d14_p1: {}", res_d14_p1);
        }
        _ => println!("Invalid day"),
    }
}
