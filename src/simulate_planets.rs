#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct Vector3 {
    x: isize,
    y: isize,
    z: isize,
}

#[aoc_generator(day12)]
pub fn parse_input(input: &str) -> Vec<State> {
    input
        .lines()
        .map(|line| {
            let x_start = line.find("x").unwrap();
            let y_start = line.find("y").unwrap();
            let z_start = line.find("z").unwrap();

            let x_comma = line[x_start..].find(",").unwrap();
            let y_comma = line[y_start..].find(",").unwrap();

            let x = line[(x_start + 2)..=x_comma].parse().unwrap();
            let y = line[(y_start + 2)..(y_start + y_comma)].parse().unwrap();
            let z = line[(z_start + 2)..(line.len() - 1)].parse().unwrap();
            Vector3 { x, y, z }
        })
        .map(|position| State {
            position,
            velocity: Vector3 { x: 0, y: 0, z: 0 },
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct State {
    position: Vector3,
    velocity: Vector3,
}

impl State {
    fn kinetic(&self) -> usize {
        (self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs()) as usize
    }
    fn potential(&self) -> usize {
        (self.position.x.abs() + self.position.y.abs() + self.position.z.abs()) as usize
    }
    fn energy(&self) -> usize {
        self.kinetic() * self.potential()
    }
}

fn delta_coord(x: isize, xp: isize) -> (isize, isize) {
    if x == xp {
        (0, 0)
    } else if x < xp {
        (1, -1)
    } else {
        (-1, 1)
    }
}

fn update_states(states: &mut [State]) {
    for i in 0..states.len() {
        for j in (i + 1)..states.len() {
            let (dx, dxp) = delta_coord(states[i].position.x, states[j].position.x);
            let (dy, dyp) = delta_coord(states[i].position.y, states[j].position.y);
            let (dz, dzp) = delta_coord(states[i].position.z, states[j].position.z);
            states[i].velocity.x += dx;
            states[i].velocity.y += dy;
            states[i].velocity.z += dz;

            states[j].velocity.x += dxp;
            states[j].velocity.y += dyp;
            states[j].velocity.z += dzp;
        }
    }
    for state in states {
        state.position.x += state.velocity.x;
        state.position.y += state.velocity.y;
        state.position.z += state.velocity.z;
    }
}

fn total_energy(state: &[State]) -> usize {
    state.iter().map(|s| s.energy()).sum()
}

#[aoc(day12, part1)]
pub fn disp(positions: &[State]) -> usize {
    let mut state = Vec::from(positions);
    for _ in 0..1000 {
        update_states(&mut state);
    }
    total_energy(&state)
}

fn update_sequence(
    seq: &mut [Vec<Vec<isize>>],
    states: &[State],
    seq_length: &[Vec<Option<usize>>],
) {
    for ((s, seq), len) in states.iter().zip(seq.iter_mut()).zip(seq_length.iter()) {
        if len[0].is_none() {
            seq[0].push(s.position.x);
        }
        if len[1].is_none() {
            seq[1].push(s.position.y);
        }
        if len[2].is_none() {
            seq[2].push(s.position.z);
        }
    }
}

fn check_seqs(seq: &[Vec<Vec<isize>>], seq_length: &mut [Vec<Option<usize>>]) {
    for (seqs, lens) in seq.iter().zip(seq_length.iter_mut()) {
        for (seq, len) in seqs
            .iter()
            .zip(lens.iter_mut())
            .filter(|(_, len)| len.is_none())
        {
            let half_length = seq.len() / 2;
            if seq[..half_length] == seq[half_length..] {
                *len = Some(half_length);
            }
        }
    }
}

fn has_all_seqs(seq_length: &[Vec<Option<usize>>]) -> bool {
    seq_length.iter().all(|p| p.iter().all(|c| c.is_some()))
}

use num::Integer;

fn lcm(numbers: &[usize]) -> usize {
    if numbers.len() == 1 {
        numbers[0]
    } else if numbers.len() == 2 {
        numbers[0].lcm(&numbers[1])
    } else {
        numbers[0].lcm(&lcm(&numbers[1..]))
    }
}

#[aoc(day12, part2)]
pub fn find_repeat(positions: &[State]) -> usize {
    let mut sequence = Vec::new();
    let mut state = Vec::from(positions);
    sequence.resize_with(state.len(), || {
        let mut coords = Vec::new();
        for _ in 0..3 {
            coords.push(vec![])
        }
        coords
    });
    let mut seq_length = Vec::new();
    seq_length.resize_with(sequence.len(), || {
        let mut le = Vec::new();
        le.resize(3, None);
        le
    });
    update_sequence(&mut sequence, &state, &mut seq_length);
    update_states(&mut state);
    update_sequence(&mut sequence, &state, &mut seq_length);

    // Let's create sequences of two at the start, seems better
    update_states(&mut state);
    update_sequence(&mut sequence, &state, &seq_length);
    update_states(&mut state);
    update_sequence(&mut sequence, &state, &seq_length);

    while !has_all_seqs(&seq_length) {
        update_states(&mut state);
        update_sequence(&mut sequence, &state, &seq_length);
        update_states(&mut state);
        update_sequence(&mut sequence, &state, &seq_length);

        check_seqs(&sequence, &mut seq_length);
    }
    let flat_seq: Vec<_> = seq_length
        .into_iter()
        .map(|p| p.into_iter().map(|c| c.unwrap()))
        .flatten()
        .collect();
    lcm(&flat_seq)
}
