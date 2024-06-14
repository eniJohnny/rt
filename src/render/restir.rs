use rand::Rng;

use crate::model::{materials::color::Color, maths::hit::Hit};

#[derive(Debug, Clone)]
pub struct Path<'a> {
    pub hit: Hit<'a>,
    pub reflect: Option<Box<Sample<'a>>>,
    pub indirect: Option<Box<Sample<'a>>>,
}

#[derive(Debug, Clone)]
pub struct Sample<'a> {
    pub path: Path<'a>,
    pub color: Color,
    pub weight: f64,
}

#[derive(Debug, Clone)]
pub struct PathBucket<'a> {
    pub sample: Option<Sample<'a>>,
    pub weight: f64,
    pub nbSamples: usize,
}

impl<'a> PathBucket<'a> {
    pub fn combine(&mut self, rhs: Self) {
        self.weight += rhs.weight;
        self.nbSamples += rhs.nbSamples;
        if self.weight < f64::EPSILON {
            self.sample = rhs.sample;
            return;
        }
        let rand: f64 = rand::thread_rng().gen_range((0.)..=(self.weight));
        if rand < rhs.weight {
            self.sample = rhs.sample;
        }
    }

    pub fn add(&mut self, sample: Sample<'a>) {
        self.weight += &sample.weight;
        self.nbSamples += 1;
        if sample.weight < f64::EPSILON {
            return;
        }
        if self.weight < f64::EPSILON {
            self.sample = Some(sample);
            return;
        }
        if self.nbSamples == 0
            || rand::thread_rng().gen_range((0.)..=(self.weight)) < sample.weight
        {
            self.sample = Some(sample);
        }
    }
}
