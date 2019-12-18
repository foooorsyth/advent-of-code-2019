mod d1_1;
mod d1_2;

fn main() {
    let res_d1_1 = d1_1::solve().unwrap();
    println!("d1_1: {}", res_d1_1);

    let res_d1_2 = d1_2::solve().unwrap();
    println!("d1_2: {}", res_d1_2)
}
