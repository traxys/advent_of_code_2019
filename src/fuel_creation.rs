use std::collections::HashMap;

type ProductName = arrayvec::ArrayString<[u8; 6]>;

// Gives output to how to make it
type Reactions = HashMap<ProductName, Reaction>;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Product {
    name: ProductName,
    amount: isize,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Reaction {
    input: Vec<Product>,
    output: Product,
}

fn split_product(product: &str) -> Product {
    let product = product.trim();
    let mut splitted = product.split(" ");
    let amount = splitted.next().unwrap().parse().unwrap();
    let name = splitted.next().unwrap().parse().unwrap();
    Product {
        name,
        amount,
    }
}

#[aoc_generator(day14)]
fn parse_reactions(input: &str) -> Reactions {
    input
        .lines()
        .map(|line| {
            let mut split_line = line.split("=>");
            let inputs = split_line.next().unwrap();
            let output = split_product(split_line.next().unwrap());
            (
                output.name,
                Reaction {
                    input: inputs.split(",").map(split_product).collect(),
                    output,
                },
            )
        })
        .collect()
}


#[aoc(day14, part1)]
fn find_how_much_ore_needed(input: &Reactions) -> usize {
    0
}
