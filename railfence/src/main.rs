mod rail_fence;


fn main() {

    let msg=String::from("Shriisagoodboy!");
    let encrypted_string:String=rail_fence::rail_fence::encrypt(&msg,3);
    println!("{}",&encrypted_string);

    println!("{}","");
    println!("{:?}",rail_fence::rail_fence::decrypt(&encrypted_string,3));


}



