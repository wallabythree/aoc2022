use std::collections::VecDeque;

enum Operation {
    Addition(usize),
    Exponentiation(u32),
    Multiplication(usize)
}

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
                "old" => {
                    Operation::Exponentiation(2)
                },
                multiplier => {
                    Operation::Multiplication(multiplier.parse().unwrap())
                }
            },
            _ => Operation::Addition(operand.parse().unwrap())
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
        let modulo: usize = monkeys
            .iter()
            .map(|monkey| monkey.divisor)
            .product();

        for i in 0..monkeys.len() {

            while let Some(mut item) = monkeys[i].items.pop_front() {

                item = match monkeys[i].operation {
                    Operation::Addition(addend) => item + addend,
                    Operation::Exponentiation(exponent) => item.pow(exponent),
                    Operation::Multiplication(multiplier) => item * multiplier,
                };

                if let Some(divisor) = worry_divisor {
                    item /= divisor
                };

                item %= modulo;

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

    for _ in  0..10000 { 
        troop.play_round(None);
    }

    troop.monkey_business()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "Monkey 0:\n\
                                Starting items: 79, 98\n\
                                Operation: new = old * 19\n\
                                Test: divisible by 23\n\
                                If true: throw to monkey 2\n\
                                If false: throw to monkey 3\n\
                              \n\
                              Monkey 1:\n\
                                Starting items: 54, 65, 75, 74\n\
                                Operation: new = old + 6\n\
                                Test: divisible by 19\n\
                                If true: throw to monkey 2\n\
                                If false: throw to monkey 0\n\
                              \n\
                              Monkey 2:\n\
                                Starting items: 79, 60, 97\n\
                                Operation: new = old * old\n\
                                Test: divisible by 13\n\
                                If true: throw to monkey 1\n\
                                If false: throw to monkey 3\n\
                              \n\
                              Monkey 3:\n\
                                Starting items: 74\n\
                                Operation: new = old + 3\n\
                                Test: divisible by 17\n\
                                If true: throw to monkey 0\n\
                                If false: throw to monkey 1\n";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 10605);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 2713310158);
    }
}

