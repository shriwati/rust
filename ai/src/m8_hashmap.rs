use std::collections::{HashMap,HashSet};



#[cfg(test)]

mod tests {
    use super::*;
    #[test]    
    fn hashmap_test(){
        dbg!("hello");
        let person1 = &"A";
        let person2 = &"B";
        let mut map = HashMap::new();
        map.insert(&person1,1);
        map.insert(&person2,2);
        
       for (k,v) in map.iter(){
           println!("{}:{}",&k,&v);
       }
        
        let score = map.get(&person1).unwrap();
        println!("score:{}",score);
    }

}
    
