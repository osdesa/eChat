pub struct UserState{
    pub username : String,
    pub auth : bool,
}

impl UserState{
    pub fn new() -> Self {
        UserState {
           auth : false,
           username : String::from(""),
        }
    }
}