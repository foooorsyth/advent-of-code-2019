use std::borrow::BorrowMut;
use std::io::Result;
use std::ops::{Index, IndexMut};

struct Deck {
    cards: Vec<u64>,
    reverse: bool,
    top: usize,
    step: usize,
}

impl Deck {
    fn new(size: usize) -> Deck {
        let mut cards_vec: Vec<u64> = Vec::with_capacity(size as usize);
        for i in 0..size {
            cards_vec.push(i as u64);
        }
        return Deck {
            cards: cards_vec,
            reverse: false,
            top: 0,
            step: 1,
        };
    }

    fn deal_into_new_stack(&mut self) {
        self.reverse = !self.reverse;
    }

    fn cut(&mut self, n: i64) {
        let len = self.cards.len();
        let top_tmp = (self.top as i64 + n) % len as i64;
        if top_tmp < 0 {
            self.top = (top_tmp + len as i64) as usize;
        } else {
            self.top = top_tmp as usize;
        }
    }

    fn print(&self) {
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
            let mut idx = (self.top + index) % self.cards.len();
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
        return self[index].borrow_mut();
    }
}

pub fn part1() -> Result<u64> {
    //let mut t = vec![0, 1, 2, 3, 4];
    let mut deck = Deck::new(10);
    deck.print();
    deck.deal_into_new_stack();
    deck.print();
    deck.cut(-4);
    deck.print();
    deck.deal_into_new_stack();
    deck.print();
    deck.deal_into_new_stack();
    deck.print();
    deck.deal_into_new_stack();
    deck.print();
    deck.cut(4);
    deck.print();
    Ok(1)
}
