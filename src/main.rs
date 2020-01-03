mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod d8;
mod intcode;

#[cfg(test)]
mod tests {
    use crate::d1;
    use crate::d2;
    use crate::d3;
    use crate::d4;
    use crate::d5;
    use crate::d6;
    use crate::d7;
    use crate::d8;

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
}

fn main() {
    let res_d1_p1 = d1::part1().unwrap();
    println!("d1_p1: {}", res_d1_p1);

    let res_d1_p2 = d1::part2().unwrap();
    println!("d1_p2: {}", res_d1_p2);

    let res_d2_p1 = d2::part1().unwrap();
    println!("d2_p1: {}", res_d2_p1);

    let res_d2_p2 = d2::part2().unwrap();
    println!("d2_p2: {}", res_d2_p2);

    let res_d3_p1 = d3::part1().unwrap();
    println!("d3_p1: {}", res_d3_p1);

    let res_d3_p2 = d3::part2().unwrap();
    println!("d3_p1: {}", res_d3_p2);

    let res_d4_p1 = d4::part1();
    println!("d4_p1: {}", res_d4_p1);

    let res_d4_p2 = d4::part2();
    println!("d4_p2: {}", res_d4_p2);

    let res_d5_p1 = d5::part1().unwrap();
    println!("d5_p1: {}", res_d5_p1);

    let res_d5_p2 = d5::part2().unwrap();
    println!("d5_p2: {}", res_d5_p2);

    let res_d6_p1 = d6::part1().unwrap();
    println!("d6_p1: {}", res_d6_p1);

    let res_d6_p2 = d6::part2().unwrap();
    println!("d6_p2: {}", res_d6_p2);

    let res_d7_p1 = d7::part1().unwrap();
    println!("d7_p1: {}", res_d7_p1);

    let res_d7_p2 = d7::part2().unwrap();
    println!("d7_p2: {}", res_d7_p2);

    let res_d8_p1 = d8::part1().unwrap();
    println!("d8_p1: {}", res_d8_p1);

    println!("d8_p2:");
    d8::part2().unwrap();
}
