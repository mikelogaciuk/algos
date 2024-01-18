/* Basic (like I would do that in Python) */
fn linear_search(list: &Vec<i32>, target: i32) -> Option<i32> {
    for i in 0..list.len() {
        if list[i] == target {
            return Some(list[i]);
        }
    }

    None
}

fn alternate_version(list: &Vec<i32>, target: i32) -> Option<i32> {
    let result = list.iter().find(|&&x| x == target);

    Some(*result?)
}

fn main() {
    let list: Vec<i32> = (1..100).collect();
    let target: i32 = 42;

    linear_search(&list, target);
    alternate_version(&list, target);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search() {
        let list: Vec<i32> = (666..50000).collect();
        let target: i32 = 666;

        let result = linear_search(&list, target);

        assert_eq!(result.unwrap(), target);
    }

    #[test]
    fn test_alternate_version() {
        let list: Vec<i32> = (666..50000).collect();
        let target: i32 = 666;

        let result = alternate_version(&list, target);

        assert_eq!(result.unwrap(), target);
    }
}
