use linear_reg::ml;
use ml::LinearRegression;
use std::fs;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub fn read_file(filename_with_path: &str) -> std::io::Result<Vec<String>> {
    //get absolute path

    let abs_file_path = get_absolute_path(filename_with_path)?;
    let file = OpenOptions::new().read(true).open(abs_file_path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn get_absolute_path(path_str: &str) -> Result<String, std::io::Error> {
    let path = Path::new(path_str);
    let absolute_path = fs::canonicalize(path)?;
    Ok(absolute_path.to_string_lossy().to_string()) // Convert to string
}

fn main() {
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

    let mut file = OpenOptions::new()
        .append(true)
        .open(get_absolute_path("wt-bias.csv").unwrap())
        .expect("Failed to open the file");
    // Append data to the file
    // header
    let _ = file.write_all("Iteration,wt,bias,mse_loss\n".to_string().as_bytes());

    for i in 1..=20000 {
        let mut csv_record = vec![];

        let wt_slope_bias_slop= lr.train(wt, bias);


        csv_record.push(i.to_string());
        csv_record.push(wt.to_string());
        csv_record.push(bias.to_string());
        csv_record.push(lr.get_mean_suqare_error().to_string());

        csv_record.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");

        let str =csv_record.join(",");
        file.write_all(&str.as_bytes()).expect("Failed to append to the file");
        file.write_all("\n".as_bytes()).expect("Failed to append to the file");
        wt = wt - (0.01 * wt_slope_bias_slop.0);
        bias = bias - (0.01 * wt_slope_bias_slop.1);
    }
    println!("Calculation completed");

}
