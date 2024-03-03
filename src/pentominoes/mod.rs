pub struct PentominoDb {
    pub data: Vec<Vec<Vec<Vec<u8>>>>,
}

impl PentominoDb {
    pub fn new() -> PentominoDb {
        let data = PentominoDb::load_pentominoes();

        // Self::print_mutations(&data);

        PentominoDb { data }
    }

    fn print_mutations(mutations: &Vec<Vec<Vec<Vec<u8>>>>) {
        for pent_id in 0..mutations.len() {
            println!("{}", mutations[pent_id].len());

            for mutation in 0..mutations[pent_id].len() {
                println!("ID: {}", pent_id);
                println!("Permutation: {}", mutation);

                for y in 0..mutations[pent_id][mutation][0].len() {
                    for x in 0..mutations[pent_id][mutation].len() {
                        print!("{}", mutations[pent_id][mutation][x][y]);
                    }
                    println!();
                }
                println!();
            }
        }
    }

    // returns 3D vec:
    // 1D: piece ID, 2D: mutation, 3D: 2D vec piece representation
    pub fn load_pentominoes() -> Vec<Vec<Vec<Vec<u8>>>> {
        let mut pentominoes = Vec::<Vec<Vec<Vec<u8>>>>::new();

        // 'include_str!':
        // small csv file, so no memory overhead
        // loaded at compile time, so no runtime file-reading overhead
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
}
