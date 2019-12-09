#[aoc_generator(day2)]
pub fn get_intcode(input: &str) -> Vec<u64> {
    input.split(",").map(|u| u.parse().unwrap()).collect()
}

fn run_intcode(code: &mut [u64]) -> Result<u64, String> {
    let mut position = 0;
    while position < code.len() {
        match code[position] {
            1 => {
                let source_a = code[position + 1] as usize;
                let source_b = code[position + 2] as usize;
                let dest = code[position + 3] as usize;
                code[dest] = code[source_a] + code[source_b];
            }
            2 => {
                let source_a = code[position + 1] as usize;
                let source_b = code[position + 2] as usize;
                let dest = code[position + 3] as usize;
                code[dest] = code[source_a] * code[source_b];
            }
            99 => return Ok(code[0]),
            i => return Err(format!("invalid opcode: {}", i)),
        }
        position += 4;
    }
    return Err("did not land on 99".to_owned());
}

#[aoc(day2, part1)]
pub fn execute_intcode(code: &[u64]) -> Result<u64, String> {
    let mut code = Vec::from(code);
    code[1] = 12;
    code[2] = 2;
    run_intcode(&mut code)
}

#[aoc(day2, part2)]
pub fn find_good_code(initial_memory: &[u64]) -> Result<u64, String> {
    let target_value = 19690720;
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut code = Vec::from(initial_memory);
            code[1] = noun;
            code[2] = verb;
            if run_intcode(&mut code)? == target_value {
                return Ok(100 * noun + verb);
            }
        }
    }
    Err("No pair found".to_owned())
}

