fn main() {
    let input: Vec<usize> = vec![1, 2, 3];
    let size: usize = 2;
    chunks(input, size);
}

fn chunks(v: Vec<usize>, size: usize) -> Vec<Vec<usize>> {
    let mut res_vec: Vec<Vec<usize>> = Vec::new();

    for num in v.iter() {
        let mut curr_sub_vec = res_vec.last_mut();

        match curr_sub_vec {
            None => res_vec.push(vec![*num]),
            Some(n) => {
                if n.len() == size {
                    res_vec.push(vec![*num])
                } else {
                    n.push(*num)
                }
            }
        }
    }

    // println!("res_vec: {:?}", res_vec);
    res_vec
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_output() {
        let input: Vec<usize> = vec![1, 2, 3];
        let expected: Vec<Vec<usize>> = vec![vec![1, 2], vec![3]];
        assert_eq!(chunks(input, 2), expected);

        let input: Vec<usize> = vec![1, 2, 3, 4];
        let size = 5_usize;
        let expected: Vec<Vec<usize>> = vec![vec![1, 2, 3, 4]];
        assert_eq!(chunks(input, size), expected);
    }
}
