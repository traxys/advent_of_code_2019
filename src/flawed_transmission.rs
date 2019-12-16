struct Pattern<'a> {
    pattern: &'a [i64],
    index: usize,
    repeat: usize,
    remain_in_repeat: usize,
}

impl<'a> Pattern<'a> {
    fn new(pattern: &'a [i64], repeat: usize) -> Self {
        Self {
            pattern,
            remain_in_repeat: repeat,
            index: 0,
            repeat,
        }
    }
}

impl<'a> Iterator for Pattern<'a> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remain_in_repeat == 0 {
            self.index = (self.index + 1) % self.pattern.len();
            self.remain_in_repeat = self.repeat;
        }
        self.remain_in_repeat -= 1;
        Some(self.pattern[self.index])
    }
}

#[aoc_generator(day16)]
fn get_array(input: &str) -> Vec<i64> {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect()
}

fn calculate_fft(mut input: nalgebra::DVector<i64>, count: usize, log: bool) -> nalgebra::DVector<i64> {
    let mut transform_matrix = nalgebra::DMatrix::from_iterator(
        input.len(),
        input.len(),
        (1..=input.len())
            .map(|i| Pattern::new(&[0, 1, 0, -1], i).skip(1).take(input.len()))
            .flatten(),
    );
    transform_matrix.transpose_mut();
    for i in 0..count {
        if log {
            println!("Iteration {}", i);
        }
        input = &transform_matrix * input;
        for elem in &mut input {
            *elem = elem.abs() % 10;
        }
    }
    input
}

#[aoc(day16, part1)]
fn part1(input: &[i64]) -> usize {
    let input = nalgebra::DVector::from_column_slice(input);

    let output = calculate_fft(input, 100, false);

    let mut result = 0;
    for &d in output.iter().skip(0).take(8) {
        result = result * 10 + (d as usize); 
    }
    result
}
#[aoc(day16, part2)]
fn part2(input: &[i64]) -> usize {
    let input_vec = nalgebra::DVector::from_vec(input.repeat(10_000));
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
