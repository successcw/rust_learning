fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                let temp = arr[j + 1];
                arr[j + 1] = arr[j];
                arr[j] = temp;
            }
        }
    }
}
fn main() {
    let mut arr = [3, 2, 1, 4];
    println!("before sort {:?}", arr);
    bubble_sort(&mut arr);
    println!("after sort {:?}", arr);

}
