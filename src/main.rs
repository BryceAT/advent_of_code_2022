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
#[allow(dead_code)]
fn p12_1() -> i32 {
    use std::collections::{HashSet,HashMap};
    let mut start = (0,0);
    let mut end = (0,0);
    let mut seen = HashSet::new();
    let mut level = HashSet::new();
    let mut map = HashMap::new();
    let mut step = 0;
    let lines = io::BufReader::new(File::open("data/p12_1.txt").expect("file not found")).lines();
    for (r,line) in lines.filter_map(|line| line.ok()).enumerate() {
        for (c,x) in line.bytes().enumerate() {
            if x == b'S' {
                start = (r,c);
                map.insert((r,c),b'a');
            } else if x == b'E' {
                end = (r,c);
                map.insert((r,c),b'z');
            } else {
                map.insert((r,c),x);
            }
        }
    }
    level.insert(start.clone());
    while !level.contains(&end) {
        step += 1;
        seen = seen.union(&level).map(|x| *x).collect();
        let mut new_level = HashSet::new();
        for (x,y) in level {
            for (a,b) in [(x.wrapping_add(!0),y),(x+1,y),(x,y.wrapping_add(!0)),(x,y+1)] {
                if map.contains_key(&(a,b)) && !seen.contains(&(a,b)) && map[&(a,b)] <= map[&(x,y)] + 1 {
                    new_level.insert((a,b));
                }
            }
        }
        level = new_level;
    }
    step
}
#[allow(dead_code)]
fn p12_2() -> i32 {
    use std::collections::{HashSet,HashMap};
    let mut end = (0,0);
    let mut seen = HashSet::new();
    let mut level = HashSet::new();
    let mut map = HashMap::new();
    let mut step = 0;
    let lines = io::BufReader::new(File::open("data/p12_1.txt").expect("file not found")).lines();
    for (r,line) in lines.filter_map(|line| line.ok()).enumerate() {
        for (c,x) in line.bytes().enumerate() {
            if x == b'S' {
                map.insert((r,c),b'a');
            } else if x == b'E' {
                end = (r,c);
                map.insert((r,c),b'z');
            } else {
                map.insert((r,c),x);
            }
        }
    }
    level.insert(end.clone());
    loop {
        seen = seen.union(&level).map(|x| *x).collect();
        let mut new_level = HashSet::new();
        for (x,y) in level {
            for (a,b) in [(x.wrapping_add(!0),y),(x+1,y),(x,y.wrapping_add(!0)),(x,y+1)] {
                if map.contains_key(&(a,b)) && !seen.contains(&(a,b)) && map[&(a,b)] >= map[&(x,y)] - 1 {
                    if map[&(x,y)] == b'a' {
                        return step
                    }
                    new_level.insert((a,b));
                }
            }
        }
        level = new_level;
        step += 1;
    }
}
#[derive(Debug, Eq)]
enum Packet {
        Num(i32),
        List(Vec<Packet>),
    }
