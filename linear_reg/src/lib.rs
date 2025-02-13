

pub mod ml {
    #[derive(Debug)]
    pub struct LinearRegression {
        feature: Vec<f32>,
        label: Vec<f32>,
        prediction: Vec<f32>,
        // learning_rate: f32,
        weight: f32,
        bias: f32,
        mean_suqare_error: f32,
    }

    /*
        1.Start with Wt / Bias = 0 , 0
        2.Calculate MSE Loss
        3.Calculate slope and tanget loss at each wt and bias
        4.Move small amount of weights and bias
            4.1 new weight = old weight - ( small amount  * weight slope)
            4.2 new bias = old bias - ( small amount * bias slope)


    */

    impl LinearRegression {
        pub fn new(
            feature: Vec<f32>,
            label: Vec<f32>,
            // learning_rate: f32
        ) -> LinearRegression {
            LinearRegression {
                feature,
                label,
                prediction: vec![],
                // learning_rate,
                weight: 0.0,
                bias: 0.0,
                mean_suqare_error: 0.0,
            }
        }

        pub fn get_label(&self) -> &Vec<f32> {
            &self.label
        }
        pub fn get_weight(&self) -> f32 {
            self.weight
        }
        pub fn get_bias(&self) -> f32 {
            self.bias
        }
        pub fn get_mean_suqare_error(&self) -> f32 {
            self.mean_suqare_error
        }
        pub fn train(&mut self, wt: f32, bias: f32) -> (f32, f32) {
            self.weight = wt;
            self.bias = bias;
            //clear prediction
            self.prediction.clear();
            // calculate prediction
            self.predict();
            // calculate loss
            self.calculate_mean_square_error();
            //return wt and bias
            self.calculate_new_wt_bias()
        }
        fn predict(&mut self) {
            //y = w1x1 + b1
            // y = wt * feature[i] + bias
            for f in &self.feature {
                self.prediction.push((f * self.weight) + self.bias);
            }
        }
        // fn get_mse_loss(&self) -> Vec<f32> {
        //     // (prediction - actual) *( 2 * feature)
        //     // it is assumed that feature index and prediction index is aligned
        //     let mut loss: Vec<f32> = vec![];
        //
        //     let len = self.prediction.len();
        //     let mut i = 0;
        //     while i <= len - 1 {
        //         loss.push((self.prediction[i] - self.label[i]) * (2.0 * self.feature[i]));
        //         i += 1;
        //     }
        //     loss
        // }
        // ( power of 2 ( actual[i] - feature[i)+...power of 2(actual[n]-feature[n]) ) /n
        fn calculate_mean_square_error(&mut self) {
            let len = self.feature.len();
            let mut i = 0;
            //get prediction
            // let prediction:Vec<f32>=self.calculate_prediction(wt, bias);
            let mut sme: Vec<f32> = vec![0.0; len];
            while i <= len - 1 {
                sme.push((self.label[i] - self.prediction[i]).powf(2f32));
                i += 1;
            }
            self.mean_suqare_error = sme.iter().sum::<f32>() / (len as f32)
        }
        fn calculate_new_wt_bias(&self) -> (f32, f32) {
            let len = self.feature.len();
            let mut i = 0;
            let mut wt_list: Vec<f32> = vec![];
            let mut bias_list: Vec<f32> = vec![];

            while i <= len - 1 {
                wt_list.push((&self.prediction[i] - self.label[i]) * (2.0 * self.feature[i]));
                bias_list.push((&self.prediction[i] - self.label[i]) * (2.0));
                i += 1;
            }
            (
                wt_list.iter().sum::<f32>() / len as f32,
                bias_list.iter().sum::<f32>() / len as f32,
            )
        }
    }
}

#[test]
fn it_works() {
    /*
        1.Start with Wt / Bias = 0 , 0
        2.Calculate MSE Loss
        3.Calculate slope and tanget loss at each wt and bias
        4.Move small amount of weights and bias
            4.1 new weight = old weight - ( small amount  * weight slope)
            4.2 new bias = old bias - ( small amount * bias slope)


    */

    let feature = vec![3.5, 3.69, 3.44, 3.43, 4.34, 4.42, 2.37];
    let label = vec![18.0, 15.0, 18.0, 16.0, 15.0, 14.0, 24.0];

    // start with wt = 0 and loss = 0
    let mut wt = 0.0;
    let mut bias = 0.0;
    let mut lr = LinearRegression::new(feature.clone(), label.clone());

    for _i in 1..=10 {
        let wt_slope_bias_slop = lr.train(wt, bias);
        println!(
            "Iteration:{}, Wt:{}, Bias:{}, MSE:{}",
            _i,
            wt,
            bias,
            lr.get_mean_suqare_error()
        );
        wt = wt - (0.01 * wt_slope_bias_slop.0);
        bias = bias - (0.01 * wt_slope_bias_slop.1);
    }
    println!("Calculation completed");
}
