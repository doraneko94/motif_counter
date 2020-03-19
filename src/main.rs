use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Rev<T>(pub T);

impl<T: PartialOrd> PartialOrd for Rev<T> {
    fn partial_cmp(&self, other: &Rev<T>) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl<T: Ord> Ord for Rev<T> {
    fn cmp(&self, other: &Rev<T>) -> Ordering {
        other.0.cmp(&self.0)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage:\nmf [filename.txt]");
        return;
    }
    let filename = &args[1];
    let f = File::open(filename).expect("file not found");
    let reader = BufReader::new(f);
    let mut a = Vec::new();
    let mut v = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        if i == 0 {
            a = line.unwrap().chars().collect::<Vec<char>>();
        } else {
            v.push(line.unwrap().chars().collect::<Vec<char>>());
        }
    }

    let l = a.len();
    let mut range = BinaryHeap::new();
    let n = v.len();

    println!("...{} bases.", l);
    println!("");
    println!("================");
    println!("|| Each motif ||");
    println!("================");
    for i in 0..n {
        let mut ans = 0;
        let seg = &v[i];
        let l_seg = seg.len();
        for j in 0..(l-l_seg+1) {
            let mut flg = true;
            for k in 0..l_seg {
                if seg[k] != a[j+k] {
                    flg = false;
                    break;
                }
            }
            if flg {
                range.push((Rev(j), Rev(j+l_seg)));
                ans += 1;
            }
        }
        println!("{}: {}", seg.iter().collect::<String>(), ans);
    }

    println!("");
    let l = range.len();
    if l == 0 {
        println!("0");
    } else {
        let (_, Rev(e0)) = range.pop().unwrap();
        let mut e = e0;
        let mut ans = 1;
        while range.len() > 0 {
            let (Rev(b0), Rev(e0)) = range.pop().unwrap();
            if b0 >= e {
                ans += 1;
            }
            e = std::cmp::max(e, e0);
        }
        println!("Total: {}", ans);
    }
}
