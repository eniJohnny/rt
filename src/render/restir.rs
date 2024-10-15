use rand::Rng;

use crate::model::{materials::color::Color, maths::hit::Hit};

#[derive(Debug, Clone)]
pub struct Path<'a> {
    pub hit: Hit<'a>,
    pub reflect: Option<Box<Sample>>,
    pub indirect: Option<Box<Sample>>,
}

#[derive(Debug, Clone)]
pub struct Sample {
    // pub path: Path<'a>,
    pub color: Color,
    pub weight: f64,
}

#[derive(Debug, Clone)]
pub struct PathBucket {
    pub sample: Option<Sample>,
    pub weight: f64,
    pub nbSamples: usize,
}

impl PathBucket {
    pub fn combine(&mut self, rhs: Self) {
        self.weight += rhs.weight;
        self.nbSamples += rhs.nbSamples;
        if rhs.weight < f64::EPSILON {
            return;
        }
        if self.weight < f64::EPSILON {
            self.sample = rhs.sample;
            return;
        }
        let rand: f64 = rand::thread_rng().gen_range((0.)..=(self.weight));
        if rand < rhs.weight {
            self.sample = rhs.sample;
        }
    }

    pub fn add(&mut self, sample: Sample) {
        self.weight += &sample.weight;
        self.nbSamples += 1;
        if sample.weight < f64::EPSILON {
            if self.nbSamples == 1 {
                self.sample = Some(sample);
            }
            return;
        }
        if self.nbSamples == 1 || rand::thread_rng().gen_range((0.)..=(self.weight)) < sample.weight
        {
            self.sample = Some(sample);
        }
    }
}
