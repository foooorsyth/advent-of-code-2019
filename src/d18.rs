use crate::shared::{atoi, in_bounds, is_alpha, is_lower, is_upper, itoa, to_lower, Point};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::ops::RangeInclusive;

pub fn part1() -> Result<usize> {
    let (img, w, h, lut) = read("input/d18_p1.txt")?;
    let cost_of_finding_keys = find_keys(&img, w, h, &lut, atoi('a')..=atoi('z'));
    Ok(cost_of_finding_keys)
}

pub fn part2() -> Result<usize> {
    let (imgs_luts, w, h) = read_quads("input/d18_p2.txt")?;
    let cost_of_finding_keys = find_keys_quads(&imgs_luts, w, h);
    Ok(cost_of_finding_keys)
}

#[derive(Debug, Clone)]
pub struct MapSequence {
    cost: usize,
    keys: HashSet<char>,
}

impl MapSequence {
    fn new() -> MapSequence {
        return MapSequence {
            cost: 0,
            keys: HashSet::new(),
        };
    }

    fn has_all_keys(&self, key_range: RangeInclusive<i64>) -> bool {
        for k in key_range {
            if !self.keys.contains(&itoa(k)) {
                return false;
            }
        }
        return true;
    }

    fn has_all_keys_from_set(&self, key_set: &HashSet<char>) -> bool {
        for k in key_set {
            if !self.keys.contains(&k) {
                return false;
            }
        }
        return true;
    }

    fn consume(&mut self, map_path: &MapPath, inverse: bool) {
        self.cost += map_path.cost;
        let target = if !inverse { map_path.to } else { map_path.from };
        let mut keys_added = 0;
        if !self.keys.contains(&target) {
            self.keys.insert(target);
            keys_added += 1;
        } else {
            panic!("unexpected");
        }
        for (key, _) in &map_path.keys {
            if !self.keys.contains(key) {
                self.keys.insert(*key);
                keys_added += 1;
            }
        }
        assert!(keys_added >= 1);
    }
}

#[derive(Debug, Clone)]
pub struct MapPath {
    from: char,
    to: char,
    cost: usize,
    keys: HashMap<char, usize>,
    doors: HashMap<char, usize>,
}

impl MapPath {
    fn new(from: char, to: char) -> MapPath {
        return MapPath {
            from: from,
            to: to,
            cost: 0,
            keys: HashMap::new(),
            doors: HashMap::new(),
        };
    }
}

