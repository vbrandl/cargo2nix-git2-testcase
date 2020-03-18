use git2::Repository;

fn main() {
    match Repository::clone("https://github.com/vbrandl/hoc", "test") {
        Ok(_) => println!("ok"),
        Err(e) => {
            println!("Code: {:?}", e.code());
            println!("Class: {:?}", e.class());
            println!("Message: {}", e.message());
        }
    }
}
