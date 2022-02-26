fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        ps: [i32; n],
    }

    for num in k..=n {
        let mut array = ps.iter().take(num).collect::<Vec<&i32>>();
        array.sort_by_key(|&x| std::cmp::Reverse(x));
        println!("{}", array[k - 1]);
    }
}
