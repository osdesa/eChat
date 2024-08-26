use sqlite::{Connection, State, Value};

pub fn init_database(){
    let connection = sqlite::open("database.sqlite").unwrap();

    match connection.execute("CREATE TABLE users (username TEXT, password TEXT);") {
        Ok(_) => println!("[INFO] Created user table"),
        Err(_) => println!("[INFO] User table exists"),
    };
}

pub fn filler(){
    let connection = sqlite::open("database.sqlite").unwrap();
    let query = "INSERT INTO users VALUES ('osdesa', '$argon2id$v=19$m=19456,t=2,p=1$cXdlcnR5cXdlcnR5cXdlcnR5cXdlcnR5cXdlcnR5$tNyWErKJ3q4NiSOj1ilSLhDaVTJOmlP2cUF8V6Wfth8');";

    connection.execute(query).unwrap();
}

pub fn check_user(username : String, password : String) -> bool{
    let connection = sqlite::open("database.sqlite").unwrap();

    let query = "SELECT * FROM users WHERE username = :username AND password = :password";
    
    let mut statement = connection.prepare(query).unwrap();

    statement.bind::<&[(_, Value)]>(&[
        (":username", username.into()),
        (":password", password.into()),
    ][..]).unwrap();

    !(statement.next().unwrap() == State::Done)
}