use crate::model::{scene::Scene, shapes::aabb::Aabb};

#[derive(Debug, Clone)]
pub struct Node {
    aabb: Aabb,
    a: Option<Box<Node>>,
    b: Option<Box<Node>>,
    elements: Vec<usize>,
    is_leaf: bool
}

impl Node {
    // Constructor
    pub fn new(aabb: &Aabb) -> Node {
        Node {
            aabb: aabb.clone(),
            a: None,
            b: None,
            elements: vec![],
            is_leaf: false
        }
    }

    // Accessors
    pub fn aabb(&self) -> &Aabb { &self.aabb }
    pub fn a(&self) -> &Option<Box<Node>> { &self.a }
    pub fn b(&self) -> &Option<Box<Node>> { &self.b }
    pub fn elements(&self) -> &Vec<usize> { &self.elements }
    pub fn is_leaf(&self) -> bool { self.is_leaf }
    pub fn node(&self) -> &Node { self }

    // Mutators
    pub fn set_aabb(&mut self, aabb: Aabb) { self.aabb = aabb; }
    pub fn set_a(&mut self, a: Option<Box<Node>>) { self.a = a; }
    pub fn set_b(&mut self, b: Option<Box<Node>>) { self.b = b; }
    pub fn set_elements(&mut self, elements: Vec<usize>) {
        self.elements = elements;
    }
    pub fn set_is_leaf(&mut self, is_leaf: bool) { self.is_leaf = is_leaf; }

    // Methods
    pub fn build_tree(&mut self, scene: &Scene) {
        let mut tree_complete = false;

        while !tree_complete {
            self.build_node(scene);

            if self.a.is_none() && self.b.is_none() {
                self.build_leaf(scene);
            } 

            if self.is_leaf {
                tree_complete = true;
            } else {
                if self.a.is_some() {
                    let a = self.a.as_mut().unwrap();
                    a.build_tree(scene);
                }

                if self.b.is_some() {
                    let b = self.b.as_mut().unwrap();
                    b.build_tree(scene);
                }
            }
        }
    }

    fn build_node(&mut self, scene: &Scene) {
        let children_number = self.aabb().get_children_number(scene) as f64;
        let surface_area = self.aabb().surface_area();
        let cost = children_number * surface_area;

        if split_cost(&self.aabb, scene) > cost {
            self.build_leaf(scene);
            return;
        }

        let (aabb_a, aabb_b) = &self.aabb.better_split(scene);
        let (mut node_a, mut node_b) = (Node::new(&aabb_a), Node::new(&aabb_b));

        let split_cost_a = split_cost(&node_a.aabb, scene);
        let split_cost_b = split_cost(&node_b.aabb, scene);

        if split_cost_a >= 1.0 && split_cost_b >= 1.0 {
            self.build_leaf(scene);
            return;
        }

        if split_cost_a < 1.0 {
            let (aabb_a_a, aabb_a_b) = &node_a.aabb.better_split(scene);
            let (node_a_a, node_a_b) = (Node::new(&aabb_a_a), Node::new(&aabb_a_b));
            node_a.set_a(Some(Box::new(node_a_a)));
            node_a.set_b(Some(Box::new(node_a_b)));
        }

        if split_cost_b < 1.0 {
            let (aabb_b_a, aabb_b_b) = &node_b.aabb.better_split(scene);
            let (node_b_a, node_b_b) = (Node::new(&aabb_b_a), Node::new(&aabb_b_b));
            node_b.set_a(Some(Box::new(node_b_a)));
            node_b.set_b(Some(Box::new(node_b_b)));
        }

    }

    fn build_leaf(&mut self, scene: &Scene) {
        let mut elements = vec![];

        for i in 0..scene.elements().len() {
            let element = scene.elements()[i].shape();
            let element_aabb = element.aabb();
            if element_aabb.is_some() {
                let element_aabb = element_aabb.unwrap();
                let intersection = self.aabb.intersection(element_aabb);

                if intersection.is_some() {
                    elements.push(i);
                }
            }
        }

        self.set_elements(elements);
        self.set_is_leaf(true);
    }

}

pub fn split_cost(aabb: &Aabb, scene: &Scene) -> f64 {
    let mut total_surface_area = 0.0;
    let mut total_volume = 0.0;

    for i in 0..scene.elements().len() {
        let element = scene.elements()[i].shape();
        let element_aabb = element.aabb();
        if element_aabb.is_none() {
            continue;
        } else {
            let element_aabb = element_aabb.unwrap();
            let intersection = aabb.intersection(element_aabb);

            if intersection.is_some() {
                let intersection = intersection.unwrap();
                let intersection_surface_area = intersection.surface_area();
                let intersection_volume = intersection.volume();

                total_surface_area += intersection_surface_area;
                total_volume += intersection_volume;
            }
        }
    }

    let aabb_surface_area = aabb.surface_area();
    let aabb_volume = aabb.volume();

    total_surface_area / aabb_surface_area + total_volume / aabb_volume
}

pub fn split_aabb(aabb: &Aabb, scene: &Scene) -> (Aabb, Aabb) {
    let mut aabb = aabb.clone();

    let mut best_cost = f64::INFINITY;
    let mut best_axis = 0;
    let mut best_position = 0.0;

    for axis in 0..3 {
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;

        for i in 0..scene.elements().len() {
            let element = scene.elements()[i].shape();
            let element_aabb = element.aabb();
            if element_aabb.is_none() {
                continue;
            } else {
                let element_aabb = element_aabb.unwrap();
                let element_min = element_aabb.min()[axis];
                let element_max = element_aabb.max()[axis];

                if element_min < min {
                    min = element_min;
                }

                if element_max > max {
                    max = element_max;
                }
            }
        }

        let step = (max - min) / 10.0;

        for i in 0..10 {
            let position = min + step * i as f64;
            let (aabb_a, aabb_b) = aabb.split(axis, position);

            let cost = split_cost(&aabb_a, scene) + split_cost(&aabb_b, scene);

            if cost < best_cost {
                best_cost = cost;
                best_axis = axis;
                best_position = position;
            }
        }
    }

    println!("Best cost: {}\nBest axis: {}\nBest position: {}", best_cost, best_axis, best_position);
    aabb.split(best_axis, best_position)
}
