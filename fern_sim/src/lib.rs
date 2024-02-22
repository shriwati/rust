pub struct Fern{
    pub size:f64,
    pub growth_rate:f64
}

impl Fern{
    fn grow(&mut self){
        self.size *= self.growth_rate + 1.0;
    }
}

pub fn run_simulation(fern: &mut Fern,days : usize){
    for _ in 0..days{
        fern.grow();
    }
}

