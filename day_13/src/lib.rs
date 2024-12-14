use prelude::*;

#[cfg(test)]
mod test;

struct Pair(i64, i64);

struct Machine {
    button_a: Pair,
    button_b: Pair,
    prize: Pair,
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
        todo!()
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
