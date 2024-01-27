//TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transpose = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    for i in 0..=2{
        for j in 0..=2{
            transpose[i][j] = matrix[j][i]
        }
    }
    transpose
}

fn main() {
    let matrix = [
    [101, 102, 103], // <-- the comment makes rustfmt add a newline
    [201, 202, 203],
    [301, 302, 303],
    ];
    println!("matrix: {:#?}", matrix);
    let transposed = transpose(matrix);
    println!("transposed: {:#?}", transposed);
}
