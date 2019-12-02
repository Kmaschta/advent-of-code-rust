fn compute_program(instructions: &mut [i32]) {
    instructions[0] = 2;
    instructions[1] = 0;
    instructions[2] = 0;
    instructions[3] = 0;
    instructions[4] = 99;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let instructions: [i32; 5] = [1, 0, 0, 0, 99];
        let mut computed_instructions: [i32; 5] = instructions;
        compute_program(&mut computed_instructions);

        assert_eq!(computed_instructions, [2, 0, 0, 0, 99])
    }

    #[test]
    fn test_multiplication() {
        let instructions: [i32; 5] = [2, 3, 0, 3, 99];
        let mut computed_instructions: [i32; 5] = instructions;
        compute_program(&mut computed_instructions);

        assert_eq!(computed_instructions, [2, 3, 0, 6, 99])
    }

    #[test]
    fn test_error() {
        let instructions: [i32; 6] = [2, 4, 4, 5, 99, 0];
        let mut computed_instructions: [i32; 6] = instructions;
        compute_program(&mut computed_instructions);

        assert_eq!(computed_instructions, [2, 4, 4, 5, 99, 9801])
    }

    #[test]
    fn test_long() {
        let instructions: [i32; 9] = [1, 1, 1, 4, 99, 5, 6, 0, 99];
        let mut computed_instructions: [i32; 9] = instructions;
        compute_program(&mut computed_instructions);

        assert_eq!(computed_instructions, [30, 1, 1, 4, 2, 5, 6, 0, 99])
    }
}

fn main() {
    println!("Hello, world!");
}
