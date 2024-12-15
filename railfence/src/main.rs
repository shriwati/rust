mod rail_fence;


fn main() {


    let msg=String::from("This is a long string to be encrypted");
    let encrypted_string:String=rail_fence::rail_fence::encrypt(&msg,4);
    println!("{}",&encrypted_string);

    println!("{}","");
    println!("{}",rail_fence::rail_fence::decrypt(&encrypted_string,4));


}



