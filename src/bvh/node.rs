use std::{fmt, io::Write};

use crate::model::{maths::vec3::Vec3, scene::Scene, shapes::aabb::{self, Aabb}};

#[derive(Debug, Clone)]
pub struct Node {
    aabb: Aabb,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    elements: Vec<usize>,
    len: usize,
}

impl Node {
    // Constructor
    pub fn new(aabb: &Aabb) -> Node {
        Node {
            aabb: aabb.clone(),
            left: None,
            right: None,
            elements: vec![],
            len: 0,
        }
    }

    // Accessors
    pub fn aabb(&self) -> &Aabb { &self.aabb }
    pub fn left(&self) -> &Option<Box<Node>> { &self.left }
    pub fn right(&self) -> &Option<Box<Node>> { &self.right }
    pub fn elements(&self) -> &Vec<usize> { &self.elements }
    pub fn len(&self) -> usize { self.len }
    pub fn node(&self) -> &Node { self }

    // Mutators
    pub fn set_aabb(&mut self, aabb: Aabb) { self.aabb = aabb; }
    pub fn set_left(&mut self, left: Option<Box<Node>>) { self.left = left; }
    pub fn set_right(&mut self, right: Option<Box<Node>>) { self.right = right; }
    pub fn set_elements(&mut self, elements: Vec<usize>) {
        self.len = elements.len();
        self.elements = elements;
    }

    // Methods
    pub fn split(&mut self, scene: &mut Scene) {
        self.add_node(scene);

        let children_nb = self.aabb().get_children_aabbs_id(&scene).len();

        if children_nb <= 1 {
            return;
        }

        if self.is_leaf() == false {
            self.left.as_mut().unwrap().split(scene);
            self.right.as_mut().unwrap().split(scene);
        } else {
            // if self.left.is_none() && self.right.is_some() {
            //     self.right.as_mut().unwrap().set_children_elements(scene);
            // } else if self.right.is_none() && self.left.is_some() {
            //     self.left.as_mut().unwrap().set_children_elements(scene);
            // }
            self.set_children_elements(scene);
        }
    }

    pub fn set_children_elements(&mut self, scene: &mut Scene) {
        let aabb = self.aabb();
        let elements = aabb.get_children_aabbs_id(scene);
        self.set_elements(elements);
    }

    pub fn add_node(&mut self, scene: &mut Scene) {
        let mut aabb = self.aabb().clone();
        let children = aabb.get_children_aabbs_id(scene);

        // if children.len() < 1 {
        //     return;
        // }

        let (aabb1, aabb2) = aabb.split(Vec3::new(1., 0., 0.), 0.5);
        let left_children = aabb1.get_children_elements(scene);
        let right_children = aabb2.get_children_elements(scene);

        if left_children == right_children || left_children.len() == 0 || right_children.len() == 0 {
            self.set_elements(children);
            return;
        }

        self.set_left(Some(Box::new(Node::new(&aabb1))));
        self.set_right(Some(Box::new(Node::new(&aabb2))));
    }

    pub fn add_element(&mut self, element: usize) {
        self.elements.push(element);
        self.len += 1;
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() || self.right.is_none()
    }

    fn format_node(&self, indent: &str, last: bool) -> String {
        let mut result = String::new();
        result.push_str(indent);
    
        if !indent.is_empty() {
            if last {
                result.push_str("└── ");
            } else {
                result.push_str("├── ");
            }
    
            result.push_str("Node\n");
        }
    
        let new_indent = if last {
            format!("{}    ", indent)
        } else {
            format!("{}│   ", indent)
        };
    
        if let Some(ref left) = self.left {
            result.push_str(&format!("{}├── Left:\n{}", new_indent, left.format_node(&format!("{}│   ", new_indent), true)));
        }
    
        if let Some(ref right) = self.right {
            result.push_str(&format!("{}└── Right:\n{}", new_indent, right.format_node(&format!("{}    ", new_indent), true)));
        }
    
        if !self.elements.is_empty() {
            result.push_str(&format!("{}    └── Elements: {:?}\n", indent, self.elements));
        }
    
        result
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.format_node("", true))
    }
}


// TESTS
pub fn test_node(scene: &mut Scene) {
    let aabbs = scene.all_aabb();
    let mut node = Node::new(aabbs.first().expect("No AABBs in scene"));

    let elements = scene.elements();
    for (i, _) in elements.iter().enumerate() {
            node.add_element(i);
    }

    // dbg!(&node);

    for i in node.elements() {
        dbg!(&elements[*i]);
    }
}

pub fn test_node_insertion(scene: &mut Scene) {
    let aabbs = scene.all_aabb();
    let aabb = Aabb::from_aabbs(&aabbs);

    let mut node = Node::new(&aabb);
    node.split(scene);


    let logfile = "node.log";
    let mut file = std::fs::File::create(logfile).expect("Unable to create file");
    let elements = scene.elements();
    file.write_all(format!("{}", node).as_bytes()).expect("Unable to write data");
    for (i, elem) in elements.iter().enumerate() {
        file.write_all(format!("\n{} : {:?}", i, elem.shape()).as_bytes()).expect("Unable to write data");
    }

    println!("Node written to logfile");
}