#![allow(
    non_snake_case,
    unused_variables,
    unused_assignments,
    unused_mut,
    unused_imports,
    unused_macros,
    dead_code
)]
use num_complex::Complex;
use proconio::{fastout, input, marker::*};
use std::cmp::*;
use std::collections::*;
//use num_traits::*;
//use std::mem::swap;
//use std::num;
//use petgraph::unionfind::UnionFind;
//use permutohedron::LexicalPermutation;
macro_rules! debug {
($($a:expr),* $(,)*) => {
    #[cfg(debug_assertions)]
    eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
};
}
const MOD: i64 = 998244353;
const PI: f64 = std::f64::consts::PI;

#[fastout()]
fn main() {
    input! {
        r:usize,g:usize,b:usize,k:usize,x:usize,y:usize,z:usize
    }
    // a>=k-y, b>=k-z, c>=k-x
    // a: k-y <= a <= r, rの選び方はchoose(r,a)
    let mut maxrgb = max(r, max(g, b));
    let mut fact = vec![1; maxrgb + 1];
    let mut ifact = vec![1; maxrgb + 1];
    for i in 2..maxrgb + 1 {
        fact[i] = fact[i - 1] * i as i64 % MOD;
        ifact[i] = modInv(fact[i] as i64);
    }
    let mut choose = |a: usize, b: usize| fact[a] * ifact[a - b] % MOD * ifact[b] % MOD;

    let mut ans = 0;
    for i in k - y..=min(r, min(x, z)) {
        let mut red = choose(r, i);
        for j in k - z..=min(g, min(x - i, y)) {
            let l = k - i - j;
            if l <= b && l + j <= y && i + l <= z {
                let green = choose(g, j);
                let blue = choose(b, l);
                ans += red * green % MOD * blue % MOD;
                ans %= MOD;
            }
        }
    }
    println!("{}", ans);
}

fn modPow(a: i64, b: i64) -> i64 {
    if b == 0 {
        1
    } else {
        if b % 2 == 0 {
            modPow(a * a % MOD, b / 2)
        } else {
            a * modPow(a, b - 1) % MOD
        }
    }
}

fn modInv(a: i64) -> i64 {
    modPow(a, MOD - 2) % MOD
}
