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

fn make_unsorted_arr() -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let arr: Vec<i32> = (0..50).map(|_| rng.gen_range(0..100000)).collect();

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
    let arr = make_unsorted_arr();
    let sorted_arr = selection_sort(arr.clone());

    println!(
        "Unsorted array is: {:?} \n\n and sorted array: {:?}",
        arr, sorted_arr
    );
}
