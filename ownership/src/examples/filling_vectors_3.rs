pub fn example() {
    let mut vec0 = Vec::new();

    fill_vec(&mut vec0);
    vec0.push(88);

    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
}

fn fill_vec(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}
