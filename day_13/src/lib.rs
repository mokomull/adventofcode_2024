use prelude::*;
use rational::extras::gcd;

#[cfg(test)]
mod test;

struct Pair(i64, i64);

struct Machine {
    button_a: Pair,
    button_b: Pair,
    prize: Pair,
}

impl Machine {
    fn solve(&self) -> Option<Pair> {
        let matrix = [
            [self.button_a.0, self.button_b.0, self.prize.0],
            [self.button_a.1, self.button_b.1, self.prize.1],
        ];
        let result = integer_rref(matrix)?;

        // if everything balances out and we have an identity matrix on the left side
        if result[0][0] == 1 && result[0][1] == 0 && result[1][0] == 0 && result[1][1] == 1 {
            Some(Pair(result[0][2], result[1][2]))
        } else {
            None
        }
    }
}

type Matrix = [[i64; 3]; 2];
fn integer_rref(mut matrix: Matrix) -> Option<Matrix> {
    // eliminate the first column of the second row
    let a_gcd = gcd(matrix[0][0] as i128, matrix[1][0] as i128) as i64;
    let x_factor = matrix[1][0] / a_gcd;
    let y_factor = matrix[0][0] / a_gcd;
    for col in 0..3 {
        matrix[1][col] = matrix[1][col] * y_factor - matrix[0][col] * x_factor;
    }
    assert_eq!(0, matrix[1][0]);

    // if this also eliminated the second column, then we're either over- or under-constrained
    if matrix[1][1]== 0 {
        // just return None in both cases -- we don't actually care if it's unwinnable or if all
        // integers are solutions, because we'd still end up spending (0, 0) tokens.
        return None;
    }

    // make the leading term positive just because I forget how the % operator works in Rust ... I
    // just know it's surprising.
    if matrix[1][1] < 0 {
        for col in 0..3 {
            matrix[1][col] *= -1;
        }
    }

    // eliminate the integer multiple across the second row
    if matrix[1][2] % matrix[1][1] != 0 {
        return None;
    }
    let divisor = matrix[1][1];
    for col in 0..3 {
        matrix[1][col] /= divisor;
    }
    assert_eq!(0, matrix[1][0]);
    assert_eq!(1, matrix[1][1]);

    // use the second row to back-substitude in the first row
    let y_factor = matrix[0][1];
    for col in 0..3 {
        matrix[0][col] = matrix[0][col] - matrix[1][col] * y_factor;
    }
    assert_eq!(0, matrix[0][1]);

    // in the general case we would also have to make the leading term of the top row positive here,
    // but we know it *started* positive and it hasn't been changed yet.

    // now eliminate the integer multiple across the top row
    if matrix[0][2] % matrix[0][0] != 0 {
        // also no integer solutions
        return None;
    }
    let divisor = matrix[0][0];
    for col in 0..3 {
        matrix[0][col] /= divisor;
    }

    Some(matrix)
}

pub struct Solution {
    machines: Vec<Machine>,
}

impl Day for Solution {
    fn new(input: &str) -> Self {
        let mut machines = Vec::new();
        let mut lines = input.lines();

        loop {
            let Some(top) = lines.next() else { break };
            let Some(middle) = lines.next() else { break };
            let Some(bottom) = lines.next() else { break };

            let button_a = parse_button(top);
            let button_b = parse_button(middle);
            let prize = parse_prize(bottom);

            machines.push(Machine {
                button_a,
                button_b,
                prize,
            });

            let Some(_) = lines.next() else { break };
        }

        Solution { machines }
    }

    fn part1(&self) -> anyhow::Result<u64> {
        Ok(self
            .machines
            .iter()
            .filter_map(Machine::solve)
            .map(|Pair(a, b)| a + b)
            .sum::<i64>() as u64)
    }

    fn part2(&self) -> anyhow::Result<u64> {
        todo!()
    }
}

fn parse_button(input: &str) -> Pair {
    let (_, input) = input.split_once(": ").expect("no colon");
    let (x, y) = input.split_once(", ").expect("no comma");

    Pair(
        x.strip_prefix("X+")
            .expect("didn't start with X+")
            .parse()
            .unwrap(),
        y.strip_prefix("Y+")
            .expect("didn't start with Y+")
            .parse()
            .unwrap(),
    )
}

fn parse_prize(input: &str) -> Pair {
    let (_, input) = input.split_once(": ").expect("no colon");
    let (x, y) = input.split_once(", ").expect("no comma");

    Pair(
        x.strip_prefix("X=")
            .expect("didn't start with X=")
            .parse()
            .unwrap(),
        y.strip_prefix("Y=")
            .expect("didn't start with Y=")
            .parse()
            .unwrap(),
    )
}
