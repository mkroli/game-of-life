/*
 * Copyright 2016 Michael Krolikowski
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use rand::Rng;
use rustty::{Attr, Cell, CellAccessor, Color, Event, Terminal};
use std::io::Error;
use std::time::Duration;

fn occurrences<T: Eq>(v: Vec<T>, t: T) -> i32 {
    let mut occurrences = 0;
    for e in v {
        if e == t {
            occurrences = occurrences + 1;
        }
    }
    occurrences
}

pub struct GameOfLife {
    empty_cell: Cell,
    full_cell: Cell,
    term: Terminal
}

impl GameOfLife {
    pub fn new() -> Result<Self, Error> {
        let term = try!(Terminal::new());
        Ok(GameOfLife {
            empty_cell: Cell::new(' ', Color::Default, Color::Default, Attr::Default),
            full_cell: Cell::new('*', Color::Blue, Color::Blue, Attr::Default),
            term: term
        })
    }

    fn neighbors(&self, clone: &Vec<Cell>, col: usize, row: usize) -> Vec<bool> {
        fn previous(s: usize) -> usize {
            match s {
                0 => 0,
                s => s - 1
            }
        }

        fn next(s: usize, max: usize) -> usize {
            match s {
                s if s == max => max,
                s => s + 1
            }
        }

        let mut neighbors = Vec::new();
        for c in previous(col)..next(col, self.term.cols() - 1) + 1 {
            for r in previous(row)..next(row, self.term.rows() - 1) + 1 {
                if !(c == col && r == row) {
                    if let Some(pos) = self.term.pos_to_index(c, r) {
                        neighbors.push(clone[pos] == self.full_cell);
                    }
                }
            }
        }
        neighbors
    }

    fn fill<F>(&mut self, mut f: F) -> Result<(), Error> where F: FnMut(bool, Vec<bool>) -> bool {
        let clone: Vec<Cell> = self.term.to_vec();
        for row in 0..self.term.rows() {
            for col in 0..self.term.cols() {
                let n = self.neighbors(&clone, col, row);
                let c = if f(self.term[(col, row)] == self.full_cell, n) {
                    self.full_cell
                } else {
                    self.empty_cell
                };
                self.term[(col, row)] = c;
            }
        }
        self.term.swap_buffers()
    }

    fn initialize(&mut self) -> Result<(), Error> {
        let mut rng = ::rand::thread_rng();
        self.fill(|_, _| {
            rng.gen()
        })
    }

    pub fn run(&mut self, pause: u64) -> Result<(), Error> {
        try!(self.initialize());
        Ok(loop {
            match self.term.get_event(Duration::from_millis(pause)) {
                Ok(Some(Event::Key(' '))) => {
                    try!(self.initialize());
                    continue;
                },
                Ok(Some(Event::Key(_))) => break,
                Err(_) => break,
                _ => ()
            }
            try!(self.fill(|state, n| {
                let alive_neighbors = occurrences(n, true);
                alive_neighbors == 3 || (state && alive_neighbors == 2)
            }))
        })
    }

    pub fn start(pause: u64) -> Result<(), Error> {
        let mut game_of_life = try!(GameOfLife::new());
        game_of_life.run(pause)
    }
}
