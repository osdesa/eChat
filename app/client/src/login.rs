use argon2::{password_hash::{Salt, SaltString}, Argon2, PasswordHasher, PasswordVerifier};
use rand::rngs::OsRng;

struct hashed {
    password : String,
    salt : SaltString,
}

pub fn login(username : String, password : String) -> bool {
    println!("username: {} password: {}", username, password);
    let hash_info = hash_password(password);
    println!("username: {} encrypted: {}", username, hash_info.password);


    false
}

fn send_info(username : String, password : String){

}

fn encrypt_info(info : String) -> String{
    
    String::from("")
}

fn hash_password(password : String) -> hashed{
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), &salt).unwrap();
    
    Argon2::default().verify_password(password.as_bytes(), &hash).expect("invalid password");

    return hashed{ password : hash.to_string(), salt : salt}
}