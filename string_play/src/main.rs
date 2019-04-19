mod hash_map;

fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    let s = format!("{}-{}", s1, s2);
    let len = s.len();
    println!("{}, {} chars", s, len);

    let hello = "Ohio, gozaimasu";
    let _s = &hello[0..4];

    // hash_map::show();
    // hash_map::update_value();
    hash_map::integer_play();
}
