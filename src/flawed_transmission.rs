#[aoc_generator(day16)]
fn get_array(input: &str) -> Vec<i64> {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect()
}

fn calulate_row(cumulative_sum: &[i64], row_index: usize) -> i64 {
    let mut sum: i64 = 0;
    let mut index = row_index - 1;
    while index < cumulative_sum.len() {
        let max_index = std::cmp::min(cumulative_sum.len() - 1, index + row_index - 1);
        sum += cumulative_sum[max_index] - cumulative_sum.get(index - 1).unwrap_or(&0);
        index += row_index + row_index;
        if index >= cumulative_sum.len() {
            break
        }
        let max_index = std::cmp::min(cumulative_sum.len() - 1, index + row_index - 1);
        sum -= cumulative_sum[max_index] - cumulative_sum.get(index - 1).unwrap_or(&0);
        index += row_index + row_index;
    }
    sum.abs() % 10
}

fn calculate_fft(mut input: Vec<i64>, count: usize, log: bool) -> Vec<i64> {
    let mut cumulative_sum = vec![0; input.len()];
    let mut output = vec![0; input.len()];
    for i in 0..count {
        if log {
            eprintln!("Iteration {}", i);
        }
        let mut sum = 0;
        for i in 0..input.len() {
            sum += input[i];
            cumulative_sum[i] = sum;
        }
        output.iter_mut().enumerate().for_each(|(row_index, out)| {
//          if log && row_index % 1000 == 0 {
//              eprintln!("Row {}", row_index);
//          }
            *out = calulate_row(&cumulative_sum, row_index + 1);
        });
        std::mem::swap(&mut output, &mut input);
    }
    input
}

#[aoc(day16, part1)]
fn part1(input: &[i64]) -> usize {
    let input = Vec::from(input);
    //let input = vec![1,2,3,4,5,6,7,8];
    let output = calculate_fft(input, 100, false);
    //dbg!(&output);

    let mut result = 0;
    for &d in output.iter().skip(0).take(8) {
        result = result * 10 + (d as usize);
    }
    result
}
#[aoc(day16, part2)]
fn part2(input: &[i64]) -> usize {
    let input_vec = input.repeat(10_000);
    let output = calculate_fft(input_vec, 100, true);

    let mut offset = 0;
    for i in &input[0..7] {
        offset = offset * 10 + *i as usize;
    }

    let mut result = 0;
    for &d in output.iter().skip(offset).take(8) {
        result = result * 10 + (d as usize);
    }
    result
}
