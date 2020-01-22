use std::cmp::Ordering;
use std::f32::consts::PI;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        return Point { x, y };
    }

    pub const fn origin() -> Point {
        return Point { x: 0, y: 0 };
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}", self.x.to_string(), ",", self.y.to_string());
    }

    #[allow(dead_code)]
    pub fn from_string(string: String) -> Point {
        let pieces: Vec<i32> = string
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        return Point {
            x: pieces[0],
            y: pieces[1],
        };
    }

    pub fn angle_to(&self, other: &Point) -> f32 {
        let y = self.y - other.y; //reversed because image space is backwards
        let x = other.x - self.x;
        let mut rads = (y as f32).atan2(x as f32);
        if rads < 0f32 {
            rads = ((-1f32 * PI) - rads).abs() + PI;
        }
        rads
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        self.x.cmp(&other.x).then_with(|| self.y.cmp(&other.y))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PointDist {
    pub point: Point,
    pub dist: i32,
}

impl Ord for PointDist {
    fn cmp(&self, other: &PointDist) -> Ordering {
        // inverted comparison on dist for min priority queue (Day 10)
        other
            .dist
            .cmp(&self.dist)
            .then_with(|| self.point.cmp(&other.point))
    }
}

impl PartialOrd for PointDist {
    fn partial_cmp(&self, other: &PointDist) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[allow(dead_code)]
pub struct UTreeNode {
    pub parent: String,
    pub children: Vec<String>,
    pub value: String,
    pub visited: bool,
}

impl UTreeNode {
    pub fn new(parent: String, val: String) -> UTreeNode {
        return UTreeNode {
            parent,
            value: val,
            children: Vec::new(),
            visited: false,
        };
    }
}

pub fn manhattan_distance(a: &Point, b: &Point) -> i32 {
    return (a.x - b.x).abs() + (a.y - b.y).abs();
}

pub fn distance(a: &Point, b: &Point) -> f32 {
    let x = (a.x - b.x).abs();
    let y = (a.y - b.y).abs();
    return ((x * x + y * y) as f32).sqrt();
}

pub fn distance_i32(a: &Point, b: &Point) -> i32 {
    return (distance(a, b) * 1000000f32).floor() as i32;
}

pub fn print_image<T>(img: &Vec<T>, w: usize, h: usize)
where
    T: std::fmt::Display,
{
    for y in 0..h {
        let row_start = w * y;
        let row_end = row_start + w;
        for x in row_start..row_end {
            print!("{}", &img[x]);
        }
        print!("\n")
    }
}

pub fn visual_image(img: &Vec<u8>) -> Vec<char> {
    return img
        .iter()
        .map(|x| match x {
            0 => ' ',
            1 => '#',
            2 => '=',
            3 => '-',
            4 => 'o',
            _ => ' ',
        })
        .collect();
}

pub fn atoi(c: char) -> i64 {
    match c {
        '\n' => 10,
        '\r' => 13,
        ' ' => 32,
        '0' => 48,
        '1' => 49,
        '2' => 50,
        '3' => 51,
        '4' => 52,
        '5' => 53,
        '6' => 54,
        '7' => 55,
        '8' => 56,
        '9' => 57,
        ',' => 44,
        '@' => 64,
        'A' => 65,
        'B' => 66,
        'C' => 67,
        'D' => 68,
        'E' => 69,
        'F' => 70,
        'G' => 71,
        'H' => 72,
        'I' => 73,
        'J' => 74,
        'K' => 75,
        'L' => 76,
        'M' => 77,
        'N' => 78,
        'O' => 79,
        'P' => 80,
        'Q' => 81,
        'R' => 82,
        'S' => 83,
        'T' => 84,
        'U' => 85,
        'V' => 86,
        'W' => 87,
        'X' => 88,
        'Y' => 89,
        'Z' => 90,
        'a' => 97,
        'b' => 98,
        'c' => 99,
        'd' => 100,
        'e' => 101,
        'f' => 102,
        'g' => 103,
        'h' => 104,
        'i' => 105,
        'j' => 106,
        'k' => 107,
        'l' => 108,
        'm' => 109,
        'n' => 110,
        'o' => 111,
        'p' => 112,
        'q' => 113,
        'r' => 114,
        's' => 115,
        't' => 116,
        'u' => 117,
        'v' => 118,
        'w' => 119,
        'x' => 120,
        'y' => 121,
        'z' => 122,
        _ => -1,
    }
}

pub fn itoa(i: i64) -> char {
    match i {
        48 => '0',
        49 => '1',
        50 => '2',
        51 => '3',
        52 => '4',
        53 => '5',
        54 => '6',
        55 => '7',
        56 => '8',
        57 => '9',
        10 => '\n',
        13 => '\r',
        32 => ' ',
        35 => '#',
        46 => '.',
        60 => '<',
        62 => '>',
        64 => '@',
        65 => 'A',
        66 => 'B',
        67 => 'C',
        68 => 'D',
        69 => 'E',
        70 => 'F',
        71 => 'G',
        72 => 'H',
        73 => 'I',
        74 => 'J',
        75 => 'K',
        76 => 'L',
        77 => 'M',
        78 => 'N',
        79 => 'O',
        80 => 'P',
        81 => 'Q',
        82 => 'R',
        83 => 'S',
        84 => 'T',
        85 => 'U',
        86 => 'V',
        87 => 'W',
        88 => 'X',
        89 => 'Y',
        90 => 'Z',
        94 => '^',
        97 => 'a',
        98 => 'b',
        99 => 'c',
        100 => 'd',
        101 => 'e',
        102 => 'f',
        103 => 'g',
        104 => 'h',
        105 => 'i',
        106 => 'j',
        107 => 'k',
        108 => 'l',
        109 => 'm',
        110 => 'n',
        111 => 'o',
        112 => 'p',
        113 => 'q',
        114 => 'r',
        115 => 's',
        116 => 't',
        117 => 'u',
        118 => 'v',
        119 => 'w',
        120 => 'x',
        121 => 'y',
        122 => 'z',
        _ => ' ',
    }
}

pub fn is_alpha(c: char) -> bool {
    let c_int = atoi(c);
    return (c_int >= 65 && c_int <= 90) || (c_int >= 97 && c_int <= 122);
}

pub fn is_lower(c: char) -> bool {
    let c_int = atoi(c);
    return c_int >= 97 && c_int <= 122;
}

pub fn is_upper(c: char) -> bool {
    let c_int = atoi(c);
    return c_int >= 65 && c_int <= 90;
}

#[allow(dead_code)]
pub fn to_upper(c: char) -> char {
    let c_int = atoi(c);
    return itoa(c_int - 32);
}

pub fn to_lower(c: char) -> char {
    let c_int = atoi(c);
    return itoa(c_int + 32);
}

#[allow(dead_code)]
pub fn ascii_image(img: &Vec<i64>) -> String {
    let chars: Vec<char> = img.iter().map(|i| itoa(*i)).collect();
    return chars.into_iter().collect();
}

pub fn get_pixel_at(img: &Vec<u8>, w: i32, p: &Point) -> u8 {
    return img[(w * p.y + p.x) as usize];
}

pub fn set_pixel_at(img: &mut Vec<u8>, w: i32, p: &Point, val: u8) {
    img[(w * p.y + p.x) as usize] = val;
}

pub fn in_bounds(w: usize, h: usize, p: &Point) -> bool {
    p.x >= 0 && p.x < (w as i32) && p.y >= 0 && p.y < (h as i32)
}
