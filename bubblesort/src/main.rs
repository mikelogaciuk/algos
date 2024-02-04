use rand::Rng;

fn main() {
    let swamp = make_unsorted_arr(16);

    let sorted: Vec<i32> = bubble_sort(swamp);

    println!("Sorted array: {:?}", sorted);
}

fn make_unsorted_arr(n: i16) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let arr: Vec<i32> = (0..n).map(|_| rng.gen_range(0..100000)).collect();

    return arr;
}

fn bubble_sort(arr: Vec<i32>) -> Vec<i32> {
    let mut sorted_arr: Vec<i32> = arr.clone();
    let mut swapped = true;

    while swapped {
        swapped = false;
        for i in 0..sorted_arr.len() - 1 {
            if sorted_arr[i] > sorted_arr[i + 1] {
                sorted_arr.swap(i, i + 1);
                swapped = true;
            }
        }
    }

    return sorted_arr;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_unsorted_arr() {
        let arr = make_unsorted_arr(16);
        assert_eq!(arr.len(), 16);
    }

    #[test]
    fn test_bubble_sort() {
        let arr = vec![6, 8, 14, 5, 1, 2, 4, 99, 16, 6];
        let sorted_arr = bubble_sort(arr);
        assert_eq!(sorted_arr, vec![1, 2, 4, 5, 6, 6, 8, 14, 16, 99]);
    }
}
