fn main() {
    let rect : (i32,i32) = (200,500);

    struct Book{
        title: String,
        author: String,
        pages: u32,
        available: bool,
    }


    struct User{
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }



    let mut user1: User = User{
        active: true,
        username: String::from("someusername"),
        email: String::from("someusername@m.com"),
        sign_in_count: 1,
    };


    user1.email = String::from("anotheremail@m.com");
    println!("User email is {}",user1.email);


    fn build_user(email: String, username: String) -> User{
        User{
            active: true,
            email,
            username,
            sign_in_count: 1,
        }
        
    }


}
