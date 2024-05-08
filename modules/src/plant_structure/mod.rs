mod leaves;
mod stem;
mod roots;

pub mod plant_structure{
    use crate::plant_structure::leaves;
    pub fn display(){
        // call forwarded
        leaves::leaves::display();
    }

}