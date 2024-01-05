// Each line in the CSV file uses 4+ values to define one permutation of a pentomino.
// - First value is the ID for a pentomino, from 0 to 11.
// - Second value is the index of the permutation (rotation, flip, etc.), between 0 to 7.
// - Third and fourth values are the X and Y sizes respectively.
// - The remaining values define an X*Y matrix, displaying the shape of the pentomino.

// This file does not contain a header.
// The pentominoes should be sorted by ID in increasing order

// EXAMPLE:

// 2,1,3,3,1,0,0,1,1,1,0,0,1

// ID: 2
// Permutation: 1
// X: 3 squares
// Y: 3 squares
// Shape:
// X 0 0
// X X X
// 0 0 X

pub struct PentominoDB {
    pub data: Vec<Vec<Vec<Vec<u8>>>>,
}

impl PentominoDB {
    pub fn new() -> PentominoDB {
        let data = PentominoDB::load_pentominoes();

        // Self::print_data(&data);

        PentominoDB { data }
    }

    fn print_data(data: &Vec<Vec<Vec<Vec<u8>>>>) {
        for id in 0..data.len() {
            for permutation in 0..data[id].len() {
                println!("ID: {}", id);
                println!("Permutation: {}", permutation);

                for y in 0..data[id][permutation][0].len() {
                    for x in 0..data[id][permutation].len() {
                        print!("{}", data[id][permutation][x][y]);
                    }
                    println!();
                }
                println!();
            }
        }
    }

    pub fn load_pentominoes() -> Vec<Vec<Vec<Vec<u8>>>> {
        let mut dynamic_list: Vec<Vec<Vec<Vec<u8>>>> = Vec::new();

        // 'include_str!':
        // small csv file, so no memory overhead
        // loaded at compile time, so no runtime file-reading overhead
        let data = include_str!("pentomino_db.csv");

        for line in data.lines() {
            let values: Vec<u8> = line.split(',').map(|s| s.parse().unwrap()).collect();

            let id = values[0] as usize;
            let x_size = values[2] as usize;
            let y_size = values[3] as usize;

            while id >= dynamic_list.len() {
                dynamic_list.push(Vec::new());
            }

            // '0u8':
            // suffixing '0' with 'u8' tells the vec macro to use 'u8' for the vec values'
            let mut piece = vec![vec![0u8; y_size]; x_size];

            // 1d array to 2d array
            for i in 0..(x_size * y_size) {
                piece[i / y_size][i % y_size] = values[4 + i];
            }

            dynamic_list[id].push(piece);
        }

        dynamic_list
    }
}
