use anyhow::Context;
use std::collections::{HashMap, VecDeque};

use crate::Day;

#[derive(Debug)]
pub struct Day20 {
    numbers: HashMap<usize, isize>,
    list: VecDeque<usize>,
}

impl Day for Day20 {
    const NAME: &'static str = "Day 20: Grove Positioning System 🌳📍🌳";

    fn solve() -> anyhow::Result<(String, String)> {
        let day = Self::load("res/day20.txt")?;

        println!("{day:?}");

        Ok(("foo".to_string(), "bar".to_string()))
    }
}

impl Day20 {
    fn part1(&mut self) -> anyhow::Result<isize> {
        for index in 0..self.numbers.len() {
            let number = *self
                .numbers
                .get(&index)
                .context("Index should be populated")?;

            let next_num = if number.current_index < 0 {
                let num_abs = number.inner.abs() as usize;

                if num_abs <= index {
                    let next_num = index - num_abs;
                    for i in next_num..index {
                        *self
                            .numbers 
                            .get_mut(&i)
                            .context("Index should be populated")? += 1;
                    }

                    next_num
                } else {
                    let next_num = self.numbers.len() - 1 - (num_abs - index);

                    for i in next_num..self.numbers.len() {
                        *self
                            .numbers
                            .get_mut(&i)
                            .context("Index should be populated")? += 1;
                    }

                    next_num
                }
            } else {
            };

            self.numbers.insert(index, next_num);
        }

        todo!()
    }

    fn move_left(&mut self, index: usize) {



        if 

        let a = self.numbers.get_mut(&index).unwrap() as *mut Number;
        let b = self.numbers.get_mut(&(index + 1)).unwrap() as *mut Number;

        unsafe {
            std::ptr::swap(a, b);
        }
    }

    fn load(path: &str) -> anyhow::Result<Self> {
        Ok(Self {
            numbers: std::fs::read_to_string(path)?
                .lines()
                .enumerate()
                .map(|(i, l)| {
                    l.parse().map(|n| {
                        (
                            i,
                            Number {
                                current_index: i,
                                inner: n,
                            },
                        )
                    })
                })
                .collect::<Result<_, _>>()?,
        })
    }
}
