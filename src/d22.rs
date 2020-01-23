use std::borrow::BorrowMut;
use std::fs::File;
use std::io::Result;
use std::io::{BufRead, BufReader};
use std::ops::{Index, IndexMut};

pub struct Deck {
    cards: Vec<u64>,
    reverse: bool,
    top: usize,
}

impl Deck {
    pub fn new(size: usize) -> Deck {
        let mut cards_vec: Vec<u64> = Vec::with_capacity(size as usize);
        for i in 0..size {
            cards_vec.push(i as u64);
        }
        return Deck {
            cards: cards_vec,
            reverse: false,
            top: 0,
        };
    }

    pub fn deal_with_increment(&mut self, increment: usize) {
        let len = self.cards.len();
        let mut new_vec: Vec<u64> = vec![0; len];
        for element in 0..len {
            let idx_in_new = (element * increment) % len;
            new_vec[idx_in_new] = self[element];
        }
        self.cards = new_vec;
        self.reverse = false;
        self.top = 0;
    }

    pub fn deal_into_new_stack(&mut self) {
        self.reverse = !self.reverse;
    }

    pub fn cut(&mut self, n: i64) {
        let len = self.cards.len();
        let top_tmp: i64;
        if !self.reverse {
            top_tmp = (self.top as i64 + n) % len as i64;
        } else {
            top_tmp = (self.top as i64 - n) % len as i64;
        }
        if top_tmp < 0 {
            self.top = (top_tmp + len as i64) as usize;
        } else {
            self.top = top_tmp as usize;
        }
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for i in 0..self.cards.len() {
            print!("{},", self[i]);
        }
        print!("\n");
    }
}

impl Index<usize> for Deck {
    type Output = u64;
    fn index(&self, index: usize) -> &Self::Output {
        return if !self.reverse {
            let idx = (self.top + index) % self.cards.len();
            &self.cards[idx]
        } else {
            let len = self.cards.len() as i64;
            let mut idx = ((self.top as i64) - (index as i64) - 1) % len;
            if idx < 0 {
                idx = idx + len;
            }
            &self.cards[idx as usize]
        };
    }
}

impl IndexMut<usize> for Deck {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return if !self.reverse {
            let idx = (self.top + index) % self.cards.len();
            self.cards[idx].borrow_mut()
        } else {
            let len = self.cards.len() as i64;
            let mut idx = ((self.top as i64) - (index as i64) - 1) % len;
            if idx < 0 {
                idx = idx + len;
            }
            self.cards[idx as usize].borrow_mut()
        };
    }
}

pub fn part1() -> Result<usize> {
    let mut deck = Deck::new(10007);
    execute(&mut deck, &read("input/d22.txt").unwrap());
    for i in 0..deck.cards.len() {
        if deck[i] == 2019 {
            return Ok(i);
        }
    }
    panic!("couldn't find card 2019");
}

fn execute(deck: &mut Deck, instructions: &Vec<String>) {
    instructions.iter().for_each(|instruction| {
        if instruction.starts_with("cut ") {
            let (_, num_str) = instruction.split_at(3);
            let num = num_str.trim().parse::<i64>().unwrap();
            deck.cut(num);
        } else if instruction.starts_with("deal with increment ") {
            let (_, num_str) = instruction.split_at("deal with increment ".len() - 1);
            let num = num_str.trim().parse::<usize>().unwrap();
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
