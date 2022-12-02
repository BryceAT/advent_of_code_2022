use std::path::Path;
use std::io::{self, BufRead};
use std::fs::File;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[allow(dead_code)]
fn p1_1() -> i32 {
    let mut data = vec![Vec::new()];
    if let Ok(lines) = read_lines("../../data/p1_1.txt") {
        for line in lines {
            if let Ok(x) = line {
                if let Some(last) = data.last_mut() {
                    match x.parse::<i32>() {
                        Ok(num) => {last.push(num);},
                        Err(_) => {data.push(Vec::new());}
                    }
                }
            }
        }
    }
    data.iter().fold(0,|best,v| {best.max(v.iter().sum::<i32>())})
}

#[allow(dead_code)]
fn p1_2() -> i32 {
    let mut data = vec![Vec::new()];
    if let Ok(lines) = read_lines("../../data/p1_1.txt") {
        for line in lines {
            if let Ok(x) = line {
                if let Some(last) = data.last_mut() {
                    match x.parse::<i32>() {
                        Ok(num) => {last.push(num);},
                        Err(_) => {data.push(Vec::new());}
                    }
                }
            }
        }
    }
    let (a,b,c) = data.iter().fold((0,0,0),|(a,b,c),v| {
        match v.iter().sum::<i32>() {
            s if s >= a => (s,a,b),
            s if s >= b => (a,s,b),
            s if s > c => (a,b,s),
            _ => (a,b,c)
        }
    });
    a + b + c
}

fn main() {
    println!("{}",p1_2());
}
