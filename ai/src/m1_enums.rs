
#[allow(unused)]
#[allow(dead_code)]
fn check_number_is_less_than_built_in(num:u8,target:u8) -> Result<u8,String> {

    if num <= target {
        Result::Ok(num)
    }else{
        Result::Err("Target number > Number".to_string())
    }
}
fn check_number_is_less_than_built_in_option(num:u8,target:u8) -> Option<u8> {

    if num <= target {
        Some(num)
    }else{
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enums(){
        dbg!("Hello, world!");
        dbg!(check_number_is_less_than_built_in_option(6, 5));
    }
}