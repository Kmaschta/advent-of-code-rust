use std::env;
use std::fs;

fn compute_fuel_for_module(module_weight: i32) -> i32 {
    return (((module_weight as f64) / 3_f64).floor() as i32) - 2;
}

fn compute_recursive_fuel_for_module(weight: i32) -> i32 {
    let intermediate = compute_fuel_for_module(weight);

    if intermediate <= 0 {
        return 0;
    }

    return intermediate + compute_recursive_fuel_for_module(intermediate);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_fuel_for_module() {
        assert_eq!(compute_fuel_for_module(12), 2);
        assert_eq!(compute_fuel_for_module(14), 2);
        assert_eq!(compute_fuel_for_module(1969), 654);
        assert_eq!(compute_fuel_for_module(100756), 33583);
    }

    #[test]
    fn test_recursive_compute_fuel_for_module() {
        assert_eq!(compute_recursive_fuel_for_module(14), 2);
        assert_eq!(compute_recursive_fuel_for_module(1969), 966);
        assert_eq!(compute_recursive_fuel_for_module(100756), 50346);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Reading {}...", filename);

    let content = fs::read_to_string(filename).expect("Error while reading the file");
    let lines = content.lines().map(|line| String::from(line));
    let modules = lines.map(|module| {
        i32::from_str_radix(&module, 10).expect("Error while casting a string to i32")
    });

    println!("Computing the fuel for modules...");

    let result = modules
        .clone()
        .map(|module_weight| compute_fuel_for_module(module_weight))
        .fold(0, |acc, fuel_for_module| acc + fuel_for_module);

    println!("PART 1: The total fuel needed is {}", result);

    let result_part_2 = modules
        .clone()
        .map(|module_weight| compute_recursive_fuel_for_module(module_weight))
        .fold(0, |acc, fuel_for_module| acc + fuel_for_module);

    println!(
        "PART 2: The total fuel needed (recursively) is {}",
        result_part_2
    );
}
