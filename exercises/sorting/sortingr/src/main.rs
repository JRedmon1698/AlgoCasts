fn main() {
    let v: Vec<u8> = vec![10, 0, 2, 5, 1, 17, 9];
    let res = selection_sort(v);
    println!("sorted: {:?}", res);
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
