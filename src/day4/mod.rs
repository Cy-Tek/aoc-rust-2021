use std::collections::HashMap;
use std::str::FromStr;

struct Board {
    tiles: [usize; 25],
    called: HashMap<usize, bool>,
}

struct Score {
    pub calls: usize,
    pub score: usize,
}

impl Board {
    pub fn set_tile(&mut self, index: usize, value: usize) {
        self.tiles[index] = value;
        self.called.insert(index, false);
    }

    pub fn check_called(&self, number: usize) -> bool {
        self.called[&number]
    }

    pub fn call(&mut self, number: usize) {
        self.called.insert(number, true);
    }

    fn check_winner(&self) -> bool {
        for i in 0..5 {
            if self.check_row(i) || self.check_col(i) {
                return true;
            }
        }

        false
    }

    fn check_row(&self, row: usize) -> bool {
        for i in 0..5 {
            if !self.check_called(row * 5 + i) {
                return false;
            }
        }

        true
    }

    fn check_col(&self, col: usize) -> bool {
        for i in 0..5 {
            if !self.check_called(i * 5 + col) {
                return false;
            }
        }

        true
    }

    fn unmarked_sum(&self) -> usize {
        self.called
            .iter()
            .fold(0, |acc, (key, value)| acc + if !*value { *key } else { 0 })
    }

    pub fn final_score(&mut self, calls: &[usize]) -> Option<Score> {
        let mut tracker = 0;
        for call in calls {
            self.call(*call);
            tracker += 1;

            if self.check_winner() {
                return Some(Score {
                    calls: tracker,
                    score: self.unmarked_sum(),
                });
            }
        }

        None
    }
}

impl FromStr for Board {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = Board {
            tiles: [0; 25],
            called: HashMap::new(),
        };
        s.split_ascii_whitespace()
            .enumerate()
            .for_each(|(index, num)| {
                let num = num
                    .parse::<usize>()
                    .expect("Could not convert string to number");
                board.set_tile(index, num);
            });

        Ok(board)
    }
}

pub fn part_1(input: &str) -> usize {
    let mut elements = input.split("\n\n");
    let calls: Vec<usize> = elements
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<usize>())
        .collect();
    
    elements
        .flat_map(|s| {
            s.parse::<Board>()
                .map_or(None, |mut board| board.final_score(&calls))
        })
        .min_by_key(|score| score.calls)
        .expect("Should be at least one board")
        .score
}
