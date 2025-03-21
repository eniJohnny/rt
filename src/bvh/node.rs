use std::io::Write;

use crate::{model::{scene::Scene, shapes::aabb::Aabb}, BVH_SPLIT_STEPS};

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
        let parent_children = self.aabb.get_children_and_shrink(scene, &(0..scene.elements().len()).collect());
        println!("-------- BUILDING BVH  --------");
        println!("-  This may take a long time  -");
        println!("-    Depending on the scene   -");
        self.set_elements(parent_children);
        self.build_node(scene, 0, 0., 100.);
        println!("- │████████████████████│ 100%  -");
        println!("---------- BVH BUILT ----------");
    }

    fn build_node(&mut self, scene: &Scene, depth: usize, mut completion: f64, mut increment: f64) {
        increment /= 10.;
        let (node_a, node_b) = self.split_node(scene, depth, completion, increment * 4.);
        completion += increment * 4.;
        if node_a.is_none() && node_b.is_none() {
            self.set_is_leaf(true);
        }
        if let Some(mut node_a) = node_a {
            node_a.build_node(scene, depth + 1, completion, increment * 3.);
            completion += increment * 3.;
            self.a = Some(Box::new(node_a));
        }
        if let Some(mut node_b) = node_b {
            node_b.build_node(scene, depth + 1, completion, increment * 3.);
            self.b = Some(Box::new(node_b));
        }
    }

    pub fn split_node(&mut self, scene: &Scene, _depth: usize, mut completion: f64, mut increment: f64) -> (Option<Node>, Option<Node>) {
        let mut current_completion = completion;
        increment /= 3. * BVH_SPLIT_STEPS as f64;
        let t_vec = get_t_vec(BVH_SPLIT_STEPS);

        let mut best_configuration: Option<(f64, (Node, Node, Vec<usize>))> = None;

        for axis in 0..3 {
            for t in &t_vec {
                let (mut aabb1_tmp, mut aabb2_tmp) = self.aabb.split_aabb(axis, *t);
                
                let aabb1_children = aabb1_tmp.get_children_and_shrink(scene, self.elements());
                let mut elements_left = vec![];
                for child in &self.elements {
                    if !aabb1_children.contains(child) {
                        elements_left.push(*child);
                    }
                }
                let aabb2_children = aabb2_tmp.get_children_and_shrink(scene, &elements_left);

                let mut node_a = Node::new(&aabb1_tmp);
                let mut node_b = Node::new(&aabb2_tmp);
                node_a.set_elements(aabb1_children);
                node_b.set_elements(aabb2_children);

                if let Some((cost, new_parent_child)) = self.try_configuration(&mut node_a, &mut node_b) {
                    if let Some((best_cost, _)) = best_configuration {
                        if cost < best_cost {
                            best_configuration = Some((cost, (node_a, node_b, new_parent_child)));
                        }
                    } else {
                        best_configuration = Some((cost, (node_a, node_b, new_parent_child)));
                    }
                }
                current_completion += increment;
                if current_completion - completion.floor() > 1. {
                    completion = current_completion;
                    print!("\r{}\r", " ".repeat(40));
                    print!("- │");
                    std::io::stdout().flush().unwrap();
                    for _ in 0..(completion / 5.) as usize {
                        print!("█");
                    }
                    for _ in 0..(20. - completion / 5.) as usize {
                        print!(" ");
                    }
                    print!("│ {: >3.0}%  -", completion);
                    write!(std::io::stdout(), "\x1b[?25l").unwrap();
                    std::io::stdout().flush().unwrap();
                }
            }
        }

        if let Some((_, (a, b, new_parent_child))) = best_configuration {
            if new_parent_child.len() == 0 && (a.elements().len() == 0 || b.elements().len() == 0) {
                return (None, None);
            }
            let a = match a.elements().len() == 0 { true => None, false => Some(a) };
            let b = match b.elements().len() == 0 { true => None, false => Some(b) };
            self.set_elements(new_parent_child);
            (a, b)
        } else {
            (None, None)
        }
    }

    pub fn try_configuration(&mut self, a: &mut Node, b: &mut Node) -> Option<(f64, Vec<usize>)> {
        let parent_surface_area = self.aabb.surface_area();
        let initial_cost = self.elements.len() as f64 * parent_surface_area;

        let mut new_parent_children = vec![];
        for child in self.elements() {
            if !a.elements().contains(&child) && !b.elements().contains(&child) {
                new_parent_children.push(*child);
            }
        }

        let aabb1_surface_area = a.aabb.surface_area();
        let aabb2_surface_area = b.aabb.surface_area();


        if a.elements().len() < 1 {
            return None;
        }

        let c1 = aabb1_surface_area * a.elements().len() as f64;
        let c2 = aabb2_surface_area * b.elements().len() as f64;
        let cp = parent_surface_area * new_parent_children.len() as f64;

        if initial_cost <  (c1 + c2 + cp) {
            return None;
        }

        Some((c1 + c2 + cp, new_parent_children))
    }
}

pub fn get_t_vec(steps: usize) -> Vec<f64> {
    let mut t_vec = vec![];

    for i in 0..steps {
        t_vec.push((i as f64 + 1.0) / (steps as f64 + 1.0));
    }

    t_vec
}