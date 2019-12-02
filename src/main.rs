//use std::env;
use std::fs;

fn main() {
    //println!("Hello, world!");
    //let path = env::current_dir();
    //println!("Current dir: {:?}", path);

    aoc_day_two_second();
}


fn print_message_and_die(message: String) {
    println!("ERROR!!!");
    println!("{}", message);
    std::process::exit(1);
}

#[allow(dead_code)]
fn aoc_day_one_first() {
    let input_raw = fs::read_to_string("input_files/input_1a.txt").expect("File read went poop-shaped");
    let input_data = input_raw.lines();

    let mut module_weight= 0u64;

    let mut fuel_amount_needed = 0u64;

    for module_weight_string in input_data {
        let module_weight_tmp = module_weight_string.parse::<u64>();
        match module_weight_tmp {
            Ok(v) => module_weight= v,
            Err(_e) => print_message_and_die("Error parsing Module Weight".to_string()),
        }

        if module_weight > 8 {
            //We only need to care if the end result actually is more than 0
            fuel_amount_needed += (module_weight / 3) - 2;
        }
    }


    println!("Fuel amount needed: {}", fuel_amount_needed);
}

#[allow(dead_code)]
fn aoc_day_one_second() {
    let input_raw = fs::read_to_string("input_files/input_1a.txt").expect("File read went poop-shaped");
    let input_data = input_raw.lines();

    let mut module_weight= 0u64;

    let mut fuel_amount_needed = 0u64;

    for module_weight_string in input_data {
        let module_weight_tmp = module_weight_string.parse::<u64>();
        match module_weight_tmp {
            Ok(v) => module_weight= v,
            Err(_e) => print_message_and_die("Error parsing Module Weight".to_string()),
        }

        while module_weight > 8 {
            //We only need to care if the end result actually is more than 0
            //NOTE: Module weight becomes Fuel weight.
            module_weight /= 3;
            module_weight -= 2;
            fuel_amount_needed += module_weight;
        }
    }


    println!("Fuel amount needed: {}", fuel_amount_needed);
}



#[allow(dead_code)]
fn aoc_day_two_first() {
    let input_raw = fs::read_to_string("input_files/input_2.txt").expect("File read went poop-shaped");
    let input_data:Vec<&str> = input_raw.split(",").collect();

    let mut program: [u64; 500] = [0; 500];

    let mut counter = 0;
    for value in input_data {
        let value_int = value.parse::<u64>();
        match value_int {
            Ok(v) => program[counter] = v,
            Err(_e) => print_message_and_die("Parsing input value failed".to_string()),
        }
        counter += 1;
    }

    // Values-fix according to challange
    program[1] = 12;
    program[2] = 2;


    let mut found_end = false;
    let mut program_pos = 0;
    while !found_end {
        let opcode = program[program_pos];

        match opcode {
            1 => program[program[program_pos as usize + 3] as usize] = program[program[program_pos as usize + 1] as usize] + program[program[program_pos as usize + 2] as usize],
            2 => program[program[program_pos as usize + 3] as usize] = program[program[program_pos as usize + 1] as usize] * program[program[program_pos as usize + 2] as usize],
            99 => found_end = true,
            _ => print_message_and_die("Found bad opcode".to_string()),
        }

        program_pos += 4;
    }


    println!("Result: {}", program[0]);
}



#[allow(dead_code)]
fn aoc_day_two_second() {
    for noun in 0..99u64 {
        for verb in 0..99u64 {
            let input_raw = fs::read_to_string("input_files/input_2.txt").expect("File read went poop-shaped");
            let input_data: Vec<&str> = input_raw.split(",").collect();

            let mut program: [u64; 500] = [0; 500];

            let mut counter = 0;
            for value in input_data {
                let value_int = value.parse::<u64>();
                match value_int {
                    Ok(v) => program[counter] = v,
                    Err(_e) => print_message_and_die("Parsing input value failed".to_string()),
                }
                counter += 1;
            }

            // Values-fix according to challange
            program[1] = noun;
            program[2] = verb;


            let mut found_end = false;
            let mut program_pos = 0;
            while !found_end {
                let opcode = program[program_pos];

                match opcode {
                    1 => program[program[program_pos as usize + 3] as usize] = program[program[program_pos as usize + 1] as usize] + program[program[program_pos as usize + 2] as usize],
                    2 => program[program[program_pos as usize + 3] as usize] = program[program[program_pos as usize + 1] as usize] * program[program[program_pos as usize + 2] as usize],
                    99 => found_end = true,
                    _ => print_message_and_die("Found bad opcode".to_string()),
                }

                program_pos += 4;
            }

            if program[0] == 19690720 {
                // Found the key!
                println!("Answer: {}", 100 * noun + verb);

                std::process::exit(0);
            }
        }
    }
}