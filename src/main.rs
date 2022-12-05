use std::path::Path;
use std::io::{self, BufRead};
use std::fs::File;

#[allow(dead_code)]
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

#[allow(dead_code)]
fn p2_1() -> i32 {
    let mut tot = 0;
    if let Ok(lines) = read_lines("../../data/p2_1.txt") {
        for line in lines {
            if let Ok(x) = line {
                let row:Vec<_> = x.chars().collect();
                match (row[0],row[2]) {
                    ('A','X') => {tot += 1 + 3},
                    ('A','Y') => {tot += 2 + 6},
                    ('A','Z') => {tot += 3},
                    ('B','X') => {tot += 1},
                    ('B','Y') => {tot += 2 + 3},
                    ('B','Z') => {tot += 3 + 6},
                    ('C','X') => {tot += 1 + 6},
                    ('C','Y') => {tot += 2},
                    ('C','Z') => {tot += 3 + 3},
                    _ => ()
                }
            }
        }
    }
    tot
}

#[allow(dead_code)]
fn p2_2() -> i32 {
    let mut tot = 0;
    if let Ok(lines) = read_lines("../../data/p2_1.txt") {
        for line in lines {
            if let Ok(x) = line {
                let row:Vec<_> = x.chars().collect();
                match (row[0],row[2]) {
                    ('A','X') => {tot += 3 + 0},
                    ('A','Y') => {tot += 1 + 3},
                    ('A','Z') => {tot += 2 + 6},
                    ('B','X') => {tot += 1 + 0},
                    ('B','Y') => {tot += 2 + 3},
                    ('B','Z') => {tot += 3 + 6},
                    ('C','X') => {tot += 2 + 0},
                    ('C','Y') => {tot += 3 + 3},
                    ('C','Z') => {tot += 1 + 6},
                    _ => ()
                }
            }
        }
    }
    tot
}

