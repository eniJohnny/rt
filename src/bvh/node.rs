use crate::{model::{scene::Scene, shapes::aabb::Aabb}, AABB_STEPS_NB};

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
        let parent_children = self.aabb.get_children_and_shrink(scene);
        self.set_elements(parent_children);

        self.build_node(scene, 1);
    }

    fn build_node(&mut self, scene: &Scene, depth: usize) {
        if let Some((mut node_a, mut node_b)) = self.split_node(scene) {
            node_a.build_node(scene, depth + 1);
            node_b.build_node(scene, depth + 1);
            self.a = Some(Box::new(node_a));
            self.b = Some(Box::new(node_b));
        } else {
            self.set_is_leaf(true);
        }
    }

    pub fn split_node(&mut self, scene: &Scene) -> Option<(Node, Node)> {
        let t_vec = get_t_vec(AABB_STEPS_NB);

        let mut best_configuration: Option<(f64, (Node, Node, Vec<usize>))> = None;

        for axis in 0..2 {
            for t in &t_vec {
                let (mut aabb1_tmp, mut aabb2_tmp) = self.aabb.split_aabb(axis, *t);
                
                let aabb1_children = aabb1_tmp.get_children_and_shrink(scene);
                let aabb2_children = aabb2_tmp.get_children_and_shrink(scene);

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
                
            }
        }

        if let Some((_, (a, b, new_parent_child))) = best_configuration {
            self.set_elements(new_parent_child);
            Some((a, b))
        } else {
            None
        }
    }

    pub fn try_configuration(&mut self, a: &mut Node, b: &mut Node) -> Option<(f64, Vec<usize>)> {
        let parent_volume = self.aabb.surface_area();
        let initial_cost = self.elements.len() as f64 * parent_volume;

        let mut new_parent_children = vec![];
        for child in self.elements() {
            if !a.elements().contains(&child) && !b.elements().contains(&child) {
                new_parent_children.push(*child);
            }
        }

        if a.elements().len() < 1 && b.elements().len() < 1 {
            return None;
        }

        let aabb1_volume = a.aabb.surface_area();
        let aabb2_volume = b.aabb.surface_area();

        if aabb1_volume < f64::EPSILON || aabb1_volume < f64::EPSILON {
            return None;
        }
        

        let c1 = aabb1_volume * a.elements().len() as f64;
        let c2 = aabb2_volume * b.elements().len() as f64;
        let cp = parent_volume * new_parent_children.len() as f64;

        if initial_cost >  c1 + c2 + cp {
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
