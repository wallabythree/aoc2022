use std::collections::VecDeque;

#[derive(Debug)]
enum Operation {
    Addend(usize),
    Exponent(usize),
    Multiplier(usize)
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    divisor: usize,
    success: usize,
    failure: usize,
    inspections: usize
}

impl Monkey {
    fn from(input: &str) -> Self {
        let parts = input
            .split(['\n', ':'])
            .collect::<Vec<_>>();

        let items = parts[3]
            .split(',')
            .map(|item| item.trim().parse::<usize>().unwrap())
            .collect::<VecDeque<_>>();

        let mut operation_parts = parts[5].trim().split(' ');
        let operator = operation_parts.nth(3).unwrap();
        let operand = operation_parts.next().unwrap();

        let operation = match operator {
            "*" => match operand {
                "old" => Operation::Exponent(2),
                multiplier => Operation::Multiplier(multiplier.parse().unwrap())
            },
            _ => Operation::Addend(operand.parse().unwrap())
        };

        let divisor: usize = parts[7]
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let success: usize = parts[9]
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let failure: usize = parts[11]
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap();

        Self { items, operation, divisor, success, failure, inspections: 0 }
    }
}

#[derive(Debug)]
struct Troop {
    monkeys: Vec<Monkey>,
}

impl Troop {
    fn from(input: &str) -> Self {
        let monkeys = input
            .split("\n\n")
            .map(Monkey::from)
            .collect();

        Self { monkeys }
    }

    fn play_round(&mut self, worry_divisor: Option<usize>) {
        let monkeys = &mut self.monkeys;

        for i in 0..monkeys.len() {

            while let Some(mut item) = monkeys[i].items.pop_front() {

                item = match monkeys[i].operation {
                    Operation::Exponent(exponent) => item.pow(exponent as u32),
                    Operation::Multiplier(multiplier) => item * multiplier,
                    Operation::Addend(addend) => item + addend,
                };

                if let Some(divisor) = worry_divisor {
                    item /= divisor
                };

                let target = if item % monkeys[i].divisor == 0 {
                    monkeys[i].success
                } else {
                    monkeys[i].failure
                };

                monkeys[target].items.push_back(item);
                monkeys[i].inspections += 1;
            }
        }
    }

    fn monkey_business(&self) -> usize {
        let mut inspections: Vec<_> = self
            .monkeys
            .iter()
            .map(|monkey| monkey.inspections)
            .collect();

        inspections.sort_by(|a, b| b.cmp(a));
        inspections.truncate(2);

        inspections.iter().product()
    }
}

pub fn part1(input: &str) -> usize {
    let mut troop = Troop::from(input);

    for _ in  0..20 { 
        troop.play_round(Some(3));
    }

    troop.monkey_business()
}

pub fn part2(input: &str) -> usize {
    let mut troop = Troop::from(input);

    for i in  0..10000 { 
        troop.play_round(None);

        println!("== After round {} ==", i);

        for j in 0..troop.monkeys.len() {
            println!("Monkey {} inspected items {} times,", j, troop.monkeys[j].inspections);
        }

        println!();
    }

    troop.monkey_business()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 10605);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 2713310158);
    }
}