#[allow(dead_code)]
fn p3_1() -> i32 {
    use std::collections::HashSet;
    let mut tot = 0;
    if let Ok(lines) = read_lines("../../data/p3_1.txt") {
        for line in lines {
            if let Ok(x) = line {
                //println!("{}",x);
                let row:Vec<u8> = x.bytes().collect();
                let intersect = *row[..row.len()/2].iter().map(|x| *x).collect::<HashSet<u8>>()
                .intersection(
                    &row[row.len()/2..].iter().map(|x| *x).collect::<HashSet<u8>>()
                ).next().unwrap();
                match intersect {
                    b'A'..=b'Z' => {tot += (27 + intersect - b'A') as i32},
                    b'a'..=b'z' => {tot += (1 + intersect - b'a') as i32},
                    _ => ()
                }
            }
        }
    }
    tot
}
#[allow(dead_code)]
fn p3_2() -> i32 {
    use std::collections::HashSet;
    let mut data = Vec::new();
    if let Ok(lines) = read_lines("data/p3_1.txt") {
        for line in lines {
            if let Ok(x) = line {
                let row:Vec<u8> = x.bytes().collect();
                data.push(row);
            }
        }
    }
    data.chunks(3)
        .map(|v| *v[0].iter().map(|x| *x).collect::<HashSet<u8>>()
            .intersection(&v[1].iter().map(|x| *x).collect::<HashSet<u8>>())
            .map(|x| *x)
            .collect::<HashSet<u8>>()
            .intersection(&v[2].iter().map(|x| *x).collect::<HashSet<u8>>())
            .next().unwrap()).fold(0,|acc, badge| {
                match badge {
                    b'A'..=b'Z' => {acc + (27 + badge - b'A') as i32},
                    b'a'..=b'z' => {acc + (1 + badge - b'a') as i32},
                    _ => acc
                }
            })
}
#[allow(dead_code)]
fn p4_1() -> i32 {
    let mut tot: i32 = 0;
    if let Ok(lines) = read_lines("data/p4_1.txt") {
        for line in lines {
            if let Ok(x) = line {
                let row:Vec<_> = x.chars().collect();
                let mut i = 0;
                let mut quad = vec![0;4];
                for c in row {
                    match c.to_digit(10) {
                        Some(d) => {quad[i] = quad[i] * 10 + d},
                        None => {i += 1;}
                    }
                }
                if (quad[0] <= quad[2] && quad[3] <= quad[1]) || (quad[2] <= quad[0] && quad[1] <= quad[3] ) {
                    tot += 1;
                }
            }
        }
    }
    tot
}
#[allow(dead_code)]
fn p4_2() -> i32 {
    let mut tot: i32 = 0;
    let lines = io::BufReader::new(File::open("data/p4_1.txt").unwrap()).lines();
    for line in lines.filter_map(|x| x.ok()) {
        let row:Vec<_> = line.chars().collect();
        let mut i = 0;
        let mut quad = vec![0;4];
        for c in row {
            match c.to_digit(10) {
                Some(d) => {quad[i] = quad[i] * 10 + d},
                None => {i += 1;}
            }
        }
        if quad[0].max(quad[2]) <= quad[1].min(quad[3]) {
            tot += 1;
        }
    }
    tot
}
#[allow(dead_code)]
fn p5_1() -> String {
    let mut crates = vec![Vec::new(),
    vec!['N','C','R','T','M','Z','P'],
    vec!['D','N','T','S','B','Z'],
    vec!['M','H','Q','R','F','C','T','G'],
    vec!['G','R','Z'],
    vec!['Z','N','R','H'],
    vec!['F','H','S','W','P','Z','L','D'],
    vec!['W','D','Z','R','C','G','M'],
    vec!['S','J','F','L','H','W','Z','Q'],
    vec!['S','Q','P','W','N']];

    let lines = io::BufReader::new(File::open("data/p5_1.txt").expect("file not found")).lines();
    for line in lines.filter_map(|x| x.ok()) {
        if !line.starts_with("move") {
            continue
        }
        let row:Vec<_> = line.chars().collect();
        let mut i = 0;
        let mut triple = vec![0;3];
        for c in row {
            match c.to_digit(10) {
                Some(d) => {triple[i] = triple[i] * 10 + d},
                None if triple[i] > 0 => {i += 1;},
                _ => ()
            }
        }
        for _ in 0..triple[0]{
            let c = crates[triple[1] as usize].pop().unwrap();
            crates[triple[2] as usize].push(c);
        }
    }
    (1..10).map(|i| crates[i].last().unwrap()).collect()
}
#[allow(dead_code)]
fn p5_2() -> String {
    let mut crates = vec![Vec::new(),
    vec!['N','C','R','T','M','Z','P'],
    vec!['D','N','T','S','B','Z'],
    vec!['M','H','Q','R','F','C','T','G'],
    vec!['G','R','Z'],
    vec!['Z','N','R','H'],
    vec!['F','H','S','W','P','Z','L','D'],
    vec!['W','D','Z','R','C','G','M'],
    vec!['S','J','F','L','H','W','Z','Q'],
    vec!['S','Q','P','W','N']];

    let lines = io::BufReader::new(File::open("data/p5_1.txt").expect("file not found")).lines();
    for line in lines.filter_map(|x| x.ok()) {
        if !line.starts_with("move") {
            continue
        }
        let row:Vec<_> = line.chars().collect();
        let mut i = 0;
        let mut triple = vec![0_usize;3];
        for c in row {
            match c.to_digit(10) {
                Some(d) => {triple[i] = triple[i] * 10 + d as usize},
                None if triple[i] > 0 => {i += 1;},
                _ => ()
            }
        }
        let take_ind =  crates[triple[1]].len() - triple[0];
        let mut stack:Vec<_> = crates[triple[1]].drain(take_ind..).collect();
        crates[triple[2]].append(&mut stack);
    }
    (1..10).map(|i| crates[i].last().unwrap()).collect()
}
fn main() {
    println!("{}",p5_1());
    println!("{}",p5_2());
}
