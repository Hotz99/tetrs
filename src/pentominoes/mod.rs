pub type Shape = Vec<Vec<u8>>;
pub type Permutations = Vec<Vec<Shape>>;

fn print_mutations(permutations: &Permutations) {
    for (pent_id, _) in permutations.iter().enumerate() {
        println!("{}", permutations[pent_id].len());

        for permutation in 0..permutations[pent_id].len() {
            println!("ID: {}", pent_id);
            println!("permutation: {}", permutation);

            for y in 0..permutations[pent_id][permutation][0].len() {
                for x in 0..permutations[pent_id][permutation].len() {
                    print!("{}", permutations[pent_id][permutation][x][y]);
                }
                println!();
            }
            println!();
        }
    }
}

pub fn char_to_id(c: char) -> u8 {
    match c {
        'X' => 0,
        'I' => 1,
        'Z' => 2,
        'T' => 3,
        'U' => 4,
        'V' => 5,
        'W' => 6,
        'Y' => 7,
        'L' => 8,
        'P' => 9,
        'N' => 10,
        'F' => 11,
        _ => 255,
    }
}

// returns 4D vec:
// 1st D: pentomino ID, 2nd D: permutation ID, 3rd D: permutation as 2d vec
pub fn load_permutations() -> Permutations {
    let mut pentominoes = Vec::<Vec<Vec<Vec<u8>>>>::default();

    // small csv file, hence small allocation
    // loaded at compile time, hence no runtime file-reading overhead
    let csv = include_str!("pentomino_db.csv");

    for line in csv.lines() {
        let mutation_data: Vec<u8> = line.split(',').map(|s| s.parse().unwrap()).collect();

        let pent_id = mutation_data[0] as usize;
        let permutation = mutation_data[1] as usize;
        let x_size = mutation_data[2] as usize;
        let y_size = mutation_data[3] as usize;

        let mut piece = vec![vec![0u8; y_size]; x_size];

        // 1d array to 2d array
        for i in 0..(x_size * y_size) {
            piece[i / y_size][i % y_size] = mutation_data[4 + i];
        }

        if pent_id >= pentominoes.len() {
            pentominoes.resize_with(pent_id + 1, Vec::new);
        }

        if permutation >= pentominoes[pent_id].len() {
            pentominoes[pent_id].resize_with(permutation + 1, Vec::new);
        }

        pentominoes[pent_id][permutation] = piece;
    }

    pentominoes
}
