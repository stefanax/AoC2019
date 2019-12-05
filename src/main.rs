//use std::env;
use std::io;
use std::fs;
use std::env::current_dir;

fn main() {
    //println!("Hello, world!");
    //let path = env::current_dir();
    //println!("Current dir: {:?}", path);

    aoc_day_five_first();
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


fn aoc_day_four_tick(input: u32) -> u32 {
    fn recurse(digits: &mut Vec<u32>, index: usize) {
        if digits[index] == 10 {
            digits[index-1] += 1;
            if index > 0 {
                recurse(digits, index-1);
                digits[index] = digits[index-1];
            }
        }
    }

    let mut digits: Vec<u32> = input.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();  //Note: Not the prettiest way, but honestly stolen from the Internets...

    digits[5] += 1;
    recurse(&mut digits, 5);

    //TODO: Make this pretty...
    let temp: u32 = digits[0] * 100000 + digits[1] * 10000 + digits[2] * 1000 + digits[3] * 100 + digits[4] * 10 + digits[5];

    return temp;
}

fn aoc_day_four_check_pairs(input: u32, index: usize) -> bool {
    let digits: Vec<_> = input.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();

    if index == 0 { return false; }

    if digits[index] == digits[index-1] { return true; }

    return aoc_day_four_check_pairs(digits[0] * 100000 + digits[1] * 10000 + digits[2] * 1000 + digits[3] * 100 + digits[4] * 10 + digits[5], index-1);
}

//TODO: Should perhaps just work with a Vec<u8> and not convert back-and-fourth? (Rethorical question...in prod code I would fix that)
#[allow(dead_code)]
fn aoc_day_four_first() {
    let start: u32 = 136777; //Orig: 136760
    let end: u32 = 595730;

    let mut current = start;  //Lower one because we add one the first thing we do...
    let mut found_matches_count = 0u16;

    while current <= end {
        if aoc_day_four_check_pairs(current, 5) {
            found_matches_count += 1;
            println!("Match: {}", current);
        } else {
            println!("No match: {}", current);
        }

        current = aoc_day_four_tick(current);  //TODO: Parse as reference?
    }

    println!("Amoount of possible passwords: {}", found_matches_count);
}


fn aoc_day_four_check_pairs_absolute(input: u32) -> bool {
    let digits: Vec<_> = input.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();

    if digits[0] == digits[1] && digits[1] != digits[2] { return true; }
    if digits[0] != digits[1] && digits[1] == digits[2] && digits[2] != digits[3] { return true; }
    if digits[1] != digits[2] && digits[2] == digits[3] && digits[3] != digits[4] { return true; }
    if digits[2] != digits[3] && digits[3] == digits[4] && digits[4] != digits[5] { return true; }
    if digits[3] != digits[4] && digits[4] == digits[5] { return true; }

    return false;
}


//TODO: Should perhaps just work with a Vec<u8> and not convert back-and-fourth? (Rethorical question...in prod code I would fix that)
#[allow(dead_code)]
fn aoc_day_four_second() {
    let start: u32 = 136777; //Orig: 136760
    let end: u32 = 595730;

    let mut current = start;  //Lower one because we add one the first thing we do...
    let mut found_matches_count = 0u16;

    while current <= end {
        if aoc_day_four_check_pairs_absolute(current) {
            found_matches_count += 1;
            println!("Match: {}", current);
        } else {
            println!("No match: {}", current);
        }

        current = aoc_day_four_tick(current);  //TODO: Parse as reference?
    }

    println!("Amoount of possible passwords: {}", found_matches_count);
}




fn intcode_computer(program: &mut Vec<i32>) -> bool {
    fn opcode_add(program: &mut Vec<i32>, program_pos: &mut usize) {
        let param_mode_immediate_a: bool = (program[*program_pos] % 1000) - (program[*program_pos] % 100) == 100;
        let param_mode_immediate_b: bool = (program[*program_pos] % 10000) - (program[*program_pos] % 1000) == 1000;
        let param_mode_immediate_c: bool = (program[*program_pos] % 100000) - (program[*program_pos] % 10000) == 10000;

        if param_mode_immediate_c {
            // This should be invalid as we can't store a value to an Immediate Value
            print_message_and_die("Can't store added values to Immediate Value!");
        }

        let value_a = if param_mode_immediate_a { program[*program_pos+1] } else { program[program[*program_pos+1] as usize] };
        let value_b = if param_mode_immediate_b { program[*program_pos+2] } else { program[program[*program_pos+2] as usize] };
        let store_pos: usize = program[*program_pos+3] as usize;
        program[store_pos] = value_a + value_b;
        *program_pos += 4;
    }

    fn opcode_multiply(program: &mut Vec<i32>, program_pos: &mut usize) {
        let param_mode_immediate_a: bool = (program[*program_pos] % 1000) - (program[*program_pos] % 100) == 100;
        let param_mode_immediate_b: bool = (program[*program_pos] % 10000) - (program[*program_pos] % 1000) == 1000;
        let param_mode_immediate_c: bool = (program[*program_pos] % 100000) - (program[*program_pos] % 10000) == 10000;

        if param_mode_immediate_c {
            // This should be invalid as we can't store a value to an Immediate Value
            print_message_and_die("Can't store added values to Immediate Value!");
        }

        let value_a = if param_mode_immediate_a { program[*program_pos+1] } else { program[program[*program_pos+1] as usize] };
        let value_b = if param_mode_immediate_b { program[*program_pos+2] } else { program[program[*program_pos+2] as usize] };
        let store_pos: usize = program[*program_pos+3] as usize;
        program[store_pos] = value_a * value_b;
        *program_pos += 4;
    }

    fn opcode_input(program: &mut Vec<i32>, program_pos: &mut usize) {
        let mut input_number: i32 = 999999;

        println!("Input: ");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                let input_number_parse = input.trim().parse::<i32>();
                match input_number_parse {
                    Ok(v) => input_number = v,
                    Err(e) => {println!("{}", e); print_message_and_die("Parsing STDIO input value failed")},
                }
            }
            Err(_error) => print_message_and_die("Error reading input...")
        }

        let store_pos: usize = program[*program_pos+1] as usize;
        program[store_pos] = input_number;
        *program_pos += 2;
    }

    fn opcode_output(program: &mut Vec<i32>, program_pos: &mut usize) {
        let param_mode_immediate_a: bool = (program[*program_pos] % 1000) - (program[*program_pos] % 100) == 100;
        let output = if param_mode_immediate_a { program[*program_pos+1] } else { program[program[*program_pos+1] as usize] };

        println!("Output: {}", output);
        *program_pos += 2;
    }

    fn opcode_jump_if_true(program: &mut Vec<i32>, program_pos: &mut usize) {
        let param_mode_immediate_a: bool = (program[*program_pos] % 1000) - (program[*program_pos] % 100) == 100;
        let param_mode_immediate_b: bool = (program[*program_pos] % 10000) - (program[*program_pos] % 1000) == 1000;

        let state_to_check = if param_mode_immediate_a { program[*program_pos+1] } else { program[program[*program_pos+1] as usize] };
        let new_pos = if param_mode_immediate_b { program[*program_pos+2] } else { program[program[*program_pos+2] as usize] };

        if state_to_check > 0 {
            *program_pos = new_pos as usize;
        } else {
            *program_pos += 3;
        }
    }

    fn opcode_jump_if_false(program: &mut Vec<i32>, program_pos: &mut usize) {
        let param_mode_immediate_a: bool = (program[*program_pos] % 1000) - (program[*program_pos] % 100) == 100;
        let param_mode_immediate_b: bool = (program[*program_pos] % 10000) - (program[*program_pos] % 1000) == 1000;

        let state_to_check = if param_mode_immediate_a { program[*program_pos+1] } else { program[program[*program_pos+1] as usize] };
        let new_pos = if param_mode_immediate_b { program[*program_pos+2] } else { program[program[*program_pos+2] as usize] };

        if state_to_check == 0 {
            *program_pos = new_pos as usize;
        } else {
            *program_pos += 3;
        }
    }

    fn opcode_check_if_less(program: &mut Vec<i32>, program_pos: &mut usize) {
        let param_mode_immediate_a: bool = (program[*program_pos] % 1000) - (program[*program_pos] % 100) == 100;
        let param_mode_immediate_b: bool = (program[*program_pos] % 10000) - (program[*program_pos] % 1000) == 1000;
        //let param_mode_immediate_c: bool = (program[*program_pos] % 100000) - (program[*program_pos] % 10000) == 10000;

        let state_to_check_a = if param_mode_immediate_a { program[*program_pos+1] } else { program[program[*program_pos+1] as usize] };
        let state_to_check_b = if param_mode_immediate_b { program[*program_pos+2] } else { program[program[*program_pos+2] as usize] };

        let store_pos = program[*program_pos+3];

        if state_to_check_a < state_to_check_b {
            program[store_pos as usize] = 1;
        } else {
            program[store_pos as usize] = 0;
        }
        *program_pos += 4;
    }

    fn opcode_check_if_equal(program: &mut Vec<i32>, program_pos: &mut usize) {
        let param_mode_immediate_a: bool = (program[*program_pos] % 1000) - (program[*program_pos] % 100) == 100;
        let param_mode_immediate_b: bool = (program[*program_pos] % 10000) - (program[*program_pos] % 1000) == 1000;
        //let param_mode_immediate_c: bool = (program[*program_pos] % 100000) - (program[*program_pos] % 10000) == 10000;

        let state_to_check_a = if param_mode_immediate_a { program[*program_pos+1] } else { program[program[*program_pos+1] as usize] };
        let state_to_check_b = if param_mode_immediate_b { program[*program_pos+2] } else { program[program[*program_pos+2] as usize] };

        let store_pos = program[*program_pos+3];

        if state_to_check_a == state_to_check_b {
            program[store_pos as usize] = 1;
        } else {
            program[store_pos as usize] = 0;
        }
        *program_pos += 4;
    }


    let program_pos: &mut usize = &mut 0;

    loop {
        //println!("DEBUG - Pos: {}", *program_pos);
        let opcode = program[*program_pos] % 100;

        match opcode {
            1 => opcode_add(program, program_pos),
            2 => opcode_multiply(program, program_pos),
            3 => opcode_input(program, program_pos),
            4 => opcode_output(program, program_pos),
            5 => opcode_jump_if_true(program, program_pos),
            6 => opcode_jump_if_false(program, program_pos),
            7 => opcode_check_if_less(program, program_pos),
            8 => opcode_check_if_equal(program, program_pos),
            99 => return true,
            _ => return false,
        }
    }
}



//NOTE: This is also the second, as I don't need to extend anything here. Functions FTW!
#[allow(dead_code)]
fn aoc_day_five_first() {
    let input_raw = fs::read_to_string("input_files/input_5.txt").expect("File read went poop-shaped");
    let input_data:Vec<&str> = input_raw.split(",").collect();

    let program: &mut Vec<i32> = &mut Vec::new();

    //let mut counter = 0;
    for value in input_data {
        let value_int = value.parse::<i32>();
        match value_int {
            Ok(v) => program.push(v),
            Err(_e) => print_message_and_die("Parsing input value failed"),
        }
        //counter += 1;
    }



    let result = intcode_computer(program);


    println!("Did it work: {}", result);
}