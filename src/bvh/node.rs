use crate::model::{scene::Scene, shapes::aabb::Aabb};

#[derive(Debug)]
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

    // Mutators
    pub fn set_aabb(&mut self, aabb: Aabb) { self.aabb = aabb; }
    pub fn set_left(&mut self, left: Option<Box<Node>>) { self.left = left; }
    pub fn set_right(&mut self, right: Option<Box<Node>>) { self.right = right; }
    pub fn set_elements(&mut self, elements: Vec<usize>) {
        self.len = elements.len();
        self.elements = elements;
    }

    // Methods
    pub fn add_element(&mut self, element: usize) {
        self.elements.push(element);
        self.len += 1;
    }
    pub fn add_elements(&mut self, elements: Vec<usize>) {
        self.len += elements.len();
        self.elements.extend(elements);
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