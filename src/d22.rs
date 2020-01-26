use std::borrow::BorrowMut;
use std::fs::File;
use std::io::Result;
use std::io::{BufRead, BufReader};
use std::ops::{Index, IndexMut};

pub struct Deck {
    cards: Vec<i128>,
    size: i128,
    slope: i128,
    intercept: i128,
}

impl Deck {
    pub fn new(size: i128) -> Deck {
        let mut cards_vec: Vec<i128> = Vec::with_capacity(size as usize);
        for i in 0..size {
            cards_vec.push(i);
        }
        return Deck {
            cards: cards_vec,
            size,
            slope: 1,
            intercept: 0,
        };
    }

    fn chain(&mut self, slope: i128, intercept: i128) {
        self.slope = (self.slope * slope) % self.size;
        if self.slope < 0 {
            self.slope = self.slope + self.size;
        }
        self.intercept = (self.intercept * slope + intercept) % self.size;
        if self.intercept < 0 {
            self.intercept = self.intercept + self.size;
        }
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

    pub fn to_string(&self) -> String {
        let mut res = String::new();
        for i in 0..self.size {
            res.push_str(&self[i as i128].to_string());
            if i != self.size - 1 {
                res.push(' ');
            }
        }
        res
    }
}

impl Index<i128> for Deck {
    type Output = i128;
    fn index(&self, index: i128) -> &Self::Output {
        let mut idx = (self.slope * index + self.intercept) % self.size;
        if idx < 0 {
            idx = idx + self.size;
        }
        &self.cards[idx as usize]
    }
}

impl IndexMut<i128> for Deck {
    fn index_mut(&mut self, index: i128) -> &mut Self::Output {
        let mut idx = (self.slope * index + self.intercept) % self.size;
        if idx < 0 {
            idx = idx + self.size;
        }
        self.cards[idx as usize].borrow_mut()
    }
}

pub fn part1() -> Result<usize> {
    let mut deck = Deck::new(10007);
    execute(&mut deck, &read("input/d22.txt").unwrap());
    for i in 0..deck.cards.len() {
        if deck[i as i128] == 2019 {
            return Ok(i);
        }
    }
    panic!("couldn't find card 2019");
}

#[test]
fn mod_arith_cut_basic() {
    let mut deck = Deck::new(10);
    deck.cut(3);
    assert_eq!(deck.to_string(), "3 4 5 6 7 8 9 0 1 2")
}

#[test]
fn mod_arith_cut_basic_neg() {
    let mut deck = Deck::new(10);
    deck.cut(-4);
    assert_eq!(deck.to_string(), "6 7 8 9 0 1 2 3 4 5")
}

#[test]
fn mod_arith_deal_into_new_stack_basic() {
    let mut deck = Deck::new(10);
    deck.deal_into_new_stack();
    assert_eq!(deck.to_string(), "9 8 7 6 5 4 3 2 1 0")
}

#[test]
fn mod_arith_deal_with_increment_basic() {
    let mut deck = Deck::new(10);
    deck.deal_with_increment(3);
    assert_eq!(deck.to_string(), "0 7 4 1 8 5 2 9 6 3")
}

#[test]
fn mod_arith_chain1() {
    let mut deck = Deck::new(10);
    deck.deal_with_increment(7);
    deck.deal_into_new_stack();
    deck.deal_into_new_stack();
    assert_eq!(deck.to_string(), "0 3 6 9 2 5 8 1 4 7")
}

#[test]
fn mod_arith_chain2() {
    let mut deck = Deck::new(10);
    deck.cut(6);
    deck.deal_with_increment(7);
    deck.deal_into_new_stack();
    assert_eq!(deck.to_string(), "3 0 7 4 1 8 5 2 9 6")
}

#[test]
fn mod_arith_chain3() {
    let mut deck = Deck::new(10);
    deck.deal_with_increment(7);
    deck.deal_with_increment(9);
    deck.cut(-2);
    assert_eq!(deck.to_string(), "6 3 0 7 4 1 8 5 2 9")
}

#[test]
fn mod_arith_chain4() {
    let mut deck = Deck::new(10);
    deck.deal_into_new_stack();
    deck.cut(-2);
    deck.deal_with_increment(7);
    deck.cut(8);
    deck.cut(-4);
    deck.deal_with_increment(7);
    deck.cut(3);
    deck.deal_with_increment(9);
    deck.deal_with_increment(3);
    deck.cut(-1);
    assert_eq!(deck.to_string(), "9 2 5 8 1 4 7 0 3 6");
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
