fn main() {
    proconio::input! {
        t: i32,
    }

    let ans = func(func(func(t) + t) + func(func(t)));

    println!("{}", ans);
}

fn func(t: i32) -> i32 {
    t * t + 2 * t + 3
}
