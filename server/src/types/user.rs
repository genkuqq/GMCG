use enet::PeerID;


#[derive(Debug)]
pub struct User{
    pub id: PeerID,
    pub state: UserState,
}
#[derive(Debug)]
pub enum UserState{
    Idle,
    Searching,
    InGame(u32)
}

impl User{
    pub fn new(id: PeerID) -> Self{
        Self{
            id,
            state: UserState::Idle,
        }
    }
}
