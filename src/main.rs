mod d1;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;
mod d19;
mod d2;
mod d20;
mod d21;
mod d22;
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
    use crate::d15;
    use crate::d16;
    use crate::d17;
    use crate::d18;
    use crate::d19;
    use crate::d2;
    use crate::d20;
    use crate::d21;
    use crate::d22;
    use crate::d3;
    use crate::d4;
    use crate::d5;
    use crate::d6;
    use crate::d7;
    use crate::d8;
    use crate::d9;
    use crate::shared::atoi;

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
    fn test_d8_p2() {
        let jafra = "  ##  ##  #### ###   ##     # #  # #    #  # #  #    # #  # ###  #  # #  #    # #### #    ###  #### #  # #  # #    # #  #  #  ##  #  # #    #  # #  # ";
        assert_eq!(d8::part2(false).unwrap(), jafra);
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
    fn test_d11_p2() {
        let lbjheklh = " #    ###    ## #  # #### #  # #    #  #    #    #  #    # #  # #    # #  #    #  #    #    ###     # #### ###  ##   #    ####    #    #  #    # #  # #    # #  #    #  #    #    #  # #  # #  # #    # #  #    #  #    #### ###   ##  #  # #### #  # #### #  #   ";
        assert_eq!(d11::part2(false).unwrap(), lbjheklh);
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
    fn test_d13_p2() {
        assert_eq!(d13::part2(false).unwrap(), 12535);
    }

    #[test]
    fn test_d14_p1() {
        assert_eq!(d14::part1().unwrap(), 899155);
    }

    #[test]
    fn test_d14_p2() {
        assert_eq!(d14::part2().unwrap(), 2390226);
    }

    #[test]
    fn test_d15_p1() {
        assert_eq!(d15::part1().unwrap(), 330);
    }

    #[test]
    fn test_d15_p2() {
        assert_eq!(d15::part2().unwrap(), 352);
    }

    #[test]
    fn test_d16_p1() {
        assert_eq!(d16::part1().unwrap().parse::<i32>().unwrap(), 42205986);
    }

    #[test]
    fn test_d16_p2() {
        assert_eq!(d16::part2().unwrap().parse::<i32>().unwrap(), 13270205);
    }

    #[test]
    fn test_d17_p1() {
        assert_eq!(d17::part1().unwrap(), 6024);
    }

    #[test]
    fn test_d17_p2() {
        assert_eq!(d17::part2().unwrap(), 897344);
    }

    #[test]
    fn test_d18_p1_t1() {
        let (img, w, h, lut) = d18::read("input/d18_p1_t1.txt").unwrap();
        let cost_of_finding_keys = d18::find_keys(&img, w, h, &lut, atoi('a')..=atoi('b'));
        assert_eq!(cost_of_finding_keys, 8);
    }

    #[test]
    fn test_d18_p1_t2() {
        let (img, w, h, lut) = d18::read("input/d18_p1_t2.txt").unwrap();
        let cost_of_finding_keys = d18::find_keys(&img, w, h, &lut, atoi('a')..=atoi('f'));
        assert_eq!(cost_of_finding_keys, 86);
    }

    #[test]
    fn test_d18_p1_t3() {
        let (img, w, h, lut) = d18::read("input/d18_p1_t3.txt").unwrap();
        let cost_of_finding_keys = d18::find_keys(&img, w, h, &lut, atoi('a')..=atoi('g'));
        assert_eq!(cost_of_finding_keys, 132);
    }

    #[test]
    fn test_d18_p1_t4() {
        let (img, w, h, lut) = d18::read("input/d18_p1_t4.txt").unwrap();
        let cost_of_finding_keys = d18::find_keys(&img, w, h, &lut, atoi('a')..=atoi('p'));
        assert_eq!(cost_of_finding_keys, 136);
    }

    #[test]
    fn test_d18_p1_t5() {
        let (img, w, h, lut) = d18::read("input/d18_p1_t5.txt").unwrap();
        let cost_of_finding_keys = d18::find_keys(&img, w, h, &lut, atoi('a')..=atoi('i'));
        assert_eq!(cost_of_finding_keys, 81);
    }

    #[test]
    fn test_d18_p1() {
        let res_d18_p1 = d18::part1().unwrap();
        assert_eq!(res_d18_p1, 4192);
    }

    #[test]
    fn test_d18_p2() {
        let res_d18_p2 = d18::part2().unwrap();
        assert_eq!(res_d18_p2, 1790);
    }

    #[test]
    fn test_d19_p1() {
        let res_d19_p1 = d19::part1().unwrap();
        assert_eq!(res_d19_p1, 189);
    }

    #[test]
    fn test_d19_p2() {
        let res_d19_p2 = d19::part2().unwrap();
        assert_eq!(res_d19_p2, 7621042);
    }

    #[test]
    fn test_d20_p1() {
        let res_d20_p1 = d20::part1().unwrap();
        assert_eq!(res_d20_p1, 714);
    }

    #[test]
    fn test_d20_p2() {
        let res_d20_p2 = d20::part2().unwrap();
        assert_eq!(res_d20_p2, 7876);
    }

    #[test]
    fn test_d21_p1() {
        let res_d21_p1 = d21::part1().unwrap();
        assert_eq!(res_d21_p1, 19355227);
    }

    #[test]
    fn test_d21_p2() {
        let res_d21_p2 = d21::part2().unwrap();
        assert_eq!(res_d21_p2, 1143802926);
    }

    #[test]
    fn test_d22_p1() {
        let res_d22_p1 = d22::part1().unwrap();
        assert_eq!(res_d22_p1, "6696");
    }

    #[test]
    fn test_d22_p2() {
        let res_d22_p2 = d22::part2().unwrap();
        assert_eq!(res_d22_p2, "93750418158025");
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
            d8::part2(true).unwrap();
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
            d11::part2(true).unwrap();
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
            let res_d13_p2 = d13::part2(true).unwrap();
            println!("d13_p2: {}", res_d13_p2);
        }
        14 => {
            let res_d14_p1 = d14::part1().unwrap();
            println!("d14_p1: {}", res_d14_p1);
            let res_d14_p2 = d14::part2().unwrap();
            println!("d14_p2: {}", res_d14_p2);
        }
        15 => {
            let res_d15_p1 = d15::part1().unwrap();
            println!("d15_p1: {}", res_d15_p1);
            let res_d15_p2 = d15::part2().unwrap();
            println!("d15_p2: {}", res_d15_p2);
        }
        16 => {
            let res_d16_p1 = d16::part1().unwrap();
            println!("d16_p1: {}", res_d16_p1);
            let res_d16_p2 = d16::part2().unwrap();
            println!("d16_p2: {}", res_d16_p2);
        }
        17 => {
            let res_d17_p1 = d17::part1().unwrap();
            println!("d17_p1: {}", res_d17_p1);
            let res_d17_p2 = d17::part2().unwrap();
            println!("d17_p2: {}", res_d17_p2);
        }
        18 => {
            let res_d18_p1 = d18::part1().unwrap();
            println!("d18_p1: {}", res_d18_p1);
            let res_d18_p2 = d18::part2().unwrap();
            println!("d18_p2: {}", res_d18_p2);
        }
        19 => {
            let res_d19_p1 = d19::part1().unwrap();
            println!("d19_p1: {}", res_d19_p1);
            let res_d19_p2 = d19::part2().unwrap();
            println!("d19_p2: {}", res_d19_p2);
        }
        20 => {
            let res_d20_p1 = d20::part1().unwrap();
            println!("d20_p1: {}", res_d20_p1);
            let res_d20_p2 = d20::part2().unwrap();
            println!("d20_p2: {}", res_d20_p2);
        }
        21 => {
            let res_d21_p1 = d21::part1().unwrap();
            println!("d21_p1: {}", res_d21_p1);
            let res_d21_p2 = d21::part2().unwrap();
            println!("d21_p2: {}", res_d21_p2);
        }
        22 => {
            let res_d22_p1 = d22::part1().unwrap();
            println!("d22_p1: {}", res_d22_p1);
            let res_d22_p2 = d22::part2().unwrap();
            println!("d22_p2: {}", res_d22_p2);
        }
        _ => println!("Invalid day"),
    }
}
