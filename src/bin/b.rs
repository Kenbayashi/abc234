fn main() {
    proconio::input! {
        n: usize,
        a: [(i32, i32); n],
    }

    let mut ans: i32 = 0;
    let mut points = a.into_iter().collect::<Vec<(i32, i32)>>();

    while let Some((t_x, t_y)) = points.pop() {
        ans = i32::max(ans, points.iter().fold(0, |acc, &(x, y)| i32::max(acc, (t_x - x).pow(2u32) + (t_y - y).pow(2u32))));
    }

    println!("{}", f32::sqrt(ans as f32));
}
