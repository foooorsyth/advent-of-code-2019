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
