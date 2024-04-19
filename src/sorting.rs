pub fn quick_sort<T, F>(arr: &mut [T], compare: F)
where
    F: Fn(&T, &T) -> bool,
    T: Clone,
{
    if arr.len() <= 1 {
        return;
    }
    let mut stack = vec![(0, arr.len() - 1)];
    while let Some((mut left, mut right)) = stack.pop() {
        if left >= right {
            continue;
        }
        let pivot = arr[left].clone();
        while left <= right {
            while left <= right && compare(&arr[left], &pivot) {
                left = left.checked_add(1).unwrap_or_else(|| panic!("Overflow when incrementing left index"));
            }
            while right > 0 && left <= right && compare(&pivot, &arr[right]) {
                right = right.checked_sub(1).unwrap_or_else(|| panic!("Overflow when decrementing right index"));
            }
            if left <= right {
                arr.swap(left, right);
                left = left.checked_add(1).unwrap_or_else(|| panic!("Overflow when incrementing left index"));
                if right > 0 {
                    right = right.checked_sub(1).unwrap_or_else(|| panic!("Overflow when decrementing right index"));
                } else {
                    break;
                }
            }
        }
        stack.push((stack.len(), right));
        stack.push((left, arr.len() - 1));
    }
}

pub fn selection_sort<T, F>(arr: &mut [T], compare: F)
where
    F: Fn(&T, &T) -> bool,
{
    for i in 0..arr.len() {
        let mut min_index = i;
        for j in (i + 1)..arr.len() {
            if compare(&arr[j], &arr[min_index]) {
                min_index = j;
            }
        }
        if i != min_index {
            arr.swap(i, min_index);
        }
    }
}

pub fn insertion_sort<T, F>(arr: &mut [T], compare: F)
where
    F: Fn(&T, &T) -> bool,
{
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && compare(&arr[j], &arr[j - 1]) {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

pub fn merge_sort<T, F>(arr: &mut [T], compare: F)
where
    T: Clone,
    F: Fn(&T, &T) -> bool,
{
    if arr.len() <= 1 {
        return;
    }

    let mut width = 1;
    let mut merged = Vec::with_capacity(arr.len());

    while width < arr.len() {
        let mut i = 0;
        while i < arr.len() {
            let mid = i + width;
            if mid >= arr.len() {
                break;
            }
            let end = std::cmp::min(i + 2 * width, arr.len());
            let mut left = i;
            let mut right = mid;

            while left < mid && right < end {
                if compare(&arr[left], &arr[right]) {
                    merged.push(arr[left].clone());
                    left += 1;
                } else {
                    merged.push(arr[right].clone());
                    right += 1;
                }
            }
            merged.extend_from_slice(&arr[left..mid]);
            merged.extend_from_slice(&arr[right..end]);

            for (j, item) in merged.drain(..).enumerate() {
                arr[i + j] = item;
            }
            i += 2 * width;
        }
        width *= 2;
    }
}


