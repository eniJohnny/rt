use rand::Rng;

use crate::model::{materials::Color, maths::hit::Hit};

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
        if self.weight + rhs.weight == 0. {
            return;
        }
        let rand: f64 = rand::thread_rng().gen_range((0.)..(self.weight + rhs.weight));
        if rand > self.weight {
            self.sample = rhs.sample;
        }
        self.weight += rhs.weight;
        self.nbSamples += rhs.nbSamples;
    }

    pub fn add(&mut self, sample: Sample<'a>) {
        self.weight += sample.weight;
        if self.weight + sample.weight == 0. {
            return;
        }
        if self.weight == 0.
            || rand::thread_rng().gen_range((0.)..(self.weight + sample.weight)) > self.weight
        {
            self.sample = Some(sample);
        }
        self.nbSamples += 1;
    }
}
