mod d1;
mod d2;
mod d3;

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
}
