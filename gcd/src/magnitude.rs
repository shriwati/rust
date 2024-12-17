//calculate magnitude of a vector

    pub fn magnitude(vector: &Vec<f64>) -> f64 {
        let mut total: f64 = 0.0;
        for f in vector {
            total += f.powf(2.0);
        }
        total.sqrt()
    }

    pub fn normalize(vec: &mut Vec<f64>) -> f64 {
        let mut total: f64 = 0.0;
        let mag = magnitude(&vec);
        for v in vec {
            *v /= mag;
        }
        total
    }

