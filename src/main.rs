use assignment1::sorting::*;

fn main() {
    let mut arr = vec![3, 2, 1, 5, 4];
    quick_sort(&mut arr, |a, b| a < b);
    println!("{:?}", arr);

    let mut arr1 = vec![3, 2, 1, 5, 4];
    insertion_sort(&mut arr1, |a, b| a > b);
    println!("{:?}", arr1);

    let mut arr2 = vec![3, 2, 1, 5, 4];
    selection_sort(&mut arr2, |a, b| a < b);
    println!("{:?}", arr2);

    let mut arr3 = vec![3, 2, 1, 5, 4];
    merge_sort(&mut arr3, |a, b| a > b);
    println!("{:?}", arr3);
}