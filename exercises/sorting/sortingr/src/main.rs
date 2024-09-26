fn main() {
    let mut v: Vec<u8> = vec![10, 0, 2, 5, 1, 17, 9];
    let res = merge_sort(&mut v);
    println!("sorted: {:?}", res);
    // let v: Vec<u8> = Vec::new();
    // println!("{:?}", v.len());
}

fn bubble_sort(v: &mut Vec<u8>) -> &Vec<u8> {
    for i in 0..v.len() - 1 {
        for k in 0..v.len() - i - 1 {
            if v[k] > v[k + 1] {
                v.swap(k, k + 1);
            }
        }
    }

    v
}

fn selection_sort(mut v: Vec<u8>) -> Vec<u8> {
    for i in 0..v.len() {
        let mut idx_of_min = i;
        for k in i + 1..v.len() {
            if v[k] < v[idx_of_min] {
                idx_of_min = k;
            }
        }
        if idx_of_min != i {
            v.swap(idx_of_min, i);
        }
    }

    v
}

fn merge_sort(v: &mut Vec<u8>) -> Vec<u8> {
    if v.len() == 1 {
        return v.to_owned();
    }

    let mid = v.len() / 2;
    let (left, right) = v.split_at(mid);

    merge(
        merge_sort(left.to_vec().as_mut()),
        merge_sort(right.to_vec().as_mut()),
    )
}

fn merge(mut left: Vec<u8>, mut right: Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    while (left.len() > 0) && (right.len() > 0) {
        if left[0] < right[0] {
            result.push(left.remove(0));
        } else {
            result.push(right.remove(0));
        }
    }
    result.extend(right);
    result.extend(left);

    result
}
