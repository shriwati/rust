
#[allow(unused)]
#[allow(dead_code)]
#[allow(unused)]
trait Attacker{
    fn choose_style(&self) -> String;
 
}

#[derive(Debug)]
pub enum Character{
    Warrior,
    Archer,
    Wizard
}

impl Attacker for Character{
    fn choose_style(&self) -> String{
        match self{
            Character::Warrior => "wing chun".to_string(),
            Character::Archer => "kung fu".to_string(),
            Character::Wizard => "thai chi".to_string()
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traits() {
        let w = Character::Archer;
        let style = w.choose_style();
        dbg!(style);
       
    }

}