fn main() {
    let mut v: Vec<u8> = vec![10, 0, 2, 5, 1, 17, 9];
    let res = bubble_sort(v.as_mut());
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
