use rand::Rng;

fn find_smallest(arr: &[i32]) -> usize {
    let mut smallest = arr[0];
    let mut smallest_idx = 0;

    for (idx, &item) in arr.iter().enumerate() {
        if item < smallest {
            smallest = item;
            smallest_idx = idx;
        }
    }

    return smallest_idx;
}

fn make_unsorted_arr(n: i16) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let arr: Vec<i32> = (0..n).map(|_| rng.gen_range(0..100000)).collect();

    return arr;
}

fn selection_sort(mut array: Vec<i32>) -> Vec<i32> {
    let mut sorted_arr: Vec<i32> = Vec::new();

    while !array.is_empty() {
        let smallest = find_smallest(&array);
        sorted_arr.push(array.remove(smallest));
    }

    return sorted_arr;
}

fn main() {
    let arr = make_unsorted_arr(50);
    let sorted_arr = selection_sort(arr.clone());

    println!(
        "Unsorted array is: {:?} \n\n and sorted array: {:?}",
        arr, sorted_arr
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_smallest() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let smallest_idx = find_smallest(&arr);

        assert_eq!(smallest_idx, 0);
    }

    #[test]
    fn test_selection_sort() {
        let arr = make_unsorted_arr(666);
        let sorted_arr = selection_sort(arr.clone());

        assert_eq!(sorted_arr.len(), arr.len());
    }

    #[test]
    fn test_make_unsorted_arr() {
        let arr = make_unsorted_arr(333);

        assert_eq!(arr.len(), 333);
    }

    #[test]
    fn test_sort_custom_arr() {
        let arr = vec![6, 5, 2, 1, 4, 9];
        let should_be = vec![1, 2, 4, 5, 6, 9];
        let sorted_arr = selection_sort(arr.clone());

        assert_eq!(sorted_arr, should_be);
    }
}
