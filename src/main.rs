
mod bubble_sort;
fn main() {
    //1. bubble sort
    let mut arr = [3, 2, 1, 4];
    println!("before sort {:?}", arr);
    bubble_sort::bubble_sort(&mut arr);
    println!("after sort {:?}", arr);

}
