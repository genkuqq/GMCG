use std::collections::VecDeque;

pub struct Player{
    pub id: u32,
}

pub struct Matchmaking{
    queue: VecDeque<Player>
}

impl Matchmaking{
    pub fn new() -> Self{
        Matchmaking{
            queue: VecDeque::new()
        }
    }
    pub fn add_player(&mut self, player:Player){
        if self.queue.iter().any(|p| p.id == player.id){
            return;
        }
        self.queue.push_back(player);
        if self.queue.len() >= 2{
            self.try_match();
        }
    }
    pub fn remove_player(&mut self, player:Player){
        if let Some(index) = self.queue.iter().position(|p| p.id == player.id){
            self.queue.remove(index).unwrap();
        }
    }
    fn try_match(&mut self){
        while self.queue.len() >= 2{
            let player1 = self.queue.pop_front().unwrap();
            let player2 = self.queue.pop_front().unwrap();
            print!("t");

        }
    }
}
