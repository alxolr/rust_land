mod sound;

fn main() {
    println!("Hello, world!");

    // absolute path
    crate::sound::instrument::clarinet();

    // relative path
    sound::instrument::clarinet();
}
