// Data structures

#[derive(Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
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
        let y = other.y - self.y;
        let x = other.x - self.x;
        (y as f32).atan2(x as f32)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
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
            parent: parent,
            value: val,
            children: Vec::new(),
            visited: false,
        };
    }
}

pub fn manhattan_distance(a: &Point, b: &Point) -> i32 {
    return (a.x - b.x).abs() + (a.y - b.y).abs();
}
