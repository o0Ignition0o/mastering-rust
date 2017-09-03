// first-program.rs
fn main() {
    let target_inferred = "inferred world";
    let target: &'static str = "non-inferred world";

    println!("Hi there, {}", target_inferred);
    println!("Hi there, {}", target);
}
