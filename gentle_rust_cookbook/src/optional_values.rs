pub fn get_optional() {
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;
    let first = slice.get(0);
    let last = slice.get(5);

    println!("first {:?}", first);
    println!("last {:?}", last);

    println!("first {} {}", first.is_some(), first.is_none());
    println!("last {} {} ", last.is_some(), last.is_none());
    println!("first value {}", first.unwrap());

    let last = *slice.get(5).unwrap_or(&-1);

    println!("last value {}", last);
}
