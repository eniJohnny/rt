use crate::model::maths::hit::Hit;


#[derive(Debug, Clone)]
pub struct PathBucket<'a> {
    pub weight: f64,
    pub path: Vec<Hit<'a>>,
    pub pathWeight: f64,
    pub nbElements: usize
}

impl<'a> PathBucket<'a> {
    pub fn combine(&mut self, rhs: Self) {
        if self.weight + weight == 0. {
            return;
        }
        let rand: f64 = rand::thread_rng().gen_range((0.)..(self.weight + rhs.weight));
        if rand > self.weight {
            self.path = rhs.path;
            self.pathWeight = rhs.pathWeight;
        }
        self.weight += rhs.weight;
        self.nbElements += rhs.nbElements;
    }

    pub fn add(&'a mut self, path: Vec<Hit<'a>>, weight: f64) {
        if self.weight + weight == 0. {
            return;
        }
        let rand: f64 = rand::thread_rng().gen_range((0.)..(self.weight + weight));
        if rand > self.weight {
            self.path = path;
            self.pathWeight = weight;
        }
        self.weight += weight;
        self.nbElements += 1;
    }
}


/*

1. Diffuse + Perfect Reflect

2. Bucket -> Indirect light + Rough reflect





*/

