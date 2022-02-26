fn main() {
    proconio::input! {
        k: i64,
    }

    let ans = (0..63).into_iter()
                     .map(|x| k & (1 << x) != 0)
                     .map(|x| if x {'2'} else {'0'})
                     .rev()
                     .skip_while(|&x| x == '0')
                     .collect::<String>();

    println!("{}", ans)
}