#[allow(dead_code)]
fn mk_p13_data() -> Vec<Packet> {
    use std::cmp::Ordering;
    impl Packet {
        fn new(x: Vec<char>) -> Packet {
            if x.len() == 0 {
                return Packet::List(Vec::new())
            }
            if x[0] == '[' {
                let mut cur: Vec<Packet> = Vec::new();
                let mut l= 1;
                let mut hill = 0;
                for (r,c) in x.iter().enumerate().skip(1) {
                    match c {
                        '[' => {hill += 1;},
                        ']' => {hill -= 1;},
                        ',' if hill == 0 => {
                        cur.push(Packet::new(x[l..r].iter().map(|x| *x).collect()));
                        l = r+1;
                        },
                        _ => (),
                    }
                }
                cur.push(Packet::new(x[l..x.len() -1].iter().map(|x| *x).collect()));
                Packet::List(cur)
            } else {
                if let Ok(v) = x.iter().collect::<String>().parse::<i32>() {
                    Packet::Num(v)
                } else {
                    println!("can not parse this number {:?}", x);
                    Packet::new(vec!['9','9','9'])
                }
                
            }
        }
        fn cmp(&self, other: &Self) -> Ordering {
            match (self,other) {
                (Packet::Num(num1),Packet::Num(num2)) => num1.cmp(num2),
                (Packet::Num(_),Packet::List(_)) => Packet::List(vec![self.clone()]).cmp(other),
                (Packet::List(_),Packet::Num(_)) => self.cmp(&Packet::List(vec![other.clone()])),
                (Packet::List(v1),Packet::List(v2)) => {
                    for (a,b) in v1.iter().zip(v2.iter()) {
                        match (*a).cmp(b) {
                            Ordering::Equal => continue,
                            order => return order
                        }
                    }
                    v1.len().cmp(&v2.len())
                }
            }
        }
    }

    impl Ord for Packet {
        fn cmp(&self, other: &Self) -> Ordering {
            (*self).cmp(other)
        }
    }
    impl PartialOrd for Packet {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some((*self).cmp(other))
        }
    }

    impl PartialEq for Packet {
        fn eq(&self, other: &Self) -> bool {
            match (self,other) {
                (Packet::Num(n1),Packet::Num(n2)) => {n1 == n2},
                (Packet::List(v1),Packet::List(v2)) => {v1.len() == v2.len() && v1.iter().zip(v2.iter()).all(|(a,b)| a == b)},
                _ => false
            }
        }
    }
    impl Clone for Packet {
        fn clone(&self) -> Self {
            match self {
                Packet::Num(n) => Packet::Num(*n),
                Packet::List(v) => Packet::List(v.clone())
            }
        }
    }
    let mut data: Vec<Packet> = Vec::new();
    let lines = io::BufReader::new(File::open("data/p13_1.txt").expect("file not found")).lines();
    for line in lines.filter_map(|line| line.ok()) {
        if !line.is_empty() {
            data.push(Packet::new(line.chars().collect()));
        }
    }
    data
}
#[allow(dead_code)]
fn p13_1() -> i32 {
    let data = mk_p13_data();
    let mut ans = 0;
    for i in 0..data.len()/2 {
        ans += if data[2*i] <= data[2*i + 1] {i+1} else {0};
    }
    ans as i32
}
#[allow(dead_code)]
fn p13_2() -> i32 {
    let mut data = mk_p13_data();
    let e2 = Packet::new(format!("[[2]]").chars().collect::<Vec<char>>());
    let e6 = Packet::new(format!("[[6]]").chars().collect::<Vec<char>>());
    data.push(e2.clone());
    data.push(e6.clone());
    data.sort();
    ((data.iter().position(|x| x == &e2).unwrap() +1) * 
     (data.iter().position(|x| x == &e6).unwrap() +1)) as i32
}
#[allow(dead_code)]
fn p14_1() -> i32 {
    use std::collections::HashSet;
    let mut m = HashSet::new();
    let lines = io::BufReader::new(File::open("data/p14_1.txt").expect("file not found")).lines();
    for line in lines.filter_map(|line| line.ok()) {
        let mut mem: Option<(u16,u16)> = None;
        for pair in line.split(" -> ") {
            let v:Vec<_> = pair.split(',').map(|x| x.parse::<u16>().unwrap()).collect();
            let cur = (v[0],v[1]);
            if let Some(mid) = mem {
                if mid.0 == cur.0 {
                    for i in mid.1.min(cur.1) ..= mid.1.max(cur.1){
                        m.insert((mid.0,i));
                    }
                } else {
                    for i in mid.0.min(cur.0) ..= mid.0.max(cur.0){
                        m.insert((i,mid.1));
                    }
                }
            }
            mem = Some(cur.clone());
            m.insert(cur);
        }
    }
    let lowest = *m.iter().map(|(_,y)| y).max().unwrap();
    let mut cur = (500,0);
    let mut ans = 0;
    while cur.1 <= lowest {
        match cur {
            (x,y) if !m.contains(&(x,y+1)) => {cur = (x,y+1);},
            (x,y) if !m.contains(&(x-1,y+1)) => {cur = (x-1,y+1);},
            (x,y) if !m.contains(&(x+1,y+1)) => {cur = (x+1,y+1);},
            _ => {ans += 1; m.insert(cur); cur = (500,0);}
        }
    }
    ans
}
#[allow(dead_code)]
fn p14_2() -> i32 {
    use std::collections::HashSet;
    let mut m = HashSet::new();
    let lines = io::BufReader::new(File::open("data/p14_1.txt").expect("file not found")).lines();
    for line in lines.filter_map(|line| line.ok()) {
        let mut mem: Option<(u16,u16)> = None;
        for pair in line.split(" -> ") {
            let v:Vec<_> = pair.split(',').map(|x| x.parse::<u16>().unwrap()).collect();
            let cur = (v[0],v[1]);
            if let Some(mid) = mem {
                if mid.0 == cur.0 {
                    for i in mid.1.min(cur.1) ..= mid.1.max(cur.1){
                        m.insert((mid.0,i));
                    }
                } else {
                    for i in mid.0.min(cur.0) ..= mid.0.max(cur.0){
                        m.insert((i,mid.1));
                    }
                }
            }
            mem = Some(cur.clone());
            m.insert(cur);
        }
    }
    let floor = *m.iter().map(|(_,y)| y).max().unwrap() + 1;
    let mut cur = (500,0);
    let mut ans = 0;
    loop {
        match cur {
            (_,y) if y == floor => {ans += 1; m.insert(cur); cur = (500,0);},
            (x,y) if !m.contains(&(x,y+1)) => {cur = (x,y+1);},
            (x,y) if !m.contains(&(x-1,y+1)) => {cur = (x-1,y+1);},
            (x,y) if !m.contains(&(x+1,y+1)) => {cur = (x+1,y+1);},
            _ if cur == (500,0) => {ans += 1;break},
            _ => {ans += 1; m.insert(cur); cur = (500,0);}
        }
    }
    ans
}
#[allow(dead_code)]
fn p15_1(check_row: i32, count_beacon: bool) -> i32 {
    //println!("part 1 {}", p15_1(2000000,false))
    let mut sensors: Vec<Vec<i32>>= Vec::new();
    let lines = io::BufReader::new(File::open("data/p15_1.txt").expect("file not found")).lines();
    for line in lines.filter_map(|line| line.ok()) {
        sensors.push(line.split(&['=',',',':']).filter_map(|part| part.parse::<i32>().ok()).collect());
    }
    let mut intervals: Vec<(i32,i32)> = Vec::new();
    for sensor in sensors {
        let mad = (sensor[0] - sensor[2]).abs() + (sensor[1] - sensor[3]).abs();
        let sensor_to_row = (check_row - sensor[1]).abs();
        if  sensor_to_row <= mad {
            let mut l = sensor[0] - mad + sensor_to_row;
            let mut r = sensor[0] + mad - sensor_to_row;
            if !count_beacon {
                if l == sensor[2] {
                    l += 1;
                } 
                if r == sensor[2] {
                    r -= 1;
                }
            }
            if l <= r {
                intervals.push((l,r));
            }
        }
    }
    intervals.sort();
    let mut i = 0;
    while i < intervals.len() - 1 {
        if intervals[i].1 >= intervals[i + 1].0 {
            intervals[i] = (intervals[i].0, intervals[i].1.max(intervals.remove(i+1).1));
        } else {
            i += 1;
        }
    }
    intervals.iter().fold(0,|acc,&(a,b)| acc + b - a + 1)
}
#[allow(dead_code)]
fn p15_2(limit: i32, count_beacon: bool) -> i64 {
    //println!("part 2 {}", p15_2(4_000_000,true));
    let mut sensors: Vec<Vec<i32>>= Vec::new();
    let lines = io::BufReader::new(File::open("data/p15_1.txt").expect("file not found")).lines();
    for line in lines.filter_map(|line| line.ok()) {
        sensors.push(line.split(&['=',',',':']).filter_map(|part| part.parse::<i32>().ok()).collect());
    }
    let mut known_locs = std::collections::HashSet::new();
    for sensor in &sensors {
        known_locs.insert((sensor[0],sensor[1]));
        known_locs.insert((sensor[2],sensor[3]));
    }
    let mut intervals: Vec<(i32,i32)>;
    for check_row in 0..limit {
        intervals = Vec::new();
        for sensor in &sensors {
            let mad = (sensor[0] - sensor[2]).abs() + (sensor[1] - sensor[3]).abs();
            let sensor_to_row = (check_row - sensor[1]).abs();
            if  sensor_to_row <= mad {
                let mut l = sensor[0] - mad + sensor_to_row;
                let mut r = sensor[0] + mad - sensor_to_row;
                if !count_beacon {
                    if l == sensor[2] {
                        l += 1;
                    } 
                    if r == sensor[2] {
                        r -= 1;
                    }
                }
                if l <= r {
                    intervals.push((l,r));
                }
            }
        }
        intervals.sort_unstable();
        let mut i = 0;
        while i < intervals.len() - 1 {
            if intervals[i].1 >= intervals[i + 1].0 {
                intervals[i] = (intervals[i].0, intervals[i].1.max(intervals.remove(i+1).1));
            } else {
                i += 1;
            }
        }
        if intervals.iter().any(|(_,b)| 0<= *b && *b < limit) {
            let b = intervals.clone().into_iter().filter_map(|(_,b)| if 0<= b && b < limit {Some(b + 1)} else {None}).next().unwrap();
            if !known_locs.contains(&(b,check_row)) {
                return b as i64 * 4_000_000 + check_row as i64
            }
        }
    }
    !0
}
#[allow(dead_code)]
fn p16_1() -> i32 {
    //println!("part 1 {}", p16_1());
    use std::collections::HashMap;
    use regex::Regex;
    let re = Regex::new(r"Valve (?P<cur_name>[A-Z]{2}) has flow rate=(?P<rate>\d+)(; tunnels lead to valves |; tunnel leads to valve )(?P<flow2>.*)").unwrap();
    let mut valves_rate: HashMap<String,i32> = HashMap::new();
    let mut valves2: HashMap<String,Vec<String>>= HashMap::new();
    let lines = io::BufReader::new(File::open("data/p16_1.txt").expect("file not found")).lines();
    for line in lines.filter_map(|line| line.ok()) {
        let groups = re.captures(&line).unwrap();
        let cur_name:String = groups.name("cur_name").unwrap().as_str().to_string();
        valves_rate.insert(cur_name.clone(), groups.name("rate").unwrap().as_str().parse::<i32>().unwrap());
        for out_name in groups.name("flow2").unwrap().as_str().split(", ") {
            valves2.entry(cur_name.clone()).or_insert(Vec::new()).push(out_name.to_string());
        }
    }
    let is_open_map: HashMap<String,usize> = valves_rate.keys().enumerate().map(|(i,k)| (k.to_string(),i)).collect();
    let is_open: Vec<bool> = vec![false; is_open_map.len()];
    let mut mem: HashMap<(String,i32,Vec<bool>),i32> = HashMap::new();
    fn dfs(valves_rate: &HashMap<String,i32>, valves2: &HashMap<String,Vec<String>>, is_open_map: &HashMap<String,usize>,
            cur_valve:String, remaining_time: i32, is_open: Vec<bool>, prev_flow: i32, 
            mem: &mut HashMap<(String,i32,Vec<bool>),i32>) -> i32 {
        let remaining_time = remaining_time - 1;
        let prev_flow = prev_flow + valves_rate.iter().filter_map(|(name,rate)| if is_open[is_open_map[name]] {Some(rate)} else {None}).sum::<i32>();
        let mut is_open = is_open.clone();
        if remaining_time <= 0 {
            return prev_flow
        } else {
            let mut best = 0;
            for next_valve in &valves2[&cur_valve] {
                if let Some(val) = mem.get(&(next_valve.to_string(), remaining_time, is_open.clone())) {
                    best = best.max(*val);
                } else {
                    let val = dfs(valves_rate, valves2,is_open_map, next_valve.to_string(), remaining_time, is_open.clone(),0,mem);
                    mem.insert((next_valve.clone(), remaining_time, is_open.clone()),val);
                    best = best.max(*mem.get(&(next_valve.clone(), remaining_time, is_open.clone())).unwrap());
                }
            }
            if valves_rate[&cur_valve] > 0 && !is_open[is_open_map[&cur_valve]] {
                is_open[is_open_map[&cur_valve]] = true;
                if let Some(val) = mem.get(&(cur_valve.clone(), remaining_time, is_open.clone())) {
                    best = best.max(*val);
                } else {
                    let val = dfs(valves_rate, valves2, is_open_map, cur_valve.clone(), remaining_time, is_open.clone(),0,mem);
                    mem.insert((cur_valve.clone(), remaining_time, is_open.clone()),val);
                    best = best.max(*mem.get(&(cur_valve.clone(), remaining_time, is_open.clone())).unwrap());
                }
            }
            return prev_flow + best
        }
    }
    dfs(&valves_rate, &valves2, &is_open_map, "AA".to_string(), 30, is_open, 0, &mut mem)
}
#[allow(dead_code)]
fn p16_2() -> i32 {
    //println!("part 2 {}", p16_2());
    #[allow(unused_imports)]
    use std::collections::{HashMap,HashSet,BinaryHeap};
    #[allow(unused_imports)]
    use itertools::Itertools;
    use regex::Regex;
    let re = Regex::new(r"Valve (?P<cur_name>[A-Z]{2}) has flow rate=(?P<rate>\d+)(; tunnels lead to valves |; tunnel leads to valve )(?P<flow2>.*)").unwrap();
    let lines = io::BufReader::new(File::open("data/p16_1.txt").expect("file not found")).lines();
    let mut names = Vec::new();
    let mut rates = Vec::new();
    let mut flow2s = Vec::new();
    for line in lines.filter_map(|line| line.ok()) {
        let groups = re.captures(&line).unwrap();
        names.push(groups.name("cur_name").unwrap().as_str().to_string());
        rates.push(groups.name("rate").unwrap().as_str().parse::<i32>().unwrap());
        let mut flow2 = Vec::new();
        for out_name in groups.name("flow2").unwrap().as_str().split(", ") {
            flow2.push(out_name.to_string().clone());
        }
        flow2s.push(flow2);
    }
    let mut data: Vec<(String,i32,Vec<String>)> = names.clone().into_iter().zip(rates.iter()).zip(flow2s.clone().into_iter()).map(|((n,r),f)| (n,*r,f)).collect();
    data.sort_by_key(|(n,r,_)| (-r,if n == "AA" {0} else {1}));
    let subset_names:Vec<String> = data.clone().into_iter().filter_map(|(n,r,_)| if (n == "AA") || (r > 0) {Some(n)} else {None}).collect();
    let name_map = data.iter().enumerate().map(|(i,(n,_,_))| (n,i)).collect::<HashMap<_,_>>();
    let mut dist: Vec<Vec<i32>> = vec![vec![0;subset_names.len()];subset_names.len()];
    for a in 0..subset_names.len() {
        for b in 0..subset_names.len() {
            let mut level = vec![a];
            let mut seen = HashSet::new();
            let mut steps = 0;
            while !level.contains(&b) {
                steps += 1;
                let mut new_level = Vec::new();
                for n in level {
                    seen.insert(n);
                    for m in data[n].2.clone() {
                        let canidate = name_map[&m];
                        if !seen.contains(&canidate) {
                            new_level.push(canidate);
                        }
                    }
                }
                level = new_level;
            }
            dist[a][b] = steps;
        }
    }
    let mut mem: HashMap<(usize,i32,Vec<bool>),i32> = HashMap::with_capacity(20000);
    fn bfs_score(cur:usize,need:HashSet<usize>,dist: &Vec<Vec<i32>>, data: &Vec<(String,i32,Vec<String>)>, steps: i32,
                mem: &mut HashMap<(usize,i32,Vec<bool>),i32>) -> i32 {
        if need.is_empty() {return 0}
        let cur_ind = (cur,steps,(0..15).map(|i| need.contains(&i)).collect());
        if let Some(val) = mem.get(&cur_ind) {return *val}
        let mut best = 0_i32;
        let mut nxt_need = need.clone();
        for nxt in need {
            nxt_need.remove(&nxt);
            best = best.max(data[nxt].1 * (steps - dist[cur][nxt] -1) + bfs_score(nxt,nxt_need.clone(), dist, data, steps - dist[cur][nxt] -1,mem));
            nxt_need.insert(nxt);
        }
        mem.insert(cur_ind,best);
        best
    }
    let mut best = 0;
    let aa = "AA".to_string();
    for need1 in (0..subset_names.len() - 1).combinations((subset_names.len() - 1)/2) {
        let need1: HashSet<usize> = need1.iter().map(|x| *x).collect();
        let need2: HashSet<usize> = (0..subset_names.len() - 1).filter(|i| !need1.contains(i)).collect();
        best = best.max(bfs_score(name_map[&aa],need1, &dist, &data, 26, &mut mem) + bfs_score(name_map[&aa],need2, &dist, &data, 26, &mut mem));
    }
    best
}
#[allow(dead_code)]
fn p17_x(rocks:i64) -> i64 {
    // part 1 rocks = 2022
    // part 2 rocks = 1000000000000
    use std::collections::{HashSet,HashMap};
    let shapes:Vec<HashSet<(i64,i64)>> = vec![
    [(0,0),(1,0),(2,0),(3,0)].into_iter().collect(), //flat
    [(1,0),(1,1),(1,2),(0,1),(2,1)].into_iter().collect(), //plus
    [(0,0),(1,0),(2,0),(2,1),(2,2)].into_iter().collect(), //ell
    [(0,0),(0,1),(0,2),(0,3)].into_iter().collect(), //vert
    [(0,0),(1,0),(1,1),(0,1)].into_iter().collect(), //square
    ];
    //there are 10_091 chars in jet, so max enumerate is 10_090
    let mut jet = ">><>><<><>><>>><<><<<>><<<><>>>><<<>>><<>><<<<>><<<>>>><<<>>><<<<>><<<>>><<<<>><<>>>><>>>><<><<>>>><<>><>>>><<>>>><<<<>>><>><>><<<<><>>><<>>><<><<<><<<<>><><<>>><<<>><<<<>>>><><<<>>>><>>>><<<><<>>><<>><<<>>>><>><>>>><><<<<>><><<<<>><<<<>><><>><<>>>><>>><<<>><<>><<>>><<<>><<<><>>><<<<>>>><<<<>>>><<<>>><>>><<<<>>>><<<<><<>>>><<<>>><<<><><>>>><<<<>>><<>>><<<>>><>>><<<>>>><><<<<>>>><>><<<>><<><<<<>><<<>><>>><<<<>>>><<<<>>>><<<>>><<<>>>><>>><<<><<<>>><<>>>><<<>>><<<>>>><<<<>><<<<>>><<<<><>><<<<>>><<<<>>>><<>>><>><<<<><<<<>>>><<<>>>><<><<<<><<<>><<<<>><>>><<><<<>>><<>><<<<>><<<>>><>>>><<>>><<<>>>><<>>>><<<><<<<>>>><<>>>><>><<<<>><<<>>>><<<<><>><><<>>><<<<>>><<<><><<<<><<<<>><<><>>><>>>><<<>>><<<>><>>>><<<<>><<<>><<>>><<<<>><<<>><<<><>><<>>><>>><<<><<<<>>><<<<>>>><<<><<<><<><<<><<<><<<<><<<>><><>>><<<><<>>><<<>><>><<<><>>>><<<<>>>><<<>>>><<>><><<<<>>><<<<>>><<<<>>>><><<>>>><<<<>>><<<>><<>>>><<<><>><<>>><<<>>>><<<>>>><<<<>><>><<<><<<>>><<>>><<>>>><<<<>><<<>>><>>><<<<>><<<<>>>><<<>>>><<<<><<<<>>><<<><<<<>>><>><<<<>>>><<><>><<<>>><<<>><<>>>><<>>><<>>><<<<>>>><>>>><<<>><>>>><<<<>>>><><<<<>>>><<<<>>>><<<<><<>><<<><<><<<>><<>><<<<><<<>>>><<<<>>>><<><>>>><<<>><<>><<<>>><<<<>>><<<>><<<<><<<<><>>><<<<>><<>>>><<<>>>><<<<>>>><<>>>><<<>>>><>><>>><<>>><<<<>>>><<<>>>><<>>>><<>>>><<<<>>>><>><>><<>><<<<><><<<>><<<<><<<>><<><<>><<<<><<>>>><><<<>>><<<><<<<>>>><>>>><>>>><<<><<<><>>>><><<<>>><><<>>><>>><<<>><<<<><<>>>><<>>><<<<><>><<<<>>><<>><<<<>><<<<>>><<>>>><<>><<>>>><<<>>>><<>>><>>><<<>>><<<<>>><>>><<<<>><<>><<<<>><>><<<<><>><>>>><<<>><<>>><>>><<<>>>><<<<>><<<<>><<>>><<>>>><<<<>>>><<<>><<>><>>>><<<><>><<<>>>><<<>>>><<<>><<><><<<>><<<<><>><>>>><<>><<<><<<<><<<<>>><<<><<>>><<>><<<<>>><<<>><<>>><>>>><<>><<<<><<>><<<<>>><<<>>><<<>><<>>><<<<><<>><<><<<<>><>><>><<>>><>>><<<<><<<>>><<<>>>><<<<>>>><<<<>>><<><<<>>>><<<<>>>><><>>><<<<>>><<<<>>>><<<>>>><<<><<>>>><<<<>>>><>>>><<<><<>>><<<>><>>><<<<><<>>>><<>>>><<<><<<<>>><<<<><<<><<<>>><<<><<<<>><>><<<><<>>><<<>><<<<><<<<><<<<><>>>><<>><>><<<>>>><<<>>><>>>><<<<>><<<><>>><<<<>>><>>>><>>>><<<<>>><<<<>>><<<>><<<<>><<>>><<<<>><<<>>>><>>><<<<>>><><<<<>>><>><<<>>><<<>>>><<<<><<<><<>>>><<<>>><>>>><>>><<>><<<><<>>>><<<<>><<<<><><<>>>><<><<>>>><<>>>><<<<>><<<<><><<<><>><<<<>>><<<<>><<<<>><>><<<<>>>><<<<><>><<<>>>><>>><<<>><<><<<>><<>>><<<><>>><<<<>>><><<<<><<<<>>><<<>>>><<<>>><>>><<<<><<<>>><>><<>><><<<>><<>>><<<<>><<<>>>><<>><>><>><<<<>>><<<><<>>>><<<<>><<>>>><<<>>><<<>><<><>>><<<<><<<>><<<<>>>><<>>><<<>>><>>><<<>>>><<>><<>>><<>><>>><<<>>><<>><<>>><<<>>><<<>><>><<<>>><>><<<>>>><<>><<>><<>>><<>>><<<<>>><<<><<>>>><<>>><<>>>><<<<><<<<>>><<<<>><>>>><<<>>><<<<>><<<<>>>><>>>><>><<<><>>><<>>><<<<>>>><<<<>><<><<><>>><<<<><<<>>><>>><><<>>><<><<><>><<<><>>>><>>>><<>>><<>><<<<>>>><<<>><<<<>>><<<<><<>>><<<<>>>><<<<>>>><><>>><<>><<>>><>>>><<<>>><<<<><>><>>><<<<>>><<<>><<>>>><>>>><<<><<<<><<<<>><<>><<<<>>>><>>>><>>>><<<><<>>><>><>>><<><>><<>>>><<>>><<<>>>><<<<>>>><>><><<>>><<<><<<<>>><>>><>>><<<>>><<<<>><<>><<<<>>><><<<>>>><<<<>><>>><<<<>>><>><>><>>>><<>>><<<<>>>><<<<>>>><<<<>>><<<<>>><<<>><<<<>>><<><<<>>><<<<>>><<<<>>><<>>>><<<>><>><>>><<<<>>><><<<<><<<><<<<>><>>>><<<<>><<<<>>><<<<>>>><<>>><<<<>><>><<>><<<>>>><<<>>><<<>>><<<<>>>><<>>>><<>>>><<<><<<<><<><<<<>><><>><>>><>>>><<<><>>>><<><>><><<<>><<<<>>>><><<><<<>>><<<<><<>>><<<>><<<<><>><>>>><>>><<<><<>><><<<>><<>>><<><<<<>>><<>><<<><<<><><<<<>>><<<<><<<><<<<>>><>>><>>>><<<>>><>>><><<>>>><<<>>><<<>>><>>>><<<>>>><>>><><<>>>><<<<>><<<>><>>><<>>>><<>>>><><><<<<>>>><<<><<>>>><<>><<<><<><>>><>>><<>>>><<><<<>><<><<<<><<<<>><<<<>>><<<>>>><<<<>>>><<<>>>><>>><>>><<>>>><<<<>>><>>><<>>><<<>><<>>><<<><<<<>>>><<<<>>>><<><<<><<><<><>><<<>>>><<<>>><<<<>><<<>><<>><>>>><<<<>>>><<<<>>>><<<>>><>>><><<>>><<<<>><<>>><<<<><>>><>>>><>><>><<<<>>>><>>><<<><<>>><>>><<><>><<>>><<<>><<<<>>>><>>><>>>><>>>><<<<><<>><<<>>><>>><>>>><><><<<<>><>><>>><<<>>><<<<>><<<><><<>>>><>>>><>>>><<<<>>>><<>>>><<<<>>>><>><<<>>><<>>>><<<><<<<>><>>>><<<>>><>>>><<<<><<>><>><>>>><<<<>>><<<<>>>><><>><<<>><><<<><<>><<<<>>>><<<>>>><<<>><><<<<><<<<>>>><<>>><<<<>><<<>><><<<<><<><<<>>><>><<<<>><>>>><<<<><<>>><<<>>>><<<<>>>><<<<><>><<<>><>>>><<<<>>>><<>>><<<<>>>><>><>>>><>>><<<>>>><<<>>><>>>><<<<>>><<<>>><<>><<<<>><<><>><<<<>>>><<>>>><<>><<>>><<<<>><<>>><<<<>>><><>>><<<>><<<>>><<>>><>>>><>>><<>>>><<<<><>>>><<<<><<<>>>><>>><<<>>><<<<>>>><<<<>>><>>><<<><>>>><<<>>><>>><<><<>>>><<><<<<>><<<><<<<>><>><<<<>><>>>><<>>><<<>><<<>>>><>>>><<<<>>>><><<<<>>>><<<<>><<<>>>><<<>><<<<>>>><<>>><>>><<<<>>><>>><><<<<>>>><<<>>><<<<>>>><>>>><<<<>><<<<>><<<>>><<<<>>><<<<>>>><<>><>>>><<<>><<<><>>>><<<<>><<>><<<<>><>>><>>>><<<><<<<>><>><<<<>><<<><<<><><<>>><>>><<<<>><><<<><<<<>>>><>><<<<>>>><<<<>>><<>><<>>>><<<<><<<><<<<>><<<<>>><><<><<<><>>><<<>>>><<<>>><<<<>><<>><<<<>>>><>><><<<><><>><<<>><<<<>>>><<<>>>><<<<>>>><<>>>><>><<<>>><<<>>><<>>><<<<>>><><<>>>><<<>>><>>>><<<>><<<<>><<<>>><<><<><<>><<<><<<>><<<>>><<>><<<<>><<><<<<><<<<>>>><>>>><>><<>>><<<<>>><<>><><>><<<>><<>>><<<<>><><>>>><<<>>>><<><<<><<><<<<>>><<<>><<<>><<<>>>><<>>>><<<<>><<<<>><<<<><<><<>>><<<>>><<>>><<<>>><<<<><<<<>>><<<><<<>>><<<>><<<>><<<<>><<<>><>>>><>><<<<>>>><<<<>>><><<<>>><<>>><><>><<>>><<<><<<>>><<<>><>>><<<>>>><<<>><<<<><<<>>><<<<><<<>>>><><><<<>><><<<><<<<>>><<><<<<><<<>>><<<<><<<<>>><<>>><<>>>><<>>>><<>>><><<<><<<<>><<<<>><<<>><<<<>>><<>>><<<<>><<<<>>>><<>>><<<<>>>><<><<>><<<>><<<<>><<<<>>>><<>>>><<<>>>><<<<>>><>><>>><<<<>>><<>>><<<>>>><<<<>>><<<<>><<>>>><<>>><<<>>>><<<>><<<><<<>>>><<<<>>>><<<>><>><<>>><<<<>><>>>><<><<<>>>><<>>>><<<<>>><<<>><<<<>>><<<<>>>><<<<><<>>>><>><>>><<<>>>><>>>><>>>><<<><<>><<><<>><<<<><<>>><><<>>>><<<<><<<<><<<>><<<>><>>><<><<<><>>>><>><<<>>><<>>><<<<>>>><<<>><<<<>>><<>><<<>><<<<>><>>>><>><<<>>><<<<>>>><<>><<<><<>><<<>><<<<><<<>><<>><<<>>>><<<<>>><<<>><<<><<>>>><<>>>><<<<>><>>>><<<>>><<<><>><<<>>>><>>>><<<<>>>><<<>><>>>><<<<>><<<<>><<<<>><<>><><<<>><<<>>><<><<>><<<<>>>><<>>><><>><>><<>>>><<<<><<>>><<<<>>>><<<>>><<>><<><>><<<<>>><<>><<>>><>>>><<<<>><>><<<><<>>>><><<<><><<>>>><><<>>><<<>>><>>>><<><<>><><><<<<>><<<<><<>>>><>>><<>>><<>>><<<>>><><<>>>><<>>><<<>>>><<<>>><>><<<<><>>>><<<>>><<<>>><<><<>><<<<>>><<<<>><<>>>><>>>><<>><<<<><<<<>><<<<><<<>>>><<><<<<>>>><<<<>>>><>>>><<>><<><<<>>>><<<><<>><<<<>>><<<><>>><>><<<>><><>><<<<>><<<<>>>><<<<><<>>>><>>>><<>>><<<<>><>>><<<>>><<><<>>><>><<>>><><<>>>><>><<<>>>><<>><<>>><<<>>>><><<><<><<>><<><<<<>>>><<<>>><>><<<<>>>><<<<>>>><>>><<<>><<>>>><<>><>>>><>>><<>>>><<<><>><<<>><<<<>><<>><<<>>><<<>>><<<<>><>>>><<<><<<><<>><<<<><<<>><><<><<<<><<<>>>><<<><>><<<><<<>><>>>><><<<<>>><<>>>><<>>><>>>><<>>><<<>>><>><<<<>>><><<><<<<>><>>>><<>>>><>>>><<><>><<<<>><<<<>>><><<<>><<><<<><<<>>><<><<<><<><><>>><<<<>><<><>>>><<<>>>><<<<>><<<<>>><>>>><<>>><<<<>><>>>><<<<>><>>><<><>>><<<><>>>><<<<>>><>><<<<>>>><<<>>>><<>>><>>><><<><<<<>><<<><>>>><>>>><<<>>><>>>><<<><<<>><><>><<<><>>><>><>><>>><<<<>>><<<<>><<<<>>>><<<>><<<>>>><<>>>><<>>>><><<<>>>><<>><<<>><<>>>><<<>>><<>>><<<><<<>><>>><>><<<<>><<<>><>>><<<<>><>>>><>><<><<<<>><>><<<<><<>>><<<><>>><<<><<><<<>><<<<>>><<<>>>><<<<>><<>>><>><><<>>><>><<<>><<>>><<>>>><<>>><<>>>><>>><>>><<>><<>>>><<<><<<<>>><<<>>><>>>><<<>>><<>><><<>>>><<<<>>><<><<>>>><>><>>><<<<><<<<>><<>><<<<>><>>>><>><<<>>>><<>><<<>>><<<>>>><<<>><<<><<<<>><<>>>><<<>>>><<<><>>><<>><<<<>>>><<>><<>>><><<<<><<<<><<>>>><<<>>>><<<<><<>>>><<<<>><>><<<>><<>><<<<>>><<>>><<<>><>>><<><<<>>><>>><><<<><<<<>>>><<<<>>>><<<<>>><<>>><<<>>>><<<>><<<<>>><<<<>>>><<<>>><<<>>><<<<><>>><>><<>>>><>>>><<<<>><<>><>>>><<>>>><<<<>>><<<><<<<>>>><<<>>>><>>>><<>>>><<<<>>>><<><<>>>><<<><<<>><<<>><<>>>><<<>>>><<<>><<<<>>>><>>>><<<><>>>><<>>><<<<>><><<<><>>><<>>><<<<>>><<<<>>><>>>><<>>>><<>><<<<><<<>>><<>>>><<<><<<>>><<>><<<>><<>><<><>><<<><<><<<>>>><><<><>>>><<<>>><><<<>>>><<<>><<<<><<<><>>><>>>><<<><<>><>>><<>><<<<>>><<<><<<>>>><<><<>>><<>><>>>><<<<>>><>>><<><<<>>>><><<<>>><><>>><>>><<<<><<<<><<<<>>>><>>>><<<<>><<<><<>>>><>>>><>>><><<<>>><<<<>><<>><<>>>><<>><><>><>><<<<>><<<>>>><<<><<<<><<<<><<<>>>><<><<<<>>>><<<>><>>>><>><<<>>><<<>>><<<<><<<<><<<>><<<<>><>><<<>><<<<>>><<<<><><<<><<>>>><<><<<<>>>><<<><>>>><<><<<<><>>><<<>>><<<<>>><<>><>><<<<>>><<>><<<<>><<<>><>><<<<><>>>><<<>><>>>><<<<><<<>><<>><<>><<>>><<<><<>>><<<>>><<<<>>>><><>>><<<<>>><>>><<<>><>><<<<>>><<>>><<><<><><<<><<>><<<>>>><<>>><>>>><>>><<>>>><<<<>><<>>><<<><<<>><<<<>><<><>><><<><>><<>>>><<<>>>><<<<>><<<<>><<<>><<<<><<>>><<<<>>>><<<>><<<<>>>><>>>><<<<><>><>><<<<>>><<>>>><<<<>><>><>>>><<<<><<>>>><<>><<>>>><<<<>>><<<<><>><>>><<<<>>>><<<>>><<><<><<<<>><<<>>>><>><<>><><<<<>>><<<>>>><<<<>>>><>>><<<<>>><<>><>>>><<>>><>>>><<<<><<>>>><<>>><<>><<<<>>>><<><<<><><<<>>><>>>><<<>>>><<<>>>><<><<>>>><<<><<>>><<><<><<>>>><<>>><<>>><<<<>><<<>>>><<<>>>><<<><<<>>><<<><<<>><<<<><<<><<>>>><>>>><<<<>>>><<>>>><<<<><<<>>>><<<>>>><><>>><<>>>><<<>>><<<<>>><>>>><<>><>><><<<>><>><<><<<<>>>><<<>><<<<>><><>>>><><<<<><><<<<>>><<<>>><<<<>>><<<<>>><>>><<<>>><<<<>>><<<>>><<>>><<>>>><<<>>><<<<>>>><<<<><><<<>>><<<><<<<>><<<<>>><<<>>>><<<<><<<>>><<>>><<<<><>>><<><<<<>>>><<<<>>><>>><<<<>><<<<><><>><>><<<<>>><<<<>><<<<>>><>>>><<<<>>><<<>>><<>>><<><<>>>><>><>>>><>><>><>>><<><<><><<<>>><<><<>>>><<<>>><<<><>>><<<>>>><<<<>>><>>>><>>><<<><<><>>>><<><<<>><<>><<<>><<<<>><<>>>><<<<><<<><>>>><<><<>><<<<>><<>>><<>><<<<>>>><<<>>>><<<>>><<<>>>><<>><<<<><<<><<><<<>><<<<>>><<<>><>>>><<<<><<<<>>><>>>><<<>><<>>>><>>><<<>><<<<>>>><<>><<><>>><<<<><><<<>><<<>><<<<>>>><<<><>>>><<><<<><<<<>>><><><<<>><<<>><<<<>>><<<>>><<<<><<<>>>><>><<><<>>>><<<>>>><<>>>><<<<>>><<>>>><<<>><<<><<<>>><<>>><<<<><<<>>>><<<>>><<<<>>><>><<<>>><<>>><<<>>><<>><<<<>><>>><<<>><>><>>><<<><>>>><<<<>>><<<<><>>><>>><<<>>><<<>>><<>>><<<>><<<>><<<<><><><><<<>>>><<<<>>><<><>>><>>>><<>>><<<<>>><<<><>><<<>>><>>>><>>><<>><<<<>>><<<><><<<<>>><<<>>>><<>>><<<<><><<>>><<<<>><<>>>><<<>>><<<<>>><<>>><<<>>><<<<>>><<<<>>><<><<><<<>>><<<>>>><><<>>>><<<>><<>>><<<>>>><<<><<>><>>>><<<>><<>><<>>>><<<><><<<>><<>><<><<<><>>><<><<<<>>>><>>><>>><<><<<>><<><<<>>>><<<>>><><<<<>><<<<><<<<>>><<<<>><<>><<<<><<<<>>><<<<>>>><<>>><<<>>><<>>><<<>>><<<>><<>>><>>><<<>><<>><><>><>>><<>><<><>>>><>>>><<>>>><<>>><<<>>><<<<>>>><<<<>>>><<<<>>><<><>>>><>><<>>>><<<<>>>><<>>><<<<>><<<>>><<<<>><>>>><<<<>><<<><<><<<<>"
    //let mut jet = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"
        .chars()
        .enumerate()
        .cycle();
    fn add_shape(shape: &HashSet<(i64,i64)>,tall:i64) -> HashSet<(i64,i64)> {
        shape.iter().map(|(x,y)| (x+3,y+tall+4)).collect()
    }
    fn slide(shape: &HashSet<(i64,i64)>,direct:(usize,char)) -> (HashSet<(i64,i64)>,usize) {
        match direct.1 {
            '>' => (shape.iter().map(|(x,y)| (x+1,*y)).collect(),direct.0 ),
            '<' => (shape.iter().map(|(x,y)| (x-1,*y)).collect(),direct.0 ),
            _ => (shape.iter().map(|(x,y)| (*x,y-1)).collect(),0),
        }
    }
    fn touch_any(shape: &HashSet<(i64,i64)>, chamber: &HashSet<(i64,i64)>) -> bool {
        shape.iter().any(|(x,_)| *x == 0 || *x == 8) ||
        shape.iter().any(|(x,y)| chamber.contains(&(*x,*y)))
    }
    let mut chamber:HashSet<(i64,i64)> = (0..8).map(|x| (x,0)).collect();
    let mut seen: HashMap<usize,(i64,i64)> = HashMap::new();
    let mut tall = 0_i64;
    let mut jump = 0_i64;
    let mut shape_iter = shapes.iter().cycle();
    let mut i = 1_i64;
    let mut ready = true;
    while i <= rocks {
        let shape = shape_iter.next().unwrap();
        let mut cur = add_shape(shape, tall);
        let dir_ind:usize;
        loop {
            let (nxt,direct_ind) = slide(&cur,jet.next().unwrap());
            if !touch_any(&nxt, &chamber) {
                cur = nxt;
            }
            let (nxt,_) = slide(&cur,(0,'d'));
            if touch_any(&nxt, &chamber) {
                for (x,y) in cur {
                    chamber.insert((x,y));
                    tall = tall.max(y);
                }
                dir_ind = direct_ind;
                break
            } else {
                cur = nxt;
            }
        }
        if ready && (i % 5 == 1) {
            if let Some((old_i,old_tall)) = seen.insert(dir_ind,(i,tall)) {
                let i_diff = i - old_i;
                let tall_diff = tall - old_tall;
                let blocks = (rocks - i) / i_diff;
                i += blocks * i_diff;
                jump = blocks * tall_diff;
                ready = false;
            }
        }
        i += 1;
    }
    tall + jump 
}
#[allow(dead_code)]
fn p18_x(check:bool) -> i32 {
    //part 1 p18_x(false)
    //part 2 p18_x(true)
    //3254 is too high
    use regex::Regex;
    use std::collections::HashSet;
    let re = Regex::new(r"([^,]+),([^,]+),([^,]+)").unwrap();
    let mut data:HashSet<(i32,i32,i32)> = HashSet::new();
    let lines = io::BufReader::new(File::open("data/p18_1.txt").expect("file not found")).lines();
    for line in lines.filter_map(|line| line.ok()) {
        for triple in re.captures_iter(line.as_str()) {
            data.insert((triple[1].parse::<i32>().unwrap(),triple[2].parse::<i32>().unwrap(),triple[3].parse::<i32>().unwrap()));
        }
    }
    //println!("{:?}",data);
    let mut seen = HashSet::new();
    let mut ans = 0_i32;
    let mut bubles = HashSet::new();
    let mut exterior = HashSet::new();
    for (a,b,c) in data.clone() {
        ans += 6;
        for (x,y,z) in [(a-1,b,c),(a+1,b,c),(a,b-1,c),(a,b+1,c),(a,b,c-1),(a,b,c+1)] {
            if seen.contains(&(x,y,z)) {
                ans -= 2;
            } else if check && !data.contains(&(x,y,z)) && is_interior((x,y,z),&data, &mut bubles, &mut exterior) {
                ans -= 1;
            }
        }
        seen.insert((a,b,c));
    } 
    fn is_interior(air:(i32,i32,i32), data: &HashSet<(i32,i32,i32)>, 
        bubles: &mut HashSet<(i32,i32,i32)>,
        exterior: &mut HashSet<(i32,i32,i32)>) -> bool {
        //if air is inside then we can only expand it so far
        let tol = 20;
        if bubles.contains(&air) { return true }
        if exterior.contains(&air) { return false }
        let mut level = HashSet::from([air]);
        let mut seen = HashSet::new();
        for _ in 0..tol {
            let mut new_level = HashSet::new();
            for (a,b,c) in level {
                seen.insert((a,b,c));
                for pt in [(a-1,b,c),(a+1,b,c),(a,b-1,c),(a,b+1,c),(a,b,c-1),(a,b,c+1)] {
                    if !seen.contains(&pt) && !data.contains(&pt) {
                        new_level.insert(pt);
                    }
                }
            }
            if new_level.is_empty() {
                //println!("{:?}",air); 
                for pt in seen {
                    bubles.insert(pt);
                }
                return true
            }
            level = new_level;
        }
        for pt in seen { exterior.insert(pt); }
        for pt in level { exterior.insert(pt); }
        false
    }
    ans
}
#[allow(dead_code)]
fn p19_x(part1:bool) -> i32 {
    //part 1 p19_x(true)
    //part 2 p19_x(false)
    use regex::Regex;
    use std::collections::HashSet;
    let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    let mut data:Vec<Vec<i32>> = Vec::new();
    let lines = io::BufReader::new(File::open("data/p19_1.txt").expect("file not found")).lines();
    for line in lines.filter_map(|line| line.ok()) {
        let nums = re.captures(line.as_str()).unwrap();
        data.push(nums.iter().skip(1).map(|num| num.unwrap().as_str().parse::<i32>().unwrap()).collect());
    }
    println!("{:?}",data);
    fn bfs(steps:i32, cost: (i32,i32,i32,i32,i32,i32)) -> i32 {
        let state = (steps,0,0,0,0,1,0,0,0);
        let mut seen = HashSet::new();
        let mut stack = vec![state.clone()];
        let mut best = 0_i32;
        while !stack.is_empty() {
            let (steps, ore, clay, obsidian, geode, ore_bot, clay_bot, obsidian_bot, geode_bot) = stack.pop().unwrap();
            if steps == 0 {best = best.max(geode); continue}
            if geode + ((2 * geode_bot + steps) * steps ) / 2 <= best {continue} //remove if even making a new geode bot every step does not make more than current best 
            if seen.contains(&(steps, ore, clay, obsidian, geode, ore_bot, clay_bot, obsidian_bot, geode_bot)) { 
                continue
            } else {
                seen.insert((steps, ore, clay, obsidian, geode, ore_bot, clay_bot, obsidian_bot, geode_bot));
            }
            stack.push((steps -1, ore + ore_bot, clay + clay_bot, obsidian + obsidian_bot, geode + geode_bot, ore_bot, clay_bot, obsidian_bot, geode_bot));
            if ore >= cost.0 {
                stack.push((steps -1, ore -cost.0 + ore_bot, clay + clay_bot, obsidian + obsidian_bot, geode + geode_bot, ore_bot + 1, clay_bot, obsidian_bot, geode_bot));
            }
            if ore >= cost.1 {
                stack.push((steps -1, ore -cost.1 + ore_bot, clay + clay_bot, obsidian + obsidian_bot, geode + geode_bot, ore_bot , clay_bot + 1, obsidian_bot, geode_bot));
            }
            if ore >= cost.2 && clay >= cost.3 {
                stack.push((steps -1, ore -cost.2 + ore_bot, clay -cost.3 + clay_bot, obsidian + obsidian_bot, geode + geode_bot, ore_bot , clay_bot, obsidian_bot + 1, geode_bot));
            }
            if ore >= cost.4 && obsidian >= cost.5 {
                stack.push((steps -1, ore -cost.4 + ore_bot, clay + clay_bot, obsidian - cost.5 + obsidian_bot, geode + geode_bot, ore_bot , clay_bot, obsidian_bot, geode_bot + 1));
            }
        }
        best
    }
    if part1 {
        data.iter().map(|d| d[0] * bfs(24, (d[1],d[2],d[3],d[4],d[5],d[6]))).sum()
    } else {
        bfs(32, (data[0][1],data[0][2],data[0][3],data[0][4],data[0][5],data[0][6])) *
        bfs(32, (data[1][1],data[1][2],data[1][3],data[1][4],data[1][5],data[1][6])) *
        bfs(32, (data[2][1],data[2][2],data[2][3],data[2][4],data[2][5],data[2][6])) 
    }
}
fn main() {
    println!("part 2 {}", p19_x(false));
}
