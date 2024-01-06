use crate::cursor_consts::{COLOUR_RESET, GREEN, RED};
use rand::{thread_rng, Rng};

pub struct Door {
    score: i8,
    safe: bool,
}

impl Door {
    fn new() -> Self {
        Self {
            score: thread_rng().gen_range(0..25),
            safe: false,
        }
    }

    pub fn set_score(&mut self, value: i8) {
        self.score += value
    }

    fn get_score(&self) -> i8 {
        self.score
    }

    fn open(&self) -> bool {
        if self.safe == true {
            return true;
        } else {
            return false;
        }
    }

    fn make_safe(&mut self) {
        self.safe = true;
    }
}

pub struct Doorgeon {
    pub room: Vec<Door>,
    pub next_size: usize,
}

impl Doorgeon {
    pub fn new() -> Self {
        Self {
            room: vec![],
            next_size: 1,
        }
    }

    pub fn available_doors(&self) -> usize {
        return self.room.len() - 1;
    }

    pub fn populate(&mut self) {
        self.reset_room();

        let safe_idx: usize = thread_rng().gen_range(0..self.next_size);

        for _ in 0..self.next_size {
            let mut d: Door = Door::new();
            d.set_score(thread_rng().gen_range(0..50));
            self.room.push(d);
        }
        self.room[safe_idx].make_safe();

        self.next_size += 1
    }

    fn reset_room(&mut self) {
        // Clears all doors from the room vec.
        self.room = vec![];
    }

    pub fn select_door(&self, choice: usize) -> (bool, i8) {
        if self.room[choice].open() {
            println!("{GREEN}You chose well.{COLOUR_RESET}");
            return (true, self.room[choice].get_score());
        } else {
            println!("{RED}FOOL! YOU HAVE DIED FOR YOU HUBRIS!{COLOUR_RESET}");
            return (false, 0);
        }
    }
}
