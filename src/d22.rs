use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::FromPrimitive;
use std::fs::File;
use std::io::Result;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Mul, Sub};

pub struct Deck {
    size: BigInt,
    slope: BigInt,
    intercept: BigInt,
    repetitions: BigInt,
}

impl Deck {
    pub fn new(size: i128) -> Deck {
        return Deck {
            size: BigInt::from(size),
            slope: BigInt::from(1),
            intercept: BigInt::from(0),
            repetitions: BigInt::from(1),
        };
    }

    fn chain(&mut self, slope: i128, intercept: i128) {
        self.slope = self.slope.clone().mul(slope).mod_floor(&self.size);
        self.intercept = (self.intercept.clone().mul(slope).add(intercept)).mod_floor(&self.size);
    }

    pub fn deal_with_increment(&mut self, n: i128) {
        self.chain(n, 0);
    }

    pub fn deal_into_new_stack(&mut self) {
        self.chain(-1, -1);
    }

    pub fn cut(&mut self, n: i128) {
        self.chain(1, -n);
    }

    pub fn position_of(&self, card: i128) -> String {
        let (slp, inter) = self.coeffs();
        let idx = (slp.mul(card).add(inter)).mod_floor(&self.size);
        idx.to_string()
    }

    pub fn get(&self, index: i128) -> String {
        let (slp, inter) = self.coeffs();
        let res = mod_div(&BigInt::from(index).sub(inter), &slp, &self.size);
        res.to_string()
    }

    pub fn coeffs(&self) -> (BigInt, BigInt) {
        let slp = self.slope.modpow(&self.repetitions, &self.size);
        let inter = mod_div(
            &self.intercept.clone().mul(BigInt::from(1).sub(&slp)),
            &BigInt::from(1).sub(&self.slope),
            &self.size,
        );
        (slp, inter)
    }
}

pub fn part1() -> Result<String> {
    let mut deck = Deck::new(10007);
    execute(&mut deck, &read("input/d22.txt").unwrap());
    Ok(deck.position_of(2019))
}

pub fn part2() -> Result<String> {
    let mut deck = Deck::new(119315717514047);
    execute(&mut deck, &read("input/d22.txt").unwrap());
    deck.repetitions = BigInt::from_i128(101741582076661).unwrap();
    Ok(deck.get(2020))
}

fn mod_div(numer: &BigInt, denom: &BigInt, m: &BigInt) -> BigInt {
    let sub_2: BigInt = m.sub(2);
    (numer.mul(&denom.modpow(&sub_2, m))).mod_floor(m)
}

fn execute(deck: &mut Deck, instructions: &Vec<String>) {
    instructions.iter().for_each(|instruction| {
        if instruction.starts_with("cut ") {
            let (_, num_str) = instruction.split_at(3);
            let num = num_str.trim().parse::<i128>().unwrap();
            deck.cut(num);
        } else if instruction.starts_with("deal with increment ") {
            let (_, num_str) = instruction.split_at("deal with increment ".len() - 1);
            let num = num_str.trim().parse::<i128>().unwrap();
            deck.deal_with_increment(num);
        } else if instruction.starts_with("deal into new stack") {
            deck.deal_into_new_stack();
        }
    });
}

fn read(input: &'static str) -> Result<Vec<String>> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);
    let mut res = Vec::new();
    reader
        .lines()
        .for_each(|line| res.push(line.unwrap().to_string()));
    Ok(res)
}
