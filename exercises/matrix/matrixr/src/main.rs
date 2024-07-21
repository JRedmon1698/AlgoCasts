fn main() {
    let res = matrix(5);
    println!("{:?}", res);
}

fn matrix(n: usize) -> Vec<Vec<usize>> {
    let mut container: Vec<Vec<usize>> = vec![vec![0; n]; n];

    let mut counter: usize = 1;
    let mut top_row: usize = 0;
    let mut bottom_row: usize = n - 1;
    let mut left_col: usize = 0;
    let mut right_col: usize = n - 1;

    while counter <= n * n {
        for i in left_col..=right_col {
            container[top_row][i] = counter;
            counter += 1;
        }
        top_row += 1;

        for i in top_row..=bottom_row {
            container[i][right_col] = counter;
            counter += 1;
        }
        right_col -= 1;

        for i in (left_col..=right_col).rev() {
            container[bottom_row][i] = counter;
            counter += 1;
        }
        bottom_row -= 1;

        for i in (top_row..=bottom_row).rev() {
            container[i][left_col] = counter;
            counter += 1;
        }
        left_col += 1;
    }

    container
}
