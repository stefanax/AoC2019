//use std::env;
use std::fs;

fn main() {
    //println!("Hello, world!");
    //let path = env::current_dir();
    //println!("Current dir: {:?}", path);

    aoc_day_three_second();
}


fn print_message_and_die(message: &str) {
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
            Err(_e) => print_message_and_die("Error parsing Module Weight"),
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
            Err(_e) => print_message_and_die("Error parsing Module Weight"),
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
            Err(_e) => print_message_and_die("Parsing input value failed"),
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
            _ => print_message_and_die("Found bad opcode"),
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
                    Err(_e) => print_message_and_die("Parsing input value failed"),
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
                    _ => print_message_and_die("Found bad opcode"),
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



#[allow(dead_code)]
fn aoc_day_three_first() {
    println!("Meep-1");
    let input_raw = fs::read_to_string("input_files/input_3.txt").expect("File read went poop-shaped");
    let input_data: Vec<&str> = input_raw.split_whitespace().collect();

    println!("Meep0");

    let input_data_first: Vec<&str> = input_data[0].split(",").collect();
    let input_data_second: Vec<&str> = input_data[1].split(",").collect();

    println!("Meep1");

    //let aaa = vec![[0;5000];10000];
    let mut grid = vec![vec![0u8; 100000]; 100000];
    println!("Meep1.1");
    let mut x = 50000i64;
    let mut y = 50000i64;
    for cable_instruction in input_data_first {
        let direction = &cable_instruction[0..1];
        let distance = &cable_instruction[1..cable_instruction.len()];

        //println!("Dir: {}     Dist: {}", direction, distance);

        for _i in 0..distance.parse::<i32>().unwrap() {
            //println!("X: {}    Y: {}", x, y);
            match direction {
                "U" => y += 1,
                "D" => y -= 1,
                "R" => x += 1,
                "L" => x -= 1,
                _ => print_message_and_die("Direction does not exist!!!"),
            }

            grid[x as usize][y as usize] = 1;
        }
    }

    println!("Meep2");

    let mut minimum_distance = 99999999u64;
    // Reset vars for the second cable
    x = 50000;
    y = 50000;
    for cable_instruction in input_data_second {
        let direction = &cable_instruction[0..1];
        let distance = &cable_instruction[1..cable_instruction.len()];

        for _i in 0..distance.parse::<i32>().unwrap() {
            match direction {
                "U" => y += 1,
                "D" => y -= 1,
                "R" => x += 1,
                "L" => x -= 1,
                _ => print_message_and_die("Direction does not exist!!!"),
            }

            if grid[x as usize][y as usize] == 1 {
                let distance: u64 = (x-50000).abs() as u64 + (y-50000).abs() as u64;
                if distance < minimum_distance {
                    println!("X: {}     Y: {}     Dist: {}", x, y, distance);
                    minimum_distance = distance;
                }
            }
        }
    }

    println!("Shortest path: {}", minimum_distance);
}



#[allow(dead_code)]
fn aoc_day_three_second() {
    println!("Reading file");
    let input_raw = fs::read_to_string("input_files/input_3.txt").expect("File read went poop-shaped");
    let input_data: Vec<&str> = input_raw.split_whitespace().collect();

    println!("Splitting input");

    let input_data_first: Vec<&str> = input_data[0].split(",").collect();
    let input_data_second: Vec<&str> = input_data[1].split(",").collect();

    println!("Create grid");

    //let aaa = vec![[0;5000];10000];
    let mut grid = vec![vec![0u16; 100000]; 100000];

    println!("Map first cable");
    let mut x = 50000i64;
    let mut y = 50000i64;
    let mut step_counter = 1u16;
    for cable_instruction in input_data_first {
        if step_counter < std::u16::MAX-1000 {  //TODO: Fulhack so bad it hurts. But works...
            println!("Counter: {}     Max: {}" , step_counter, std::u16::MAX);
            let direction = &cable_instruction[0..1];
            let distance = &cable_instruction[1..cable_instruction.len()];

            //println!("Dir: {}     Dist: {}", direction, distance);

            for _i in 0..distance.parse::<i32>().unwrap() {
                //println!("X: {}    Y: {}", x, y);
                match direction {
                    "U" => y += 1,
                    "D" => y -= 1,
                    "R" => x += 1,
                    "L" => x -= 1,
                    _ => print_message_and_die("Direction does not exist!!!"),
                }

                grid[x as usize][y as usize] = step_counter;
                step_counter += 1;
            }
        }
    }

    println!("Map second cable");

    let mut minimum_distance = 99999999u64;
    // Reset vars for the second cable
    x = 50000;
    y = 50000;
    step_counter = 1;
    for cable_instruction in input_data_second {
        if step_counter as u64 > minimum_distance {
            // We CAN'T get a shorter path
            println!("Shortest path: {}", minimum_distance);
            std::process::exit(0);
        }
        let direction = &cable_instruction[0..1];
        let distance = &cable_instruction[1..cable_instruction.len()];

        for _i in 0..distance.parse::<i32>().unwrap() {
            match direction {
                "U" => y += 1,
                "D" => y -= 1,
                "R" => x += 1,
                "L" => x -= 1,
                _ => print_message_and_die("Direction does not exist!!!"),
            }

            if grid[x as usize][y as usize] > 0 {
                let distance: u64 = grid[x as usize][y as usize] as u64 + step_counter as u64;
                if distance < minimum_distance {
//                    println!("X: {}     Y: {}     Dist: {}", x, y, distance);
                    minimum_distance = distance;
                }
            }
            step_counter += 1;
        }
    }

    println!("Shortest path: {}", minimum_distance);
}