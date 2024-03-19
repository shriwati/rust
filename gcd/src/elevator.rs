// Elevator
// Ddta
    // current_floor : u32
    // requested_floor : u32
    // direction :Direction
    // stopped : bool
    // moving : moving
//Actions
    // go to floor
    // current state - stopped / moving /

const MAX_FLOOR_NO: u32 =50;
const MIN_FLOOR_NO:u32=1;
#[derive(Debug)]
pub enum Direction{
    Up,
    Down,
    STOPPED
}
pub struct Elevator{
    current_floor:u32,
    requested_floor:u32,
    direction:Direction,
    stopped:bool,
    moving:bool

}

impl Elevator {
    pub fn start()->Elevator{
        Elevator{current_floor:0,requested_floor:0,direction:Direction::STOPPED,stopped:true,moving:false }
    }
    pub fn move_elevator(&mut self, direction:Direction,to_floor:u32){
        match direction {
            Direction::Up=>{
                    if self.current_floor < to_floor && to_floor <= MAX_FLOOR_NO as u32 {
                        self.requested_floor=to_floor;
                        self.direction=direction;
                        self.current_floor =to_floor;
                        self.stopped=true;

                    }

            },
            Direction::Down =>{
                if self.current_floor > to_floor && to_floor >= MIN_FLOOR_NO{
                    self.requested_floor=to_floor;
                    self.direction=direction;
                    self.current_floor = to_floor;
                    self.stopped=true;
                }
            }
            Direction::STOPPED=>{
                self.moving = false;
            }
        }
        self.elevator_is_on();
    }
     pub fn elevator_is_on(&self){
        println!("Elevator is on floor #{} ",self.current_floor);
    }

}