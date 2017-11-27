// array-exercices.rs
fn main() {
    let awesome_array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice_1 = &awesome_array[1..9];
    let mut sum = 0;
    for num in awesome_array.iter() {
        sum = sum + num
    }
    println!("Array sum is : {}", sum);
    sum = 0;
    for num in slice_1.iter() {
        sum = sum + num
    }
    println!("Slice sum is: {}", sum);
}