pub fn find_keys_quads(
    imgs_luts: &Vec<(Vec<char>, HashMap<char, Point>)>,
    w: usize,
    h: usize,
) -> usize {
    // memoization of all key to key paths
    let mut path_memos = Vec::<HashMap<String, Vec<MapPath>>>::new();
    let mut start_from_to_keys = Vec::<HashSet<String>>::new();
    let mut base_key_sets = Vec::<HashSet<char>>::new();
    let from = 64;
    for quad in 0..4 {
        base_key_sets.push(HashSet::new());
        path_memos.push(HashMap::new());
        start_from_to_keys.push(HashSet::new());
        //println!("{:?}", imgs_luts[quad].1);
        for to in imgs_luts[quad].1.keys() {
            if is_lower(*to) {
                base_key_sets[quad].insert(*to);
                let start_from_to_key = from_to_key(from, atoi(*to));
                start_from_to_keys[quad].insert(start_from_to_key.clone());
                // @ to all keys
                path_memos[quad].insert(
                    start_from_to_key,
                    calc_paths(
                        &imgs_luts[quad].0,
                        w,
                        h,
                        &imgs_luts[quad].1,
                        from,
                        atoi(*to),
                    ),
                );
            }
        }
    }
    for quad in 0..4 {
        let mut indexable_keys: Vec<char> = imgs_luts[quad].1.keys().cloned().map(|x| x).collect();
        indexable_keys.sort();
        for from in 0..indexable_keys.len() {
            if is_lower(indexable_keys[from]) {
                // all keys to all other keys
                for to in (from + 1)..indexable_keys.len() {
                    if is_lower(indexable_keys[to]) {
                        path_memos[quad].insert(
                            from_to_key(atoi(indexable_keys[from]), atoi(indexable_keys[to])),
                            calc_paths(
                                &imgs_luts[quad].0,
                                w,
                                h,
                                &imgs_luts[quad].1,
                                atoi(indexable_keys[from]),
                                atoi(indexable_keys[to]),
                            ),
                        );
                    }
                }
            }
        }
    }
    // another bfs across all paths until all keys are collected
    let mut q = VecDeque::<(MapSequence, MapPath, bool, usize, Vec<char>)>::new();
    let mut positions = Vec::<char>::new();
    for _ in 0..4 {
        positions.push('@');
    }
    // push all paths from start positions
    for quad in 0..4 {
        for start_from_to_key in &start_from_to_keys[quad] {
            let paths = &path_memos[quad][start_from_to_key];
            for path in paths {
                q.push_back((
                    MapSequence::new(),
                    path.clone(),
                    false,
                    quad,
                    positions.clone(),
                ))
            }
        }
    }
    let mut min_sequence = MapSequence::new();
    min_sequence.cost = usize::max_value();
    let mut state_cache = HashMap::<String, usize>::new();
    loop {
        if q.len() == 0 {
            return min_sequence.cost;
        }
        let current = q.pop_front().unwrap();
        let mut current_seq = current.0;
        let current_path = current.1;
        let inverse = current.2;
        let current_quad = current.3;
        positions = current.4;
        positions[current_quad] = if !inverse {
            current_path.to
        } else {
            current_path.from
        };
        current_seq.consume(&current_path, inverse);
        let collected_all_keys = current_seq.has_all_keys_from_set(&base_key_sets[0])
            && current_seq.has_all_keys_from_set(&base_key_sets[1])
            && current_seq.has_all_keys_from_set(&base_key_sets[2])
            && current_seq.has_all_keys_from_set(&base_key_sets[3]);
        if collected_all_keys {
            if current_seq.cost < min_sequence.cost {
                min_sequence = current_seq;
            }
            continue;
        }
        let mut state = String::new();
        for quad in 0..4 {
            state.push(positions[quad]);
        }
        let sorted_keys_str: String = current_seq.keys.iter().collect();
        let mut sorted_keys: Vec<char> = sorted_keys_str.chars().collect();
        sorted_keys.sort();
        for k in sorted_keys {
            state.push(k);
        }
        if state_cache.contains_key(&state) && state_cache[&state] <= current_seq.cost {
            // We've already been here in this state before with an equal or lower cost.
            // No need to do the calculation again.
            continue;
        }
        for quad in 0..4 {
            for destination in &base_key_sets[quad] {
                if !current_seq.keys.contains(&destination) {
                    let mut ft_key = from_to_key(atoi(positions[quad]), atoi(*destination));
                    let mut next_inverse = false;
                    if !path_memos[quad].contains_key(&ft_key) {
                        ft_key = from_to_key(atoi(*destination), atoi(positions[quad]));
                        next_inverse = true;
                    }
                    if path_memos[quad].contains_key(&ft_key) {
                        for next_path in &path_memos[quad][&ft_key] {
                            if is_viable(
                                positions[quad],
                                *destination,
                                &current_seq.keys,
                                next_path,
                            ) {
                                q.push_back((
                                    current_seq.clone(),
                                    next_path.clone(),
                                    next_inverse,
                                    quad,
                                    positions.clone(),
                                ));
                            }
                        }
                    }
                }
            }
        }
        state_cache.insert(state, current_seq.cost);
    }
}

