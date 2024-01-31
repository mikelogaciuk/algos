use rand::Rng;


fn main() {
    println!("Hello, world!");
}

fn make_unsorted_arr(n: i16) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let arr: Vec<i32> = (0..n).map(|_| rng.gen_range(0..100000)).collect();

    return arr;
}

fn xd(unsorted_arr: Vec<i32>) -> Vec<i32> {
    let n: usize = unsorted_arr.len();

    loop {
        let mu
    }


}