use std::collections::HashMap;

#[derive(Clone)]
enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Clone)]
struct Operation {
    operator: Operator,
    operand1: String,
    operand2: String,
}

#[derive(Clone)]
enum Expression {
    Number(i64),
    Operation(Operation),
}

#[derive(Clone)]
struct Monkey {
    name: String,
    val: Expression,
}

impl Monkey {
    fn from(input: &str) -> Self {
        let parts: Vec<&str> = input.split([' ', ':']).collect();

        let name = parts[0].to_owned();
        
        let val = if parts.len() == 5 {
            let operator = match parts[3].chars().next().unwrap() {
                '+' => Operator::Addition,
                '-' => Operator::Subtraction,
                '*' => Operator::Multiplication,
                '/' => Operator::Division,
                _ => panic!()
            };

            Expression::Operation(
                Operation {
                    operator,
                    operand1: parts[2].to_owned(),
                    operand2: parts[4].to_owned(),
                }
            )
        } else {
            Expression::Number(parts[2].parse().unwrap())
        };

        Self { name, val }
    }

    fn val(&self, table: &HashMap<String, Monkey>) -> Result<i64, ()> {
        match &self.val {
            Expression::Number(n) => Ok(*n),
            Expression::Operation(o) => {
                let a = table.get(&o.operand1).ok_or(())?.val(table)?;
                let b = table.get(&o.operand2).ok_or(())?.val(table)?;

                let result = match o.operator {
                    Operator::Addition => a + b,
                    Operator::Subtraction => a - b,
                    Operator::Multiplication => a * b,
                    Operator::Division => a / b,
                };

                Ok(result)
            },
        }
    }

    #[allow(clippy::unnecessary_unwrap)]
    fn solve_for(
        &self,
        table: &HashMap<String, Monkey>,
        variable: &str,
        val: i64
    ) -> Result<i64, ()> {

        match &self.val {
            Expression::Number(_) => Err(()),
            Expression::Operation(o) => {
                let (left, right) = (
                    table.get(&o.operand1),
                    table.get(&o.operand2)
                );

                if left.is_some() && left.unwrap().val(table).is_ok() {

                    let left = left.unwrap().val(table).unwrap();

                    let val = match o.operator {
                        Operator::Addition => val - left,
                        Operator::Subtraction => left - val,
                        Operator::Multiplication => val / left,
                        Operator::Division => left / val,
                    };

                    if o.operand2 == variable {
                        return Ok(val);
                    }

                    table
                        .get(&o.operand2)
                        .unwrap()
                        .solve_for(table, variable, val)

                } else if right.is_some() && right.unwrap().val(table).is_ok() {

                    let right = right.unwrap().val(table).unwrap();

                    let val = match o.operator {
                        Operator::Addition => val - right,
                        Operator::Subtraction => val + right,
                        Operator::Multiplication => val / right,
                        Operator::Division => val * right,
                    };

                    if o.operand1 == variable {
                        return Ok(val);
                    }

                    table
                        .get(&o.operand1)
                        .unwrap()
                        .solve_for(table, variable, val)

                } else {
                    Err(())
                }
            }
        }
    }
}

pub fn part1(input: &str) -> i64 {
    let mut table: HashMap<String, Monkey> = HashMap::new();

    input
        .lines()
        .map(Monkey::from)
        .for_each(|monkey| { table.insert(monkey.name.clone(), monkey); } );

    table
        .get("root")
        .unwrap()
        .val(&table)
        .unwrap()
}

pub fn part2(input: &str) -> i64 {
    let mut table: HashMap<String, Monkey> = HashMap::new();

    input
        .lines()
        .map(Monkey::from)
        .for_each(|monkey| { table.insert(monkey.name.clone(), monkey); } );

    table.remove("humn");

    let root_monkey = table.get("root").unwrap();

    let (left, right) = match &root_monkey.val {
        Expression::Operation(o) => {
            (table.get(&o.operand1).unwrap(), table.get(&o.operand2).unwrap())
        },
        _ => panic!(),
    };

    if let Ok(a) = left.val(&table) {
        table
            .get(&right.name)
            .unwrap()
            .solve_for(&table, "humn", a)
            .unwrap()
    } else if let Ok(b) = right.val(&table) {
        table
            .get(&left.name)
            .unwrap()
            .solve_for(&table, "humn", b)
            .unwrap()
    } else {
        panic!()
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "root: pppw + sjmn\n\
                              dbpl: 5\n\
                              cczh: sllz + lgvd\n\
                              zczc: 2\n\
                              ptdq: humn - dvpt\n\
                              dvpt: 3\n\
                              lfqf: 4\n\
                              humn: 5\n\
                              ljgn: 2\n\
                              sjmn: drzm * dbpl\n\
                              sllz: 4\n\
                              pppw: cczh / lfqf\n\
                              lgvd: ljgn * ptdq\n\
                              drzm: hmdt - zczc\n\
                              hmdt: 32\n";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 152);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 301);
    }
}

