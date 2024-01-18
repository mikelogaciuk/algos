fn binary_search(list: Vec<i32>, target: i32) -> Option<i32> {
    let mut lower = 0 as usize;
    let mut upper = list.len() - 1;

    while lower <= upper {
        let mid = (lower + upper) / 2;
        let value = list[mid];

        if value == target {
            return Some(value);
        } else if target < value {
            upper = mid - 1;
        } else {
            lower = mid + 1;
        }
    }

    None
}

fn main() {
    let list: Vec<i32> = (1..100).collect();
    let target: i32 = 50;

    let result = binary_search(list, target);

    match result {
        Some(value) => println!("Found value: {}", value),
        None => println!("Value not found"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let list: Vec<i32> = (666..50000).collect();
        let target: i32 = 666;

        let result = binary_search(list, target);

        assert_eq!(result.unwrap(), target);
    }
}
