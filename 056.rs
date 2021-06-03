#![allow(
    non_snake_case,
    unused_variables,
    unused_assignments,
    unused_mut,
    unused_imports,
    unused_macros,
    dead_code
)]
use proconio::{fastout, input, marker::*};
use std::cmp::*;
use std::collections::*;
macro_rules! debug {
  ($($a:expr),* $(,)*) => {
      #[cfg(debug_assertions)]
      eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
  };
}

#[fastout()]
fn main() {
    input! {
        n:usize,s:i64,ab:[(i64,i64);n]
    }
    let mut x = 0;
    let mut c = vec![0; n];
    for i in 0..n {
        x += min(ab[i].0, ab[i].1);
        c[i] = (ab[i].0 - ab[i].1).abs();
    }
    if s < x {
        println!("Impossible");
        return;
    }
    // mp(k,v) 値がkになるとき、高額の福袋を選択した日のVec
    // kがs以下の時のみ追記するため、mpの長さは高々s+1、各vの長さは高々n
    let mut mp = BTreeMap::new();
    // 初期値：すべて安い方を選んだ時、総額x円。
    mp.insert(x, vec![]);
    'l: for i in 0..n {
        for (&k, v) in mp.clone().iter() {
            // s円以下かつs円になる組み合わせがまだないときのみ追記。
            // 追記回数は高々s回。
            if k + c[i] <= s && None == mp.get(&(k + c[i])) {
                let mut vv = v.clone();
                vv.push(i);
                mp.insert(k + c[i], vv);
                if k + c[i] == s {
                    break 'l;
                }
            }
        }
    }
    //ansの構築
    if let Some(v) = mp.get(&s) {
        let mut v = v.clone();
        v.reverse();
        let mut ans = vec![];
        for i in 0..n {
            if v.len() > 0 && i == v[v.len() - 1] {
                v.pop();
                //max
                ans.push(if ab[i].0 < ab[i].1 { 'B' } else { 'A' });
            } else {
                //min
                ans.push(if ab[i].0 > ab[i].1 { 'B' } else { 'A' });
            }
        }
        println!("{}", ans.iter().collect::<String>());
    } else {
        println!("Impossible");
    }
}
