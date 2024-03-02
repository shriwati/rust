use crate::matrix;

pub fn transpose(matrix: [ [i32;3];3]) -> [ [i32;3];3]{
    // read each row,
    let mut mat=[[0; 3];3];;
    for i  in 0..=2 {
        for j in 0..=2 {
            mat[i][j]=matrix[j][i];
        }
    }
    mat
}
#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = matrix::transpose(matrix);
    assert_eq!(
        transposed,
        [
            [101, 201, 301], //
            [102, 202, 302],
            [103, 203, 303],
        ]
    );
}