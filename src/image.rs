type Layer = Vec<Vec<u8>>;
use itertools::Itertools;

#[aoc_generator(day8)]
pub fn extract_layes(input: &str) -> Vec<Layer> {
    let length = 25;
    let height = 6;
    input
        .chars()
        .chunks(length * height)
        .into_iter()
        .map(|layer| {
            layer
                .chunks(length)
                .into_iter()
                .map(|line| line.map(|d| d.to_digit(10).unwrap() as u8).collect())
                .collect()
        })
        .collect()
}
fn count_digits_in_layer(layer: &Layer) -> (usize, usize, usize) {
    layer
        .iter()
        .map(|line| {
            line.iter()
                .fold((0, 0, 0), |(zeros, ones, twos), digit| match digit {
                    0 => (zeros + 1, ones, twos),
                    1 => (zeros, ones + 1, twos),
                    2 => (zeros, ones, twos + 1),
                    _ => unreachable!(),
                })
        })
        .fold((0, 0, 0), |total, layer| {
            (total.0 + layer.0, total.1 + layer.1, total.2 + layer.2)
        })
}
#[aoc(day8, part1)]
pub fn check_layers_for_error(input: &Vec<Layer>) -> u64 {
    input
        .iter()
        .map(|layer| count_digits_in_layer(layer))
        .fold(
            (std::usize::MAX, 0),
            |(min_zero, current_out), (zeros, ones, twos)| {
                if zeros < min_zero {
                    (zeros, ones * twos)
                } else {
                    (min_zero, current_out)
                }
            },
        )
        .1 as u64
}

fn create_image(encoded: &Vec<Layer>) -> Layer {
    let mut current_image = vec![vec![2; 25]; 6];
    let mut remain = 25 * 6;
    for layer in encoded {
        if remain == 0 {
            break;
        }
        for (i, line) in layer.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if current_image[i][j] == 2 {
                    current_image[i][j] = *c;
                    remain -= (*c != 2) as u16;
                }
            }
        }
    }
    current_image
}
#[aoc(day8, part2)]
pub fn find_password_in_image(input: &Vec<Layer>) -> String {
    let image = create_image(input);
    format!(
        "\n{}",
        image
            .iter()
            .map(|line| line
                .iter()
                .map(|d| match d {
                    0 => yansi::Paint::default(" "),
                    1 => yansi::Paint::new(" ").bg(yansi::Color::White),
                    _ => unreachable!(),
                })
                .format(""))
            .format("\n")
    )
}