pub fn find_keys(
    img: &Vec<char>,
    w: usize,
    h: usize,
    lut: &HashMap<char, Point>,
    key_range: RangeInclusive<i64>,
) -> usize {
    // memoization of all key to key paths
    let mut path_memo = HashMap::<String, Vec<MapPath>>::new();
    let mut start_from_to_keys = HashSet::new();
    let mut base_key_set = HashSet::<char>::new();
    let from = 64;
    for to in key_range.clone() {
        base_key_set.insert(itoa(to));
        let start_from_to_key = from_to_key(from, to);
        start_from_to_keys.insert(start_from_to_key.clone());
        // @ to all keys
        path_memo.insert(start_from_to_key, calc_paths(&img, w, h, lut, from, to));
    }
    for from in key_range.clone() {
        // all keys to all other keys
        for to in (from + 1)..=*key_range.end() {
            path_memo.insert(from_to_key(from, to), calc_paths(&img, w, h, lut, from, to));
        }
    }
    // another bfs across all paths until all keys are collected
    let mut q = VecDeque::<(MapSequence, MapPath, bool)>::new();
    // push all paths from start position
    for start_from_to_key in start_from_to_keys {
        let paths = &path_memo[&start_from_to_key];
        for path in paths {
            q.push_back((MapSequence::new(), path.clone(), false))
        }
    }
    let mut min_sequence = MapSequence::new();
    min_sequence.cost = usize::max_value();
    let mut state_cache = HashMap::<String, usize>::new();
    loop {
        if q.len() == 0 {
            return min_sequence.cost;
        }
        let current = q.pop_front().unwrap();
        let mut current_seq = current.0;
        let current_path = current.1;
        let inverse = current.2;
        let current_pos = if !inverse {
            current_path.to
        } else {
            current_path.from
        };
        current_seq.consume(&current_path, inverse);
        let collected_all_keys = current_seq.has_all_keys(key_range.clone());
        if collected_all_keys {
            if current_seq.cost < min_sequence.cost {
                min_sequence = current_seq;
            }
            continue;
        }
        let mut state = String::new();
        state.push(current_pos);
        let sorted_keys_str: String = current_seq.keys.iter().collect();
        let mut sorted_keys: Vec<char> = sorted_keys_str.chars().collect();
        sorted_keys.sort();
        for k in sorted_keys {
            state.push(k);
        }
        if state_cache.contains_key(&state) && state_cache[&state] <= current_seq.cost {
            // We've already been here in this state before with an equal or lower cost.
            // No need to do the calculation again.
            continue;
        }
        for destination in &base_key_set {
            if !current_seq.keys.contains(&destination) {
                assert!(current_seq.keys.contains(&current_pos));
                let mut ft_key = from_to_key(atoi(current_pos), atoi(*destination));
                let mut next_inverse = false;
                if !path_memo.contains_key(&ft_key) {
                    ft_key = from_to_key(atoi(*destination), atoi(current_pos));
                    next_inverse = true;
                    assert!(path_memo.contains_key(&ft_key))
                }
                for next_path in &path_memo[&ft_key] {
                    if is_viable(current_pos, *destination, &current_seq.keys, next_path) {
                        q.push_back((current_seq.clone(), next_path.clone(), next_inverse));
                    }
                }
            }
        }
        state_cache.insert(state, current_seq.cost);
    }
}

pub fn is_viable(from: char, to: char, current_keys: &HashSet<char>, path: &MapPath) -> bool {
    let inverse = if from == path.from && to == path.to {
        false
    } else if from == path.to && to == path.from {
        true
    } else {
        panic!("wtf")
    };
    for door in path.doors.keys() {
        let key_on_path = path.keys.contains_key(&to_lower(*door));
        let door_before_key = if key_on_path {
            if !inverse {
                path.doors[door] < path.keys[&to_lower(*door)]
            } else {
                path.doors[door] > path.keys[&to_lower(*door)]
            }
        } else {
            false
        };
        let key_to_door = to_lower(*door);
        let has_key_to_door = current_keys.contains(&key_to_door);
        if (!has_key_to_door && !key_on_path)
            || (!has_key_to_door && key_on_path && door_before_key)
        {
            return false;
        }
    }
    return true;
}

pub fn from_to_key(from: i64, to: i64) -> String {
    let from_c = itoa(from);
    let to_c = itoa(to);
    return from_to_key_c(from_c, to_c);
}

pub fn from_to_key_c(from: char, to: char) -> String {
    let mut from_to_key = String::new();
    from_to_key.push(from);
    from_to_key.push(',');
    from_to_key.push(to);
    return from_to_key;
}

