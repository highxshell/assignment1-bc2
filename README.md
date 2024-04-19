## Rust Sorting Library: A Collection of Sorting Algorithms

# Usage: The sorting library provides a set of generic sorting algorithms that can be used across different projects and domains without needing to rewrite the sorting logic each time.

![bc1](https://github.com/highxshell/assignment1-bc2/assets/121538758/12739f37-ad42-4234-a7f5-10ee37d63ca3)

![bc2](https://github.com/highxshell/assignment1-bc2/assets/121538758/6bd0fd6b-df45-485a-9b8d-d7954d2e438d)

# Examples: 

```bash
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
```

```bash
[1, 2, 3, 4, 5]
[5, 4, 3, 2, 1]
[1, 2, 3, 4, 5]
[5, 4, 3, 2, 1]
```
