pub fn dump(arr: &[i32]) {
    println!("slice is {:?}", arr);
}

pub fn play_vector() {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    let first = v[0];
    let maybe_first = v.get(0);

    let slice = &v[0..3];
    dump(&slice);
    println!("v is {:?}", v);
    println!("first is {}", first);
    println!("maybe_first is {:?}", maybe_first);
}
