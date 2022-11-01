use crate::utils::read_input_from_string;
use std::str::FromStr;

#[derive(Clone)]
pub struct Input {
    bits: Vec<u8>,
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bits = s
            .chars()
            .flat_map(|c| c.to_digit(10))
            .map(|num| num as u8)
            .collect();

        Ok(Input { bits })
    }
}

pub fn part_1(input: &str) -> u64 {
    let inputs = read_input_from_string::<Input>(input);
    let tracker: Vec<(u64, u64)> = vec![(0, 0); inputs[0].bits.len()];

    let gamma = inputs
        .iter()
        .fold(tracker, |mut acc, input| {
            for (i, bit) in input.bits.iter().enumerate() {
                match *bit {
                    0 => acc[i].0 += 1,
                    1 => acc[i].1 += 1,
                    _ => unreachable!(),
                }
            }

            acc
        })
        .iter()
        .fold(0, |mut acc, input| {
            acc <<= 1;
            match input.0 > input.1 {
                true => acc,
                false => acc | 1,
            }
        });

    gamma * (gamma ^ 0b1111_1111_1111)
}

pub fn part_2(input: &str) -> u64 {
    let inputs = read_input_from_string::<Input>(input);

    get_co2_rating(&inputs) * get_o2_rating(&inputs)
}

#[derive(Clone, Copy, Debug)]
enum BitPattern {
    Less,
    More
}

fn get_co2_rating(inputs: &[Input]) -> u64 {
    get_bit_number_from_rating(inputs, BitPattern::Less)
}

fn get_o2_rating(inputs: &[Input]) -> u64 {
    get_bit_number_from_rating(inputs, BitPattern::More)
}

fn get_bit_number_from_rating(inputs: &[Input], pattern: BitPattern) -> u64 {
    let num_bits = inputs[0].bits.len();
    let mut temp = Vec::from(inputs);

    for i in 0..num_bits {
        temp = filter_inputs(&temp, i, pattern);
        if temp.len() == 1 || i == num_bits - 1 {
            return temp[0].bits.iter().fold(0, |mut acc, bit| {
                acc <<= 1;
                acc | *bit as u64
            });
        }
    }

    0
}

fn filter_inputs(inputs: &[Input], bit_pos: usize, pattern: BitPattern) -> Vec<Input> {
    let mut zeros = 0;
    let mut ones = 0;

    for input in inputs {
        match input.bits.get(bit_pos) {
            Some(0) => zeros += 1,
            Some(1) => ones += 1,
            _ => unreachable!("The bits should only be 0 or 1")
        };
    }

    let bit = match pattern {
        BitPattern::Less => if zeros <= ones { 0 } else { 1 },
        BitPattern::More => if ones >= zeros { 1 } else { 0 },
    };

    inputs.iter().filter(|input| input.bits[bit_pos] == bit).cloned().collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_2() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        let result = super::part_2(input);
        assert_eq!(result, 230)
    }
}