use std::env;
use std::fs;

#[derive(PartialEq)]
enum Operation {
    Add,
    Multiply,
    Finished,
    Error,
}

fn get_operation_from_instruction(opcode: i32) -> Operation {
    match opcode {
        1 => Operation::Add,
        2 => Operation::Multiply,
        99 => Operation::Finished,
        _ => Operation::Error,
    }
}

fn compute_program(instructions: &Vec<i32>) -> Vec<i32> {
    let mut computed_instructions = instructions.to_vec(); // Copy the vector
    let mut finished = false;
    let mut cursor = 0;

    while !finished {
        let operation = get_operation_from_instruction(computed_instructions[cursor]);

        let result = match operation {
            Operation::Error => panic!("The opcode was invalid"),
            Operation::Finished => {
                finished = true;
                continue;
            }
            Operation::Add => {
                computed_instructions[computed_instructions[cursor + 1] as usize]
                    + computed_instructions[computed_instructions[cursor + 2] as usize]
            }
            Operation::Multiply => {
                computed_instructions[computed_instructions[cursor + 1] as usize]
                    * computed_instructions[computed_instructions[cursor + 2] as usize]
            }
        };

        let target = computed_instructions[cursor + 3] as usize;
        computed_instructions[target] = result;
        cursor = cursor + 4;
    }

    return computed_instructions;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let instructions: Vec<i32> = vec![1, 0, 0, 0, 99];
        assert_eq!(compute_program(&instructions), [2, 0, 0, 0, 99])
    }

    #[test]
    fn test_multiplication() {
        let instructions: Vec<i32> = vec![2, 3, 0, 3, 99];

        assert_eq!(compute_program(&instructions), [2, 3, 0, 6, 99])
    }

    #[test]
    fn test_error() {
        let instructions: Vec<i32> = vec![2, 4, 4, 5, 99, 0];

        assert_eq!(compute_program(&instructions), [2, 4, 4, 5, 99, 9801])
    }

    #[test]
    fn test_long() {
        let instructions: Vec<i32> = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];

        assert_eq!(
            compute_program(&instructions),
            [30, 1, 1, 4, 2, 5, 6, 0, 99]
        )
    }

    #[test]
    fn test_example() {
        let instructions: Vec<i32> = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];

        assert_eq!(
            compute_program(&instructions),
            [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        )
    }
}

fn fix_program(instructions: &Vec<i32>) -> Vec<i32> {
    let mut fixed_instructions = instructions.to_vec(); // Copy the vector

    fixed_instructions[1] = 12;
    fixed_instructions[2] = 2;

    return fixed_instructions;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Reading {}...", filename);

    let content = fs::read_to_string(filename).expect("Error while reading the file");
    let program: Vec<&str> = content.split(',').collect();
    let (numbers, _errors): (Vec<_>, Vec<_>) = program
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    let instructions: Vec<i32> = numbers.into_iter().map(Result::unwrap).collect();

    println!("Fixing the 1202 program...");
    let fixed_instructions = fix_program(&instructions);

    println!("Computing the 1202 program...");
    let result = compute_program(&fixed_instructions);

    println!("PART 1: The value left at position 0 is: {}", result[0]);
}
