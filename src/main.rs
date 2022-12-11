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
    let mut crates = vec![Vec::new();10];
    let mut ready = false;
    let lines = io::BufReader::new(File::open("data/p5_1.txt").expect("file not found")).lines();
    for (line_nbr,line) in lines.filter_map(|x| x.ok()).enumerate() {
        if line_nbr < 8 {
            for (i,c) in line.chars().enumerate() {
                if i % 4 == 1 && c != ' ' {
                    crates[i/4 + 1].push(c);
                }
            }
            continue
        }
        if !line.starts_with("move") {
            continue
        }
        if !ready {
            ready = true;
            for i in 1..10 {
                crates[i].reverse();
            }
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
#[allow(dead_code)]
fn p6_x(q_len:usize) -> i32 {
    // part1 is p6_x(4) and part2 is p6_x(14)
    use std::collections::VecDeque;
    let mut q = VecDeque::new();
    let lines = io::BufReader::new(File::open("data/p6_1.txt").expect("file not found")).lines();
    for line in lines.filter_map(|x| x.ok()) {
        for (i,c) in line.chars().enumerate() {
            if q.contains(&c) {
                while Some(c) != q.pop_front() {();}
            }
            q.push_back(c);
            if q.len() == q_len {
                return i as i32 +1
            }
        }
    }
    -1
}
#[allow(dead_code)]
fn p7_1() -> i64 {
    use std::collections::HashMap;
    let mut m = HashMap::new();
    let mut cur = "/".to_string();
    let lines = io::BufReader::new(File::open("data/p7_1.txt").expect("file not found")).lines();
    for line in lines.filter_map(|x| x.ok()) {
        if line.starts_with("$ cd") {
            match &line[5..] {
                "/" => {cur = "/".to_string();},
                ".." => {
                    let mut it = cur.chars();
                    it.next_back();
                    while '/' != it.next_back().unwrap() {();}
                    cur = it.collect::<String>() + "/"
                },
                f => {cur += &(f.to_string() + "/")},
            }
            continue
        }
        if line == "$ ls" {continue}
        let v = line.split(' ').collect::<Vec<_>>();
        if let Ok(size) = v[0].parse::<i32>() {
            m.insert(cur.clone() + v[1], size);
        }
    }
    let mut ans = HashMap::new();
    for (k,v) in m {
        let mut cur: String = k[..k.rfind('/').unwrap()].chars().collect();
        while cur.len() > 0 {
            *ans.entry(cur.clone()).or_insert(0) += v as i64;
            cur = cur[..cur.rfind('/').unwrap()].chars().collect();
        }
    }
    ans.values().filter(|v| *v < &100_000).sum()
}
#[allow(dead_code)]
fn p7_2() -> i64 {
    use std::collections::HashMap;
    let mut m = HashMap::new();
    let mut cur = "/".to_string();
    let lines = io::BufReader::new(File::open("data/p7_1.txt").expect("file not found")).lines();
    for line in lines.filter_map(|x| x.ok()) {
        if line.starts_with("$ cd") {
            match &line[5..] {
                "/" => {cur = "/".to_string();},
                ".." => {
                    let mut it = cur.chars();
                    it.next_back();
                    while '/' != it.next_back().unwrap() {();}
                    cur = it.collect::<String>() + "/"
                },
                f => {cur += &(f.to_string() + "/")},
            }
            continue
        }
        if line == "$ ls" {continue}
        let v = line.split(' ').collect::<Vec<_>>();
        if let Ok(size) = v[0].parse::<i32>() {
            m.insert(cur.clone() + v[1], size);
        }
    }
    let total_used = m.values().sum::<i32>();
    let min_drop_size = total_used - 40_000_000;
    let mut ans = HashMap::new();
    for (k,v) in m {
        let mut cur: String = k[..k.rfind('/').unwrap()].chars().collect();
        while cur.len() > 0 {
            *ans.entry(cur.clone()).or_insert(0) += v as i64;
            cur = cur[..cur.rfind('/').unwrap()].chars().collect();
        }
    }
    *ans.values().filter(|v| *v >= &(min_drop_size as i64)).min().unwrap()
}
#[allow(dead_code)]
fn p8_1() -> i32 {
    let mut grid:Vec<Vec<u32>> = Vec::new();
    let lines = io::BufReader::new(File::open("data/p8_1.txt").expect("file not found")).lines();
    for line in lines.filter_map(|x| x.ok()) {
        grid.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }
    fn is_visable(x:usize,y:usize,grid: &Vec<Vec<u32>>) -> bool {
        grid[x][..y].iter().all(|t| *t < grid[x][y]) ||
        grid[x][(y+1)..].iter().all(|t| *t < grid[x][y]) ||
        (0..x).all(|i| grid[i][y] < grid[x][y]) ||
        ((x+1)..grid.len()).all(|i| grid[i][y] < grid[x][y]) 
    }
    (0..grid.len()).fold(0,|acc,r| {
        acc + (0..grid[0].len()).fold(0,|col_sum,c|{
            col_sum+if is_visable(r,c,&grid) {1} else {0}
        })
    })
}
#[allow(dead_code)]
fn p8_2() -> i32 {
    let mut grid:Vec<Vec<u32>> = Vec::new();
    let lines = io::BufReader::new(File::open("data/p8_1.txt").expect("file not found")).lines();
    for line in lines.filter_map(|x| x.ok()) {
        grid.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }
    fn compare(a:u32,b:u32,acc:i32,done:bool) -> (i32,bool) {
        match (done,b) {
            (false,t) if t < a => {(acc+1,false)},
            (false,t) if t >=a => {(acc+1,true)},
            (_,_)  => {(acc,true)},
        }
    }
    fn ct_trees(x:usize,y:usize,grid: &Vec<Vec<u32>>) -> i32 {
        (0..y).rev().fold((0,false),|(acc,done),j|{compare(grid[x][y],grid[x][j],acc,done)}).0 *
        ((y+1)..grid[0].len()).fold((0,false),|(acc,done),j|{ compare(grid[x][y],grid[x][j],acc,done)}).0 *
        (0..x).rev().fold((0,false),|(acc,done),i|{ compare(grid[x][y],grid[i][y],acc,done) }).0 *
        ((x+1)..grid.len()).fold((0,false),|(acc,done),i|{ compare(grid[x][y],grid[i][y],acc,done)}).0 
    }
    (0..grid.len()).fold(0,|acc,r| {
        acc.max((0..grid[0].len()).fold(0,|col_max,c|{
            col_max.max(ct_trees(r,c,&grid))
        }))
    })
}
#[allow(dead_code)]
fn p9_1() -> usize {
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    let mut h = (0_i32,0_i32);
    let mut t = (0_i32,0_i32);
    seen.insert(t.clone());
    let lines = io::BufReader::new(File::open("data/p9_1.txt").expect("file not found")).lines();
    for line in lines.filter_map(|x| x.ok()) {
        let mut direction = ' ';
        let mut num = 0;
        for (i,c) in line.chars().enumerate() {
            match i {
                0 => {direction = c;},
                2.. => {num = num * 10 + c.to_digit(10).unwrap()},
                _ => (),
            }
        }
        match direction {
            'U' => {h = (h.0 + num as i32, h.1)},
            'D' => {h = (h.0 - num as i32, h.1)},
            'L' => {h = (h.0, h.1 - num as i32)},
            'R' => {h = (h.0, h.1 + num as i32)},
            _ => ()
        }
        while (h.0 - t.0).abs().max((h.1 - t.1).abs()) > 1 {
            t = (t.0 + (if h.0 == t.0 {0} else if h.0 > t.0 {1} else {-1}),
                t.1 + (if h.1 == t.1 {0} else if h.1 > t.1 {1} else {-1}));
            seen.insert(t.clone());
        }
    }
    seen.len()
}
#[allow(dead_code)]
fn p9_2() -> usize {
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    let mut rope = vec![(0_i32,0_i32);10];
    seen.insert(rope[9].clone());
    let lines = io::BufReader::new(File::open("data/p9_1.txt").expect("file not found")).lines();
    for line in lines.filter_map(|x| x.ok()) {
        let mut direction = ' ';
        let mut num = 0;
        for (i,c) in line.chars().enumerate() {
            match i {
                0 => {direction = c;},
                2.. => {num = num * 10 + c.to_digit(10).unwrap()},
                _ => (),
            }
        }
        for _ in 0..num {
            match direction {
                'U' => {rope[0] = (rope[0].0 + 1, rope[0].1)},
                'D' => {rope[0] = (rope[0].0 - 1, rope[0].1)},
                'L' => {rope[0] = (rope[0].0, rope[0].1 - 1)},
                'R' => {rope[0] = (rope[0].0, rope[0].1 + 1)},
                _ => ()
            }
            for i in 0..9 {
                while (rope[i].0 - rope[i+1].0).abs().max((rope[i].1 - rope[i+1].1).abs()) > 1 {
                    rope[i+1] = (rope[i+1].0 + (if rope[i].0 == rope[i+1].0 {0} else if rope[i].0 > rope[i+1].0 {1} else {-1}),
                    rope[i+1].1 + (if rope[i].1 == rope[i+1].1 {0} else if rope[i].1 > rope[i+1].1 {1} else {-1}));
                    if i == 8 {
                        seen.insert(rope[9].clone());
                    }
                }
            }
        }
    }
    //this test prints the path of the tail for slightly larger test. for a bigger board 12 and 25 would change
    //let shif:HashSet<(i32,i32)> = seen.iter().map(|(i,j)| (i+12,j+12)).collect();
    //for r in (0..25).rev() { println!("{:?}",(0..25).map(|c| if shif.contains(&(r,c)) {'#'} else {'.'}).collect::<String>());}
    seen.len()
}

#[allow(dead_code)]
fn p10_1() -> i32 {
    let mut cycle = 0;
    let mut prev = 1;
    let mut x = 1;
    let mut ans = 0;
    let mut check_point = vec![220,180,140,100,60,20];
    let lines = io::BufReader::new(File::open("data/p10_1.txt").expect("file not found")).lines();
    for line in lines.filter_map(|line| line.ok()) {
        if line.starts_with("noop") {
            cycle += 1;
        } else {
            cycle += 2;
            x += line.split(' ').collect::<Vec<_>>()[1].parse::<i32>().unwrap();
        }
        if !check_point.is_empty() && cycle >= *check_point.last().unwrap() {
            ans += check_point.pop().unwrap() * prev;
        }
        prev = x;
    }
    ans
}
#[allow(dead_code)]
fn p10_2() {
    let mut cycle = 0;
    let mut prev = 1;
    let mut x = 1;
    let mut loc = 0_i32;
    let mut ans: Vec<String> = Vec::new();
    let mut cur = "".to_string();
    let lines = io::BufReader::new(File::open("data/p10_1.txt").expect("file not found")).lines();
    for line in lines.filter_map(|line| line.ok()) {
        if line.starts_with("noop") {
            cycle += 1;
        } else {
            cycle += 2;
            x += line.split(' ').collect::<Vec<_>>()[1].parse::<i32>().unwrap();
        }
        while loc < cycle {
            cur += if (loc %40 - prev).abs() <= 1 {"X"} else {"."};
            loc += 1;
            if loc %40 == 0 {
                ans.push(cur);
                cur = "".to_string();
            }
        }
        prev = x;
    }
    for row in ans {
        println!("{row:?}");
    }
}
#[allow(dead_code)]
fn p11_x(div:i64,rounds:usize) -> i64 {
    //part 1: p11_x(3,20)
    //part 2: p11_x(1,10_000)
    use std::collections::{HashMap,VecDeque};
    let mut items = HashMap::new();
    let mut ops:HashMap<i64,Box<dyn Fn(i64)-> i64>> = HashMap::new();
    let mut throw = HashMap::new();
    let mut ct:HashMap<i64,i64> = HashMap::new();
    let mut cur_monkey = -1_i64;
    let lines = io::BufReader::new(File::open("data/p11_1.txt").expect("file not found")).lines();
    for line in lines.filter_map(|line| line.ok()) {
        if line.starts_with("Monkey") {
            cur_monkey += 1;
        } else if line.starts_with("  Starting items:") {
            let mut v = VecDeque::new();
            let mut cur_val = 0;
            for c in line.chars() {
                match c.to_digit(10) {
                    Some(d) => {cur_val = cur_val *10 + d;},
                    _ if cur_val > 0 => {v.push_back(cur_val as i64);cur_val = 0;},
                    _ => (),
                }
            }
            if cur_val > 0 {v.push_back(cur_val as i64);}
            items.insert(cur_monkey,v);
        } else if line.starts_with("  Operation:") {
            let ln = line.split(' ').collect::<Vec<_>>();
            if let Ok(val) = ln.last().unwrap().parse::<i64>() {
                if ln[ln.len() -2] == "+" {
                    ops.insert(cur_monkey,Box::new(move |x| {x + val}));
                } else {
                    ops.insert(cur_monkey,Box::new(move |x| {x * val}));
                }
            } else {
                ops.insert(cur_monkey,Box::new(|x| {x * x}));
            }
        } else if line.starts_with("  ") {
            let val = line.split(' ').collect::<Vec<_>>().last().unwrap().parse::<i64>().unwrap();
            throw.entry(cur_monkey).or_insert(Vec::new()).push(val);
        }
    }
    let worry_control = throw.values().fold(1,|acc,v| acc * v[0]);
    for monkey in (0..=cur_monkey).collect::<Vec<i64>>().repeat(rounds) {
        while !items[&monkey].is_empty() {
            *ct.entry(monkey).or_insert(0) += 1;
            let mut worry = items.get_mut(&monkey).unwrap().pop_front().unwrap();
            worry = (ops[&monkey](worry) / div) % worry_control;
            if worry % throw[&monkey][0] == 0 {
                items.get_mut(&throw[&monkey][1]).unwrap().push_back(worry);
            } else {
                items.get_mut(&throw[&monkey][2]).unwrap().push_back(worry);
            }
        }
    }
    let mut ans: Vec<_> = ct.values().collect();
    ans.sort_unstable();
    ans.pop().unwrap() * ans.pop().unwrap()
}
fn main() {
    println!("part 1 {}, part 2 {}", p11_x(3,20), p11_x(1,10_000));
}
