use proconio::{fastout, input};

#[fastout()]
fn main() {
    input! {
        n:usize,lr:[(usize,usize);n]
    }
    // 各値の個数の期待値
    let mut num_exp = vec![0f64; 101];
    // 各値以下の数の個数の期待値
    let mut num_exp_sum = vec![0f64; 101];

    let mut ans = 0f64;
    for i in 0..n - 1 {
        let (l0, r0) = lr[i];
        let rate0 = 1f64 / (r0 - l0 + 1) as f64;
        for j in l0..=r0 {
            num_exp[j] += rate0;
        }
        for j in 1..=100 {
            num_exp_sum[j] = num_exp_sum[j - 1] + num_exp[j];
        }

        let (l1, r1) = lr[i + 1];
        // a[i+1]を選ぶ
        let rate1 = 1f64 / (r1 - l1 + 1) as f64;
        // 事前にi+1個の値がある。
        for j in l1..=r1 {
            ans += rate1 * ((i + 1) as f64 - num_exp_sum[j]);
        }
    }

    println!("{}", ans);
}
