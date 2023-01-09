// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    // turn rows into columns
    let mut transposed_matrix :[[i32; 3]; 3] = matrix.clone();
    for i in 0..3 {
        for (pos, val) in (0..3).enumerate() {
            transposed_matrix[pos][i] = matrix[i][pos];
        }
    }
    transposed_matrix
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    println!("{} {} {}", matrix[0][0], matrix[0][1], matrix[0][2]);
    println!("{} {} {}", matrix[1][0], matrix[1][1], matrix[1][2]);
    println!("{} {} {}", matrix[2][0], matrix[2][1], matrix[2][2]);
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}