pub fn calc_paths(
    img: &Vec<char>,
    w: usize,
    h: usize,
    lut: &HashMap<char, Point>,
    from: i64,
    to: i64,
) -> Vec<MapPath> {
    let mut res = Vec::new();
    let mut visited = vec![false; w * h];
    let from_c = itoa(from);
    let to_c = itoa(to);
    let from_pt = lut[&from_c];
    let mut q = VecDeque::<(MapPath, Point)>::new();
    q.push_back((MapPath::new(from_c, to_c), from_pt.clone()));
    loop {
        if q.len() == 0 {
            return res;
        }
        let mut current_state = q.pop_front().unwrap();
        // mark current position as visited
        let img_pos = w * (current_state.1.y as usize) + (current_state.1.x as usize);
        visited[img_pos] = true;
        let current_c = img[img_pos];
        // check if it's a door or key, add to mappath
        if is_upper(current_c) {
            if !current_state.0.doors.contains_key(&current_c) {
                current_state
                    .0
                    .doors
                    .insert(current_c, current_state.0.cost);
            }
        }
        if is_lower(current_c) {
            if !current_state.0.keys.contains_key(&current_c) {
                current_state.0.keys.insert(current_c, current_state.0.cost);
            }
        }
        // check if we're at the 'to' pt, if so, add to res and continue
        if current_c == to_c {
            res.push(current_state.0);
            continue;
        }
        // check neighbor pts and push to q if they're not a wall or visited
        let n_test = Point::new(current_state.1.x, current_state.1.y - 1);
        let n_pos = w * (n_test.y as usize) + (n_test.x as usize);
        if in_bounds(w, h, &n_test) && img[n_pos] != '#' && !visited[n_pos] {
            let mut state = current_state.0.clone();
            state.cost += 1;
            q.push_back((state, n_test));
        }
        let s_test = Point::new(current_state.1.x, current_state.1.y + 1);
        let s_pos = w * (s_test.y as usize) + (s_test.x as usize);
        if in_bounds(w, h, &s_test) && img[s_pos] != '#' && !visited[s_pos] {
            let mut state = current_state.0.clone();
            state.cost += 1;
            q.push_back((state, s_test));
        }
        let w_test = Point::new(current_state.1.x - 1, current_state.1.y);
        let w_pos = w * (w_test.y as usize) + (w_test.x as usize);
        if in_bounds(w, h, &w_test) && img[w_pos] != '#' && !visited[w_pos] {
            let mut state = current_state.0.clone();
            state.cost += 1;
            q.push_back((state, w_test));
        }
        let e_test = Point::new(current_state.1.x + 1, current_state.1.y);
        let e_pos = w * (e_test.y as usize) + (e_test.x as usize);
        if in_bounds(w, h, &e_test) && img[e_pos] != '#' && !visited[e_pos] {
            let mut state = current_state.0.clone();
            state.cost += 1;
            q.push_back((state, e_test));
        }
    }
}

pub fn read_quads(
    input: &'static str,
) -> Result<(Vec<(Vec<char>, HashMap<char, Point>)>, usize, usize)> {
    let (img, w, h, _) = read(input).unwrap();
    let mut res_imgs = Vec::<(Vec<char>, HashMap<char, Point>)>::new();

    //q1
    let (q1_img, q1_lut) = read_quad(&img, w, 0, h / 2, w / 2, w - 1);
    res_imgs.push((q1_img, q1_lut));

    //q2
    let (q2_img, q2_lut) = read_quad(&img, w, 0, h / 2, 0, w / 2);
    res_imgs.push((q2_img, q2_lut));

    //q3
    let (q3_img, q3_lut) = read_quad(&img, w, h / 2, h - 1, 0, w / 2);
    res_imgs.push((q3_img, q3_lut));

    //q4
    let (q4_img, q4_lut) = read_quad(&img, w, h / 2, h - 1, w / 2, w - 1);
    res_imgs.push((q4_img, q4_lut));

    return Ok((res_imgs, w / 2 + 1, h / 2 + 1));
}

fn read_quad(
    img: &Vec<char>,
    w: usize,
    y_lo: usize,
    y_hi: usize,
    x_lo: usize,
    x_hi: usize,
) -> (Vec<char>, HashMap<char, Point>) {
    let mut q_img = Vec::<char>::new();
    let mut q_lut = HashMap::<char, Point>::new();
    for y in y_lo..=y_hi {
        for x in x_lo..=x_hi {
            let c = img[y * w + x];
            q_img.push(c);
            if is_alpha(c) || c == '@' {
                q_lut.insert(c, Point::new((x - x_lo) as i32, (y - y_lo) as i32));
            }
        }
    }
    (q_img, q_lut)
}

pub fn read(input: &'static str) -> Result<(Vec<char>, usize, usize, HashMap<char, Point>)> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);
    let mut w = 0;
    let mut h = 0;
    let mut img = Vec::<char>::new();
    let mut lut = HashMap::<char, Point>::new();
    for (y_index, line) in reader.lines().enumerate() {
        let line = line.unwrap().to_string();
        w = line.len();
        h = y_index + 1;
        for (x_index, c) in line.chars().enumerate() {
            img.push(c);
            if is_alpha(c) || c == '@' {
                lut.insert(c, Point::new(x_index as i32, y_index as i32));
            }
        }
    }
    Ok((img, w, h, lut))
}
