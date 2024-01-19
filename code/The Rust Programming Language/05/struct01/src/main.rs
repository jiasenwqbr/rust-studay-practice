fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("some@example.com"),
        sign_in_count: 1,
    };
    user1.active = false;
    user1.username = String::from("Jason");
    user1.email = String::from("jason@example.com");
    user1.sign_in_count = 20;

    let user2 = build_user(String::from("jason@example.com"), String::from("jason"));
    let user2 = build_user_short(String::from("jason@example.com"), String::from("jason"));
    let user3 = User { ..user2 };
    let user4 = User {
        active: false,
        ..user3
    };

    let c1 = Color(12, 34, 34);
    let mut c2 = c1;
    c2.0 = 34;
    c2.1 = 35;
    //let c3 = c1;

    let p1 = Person(String::from("Jason"), String::from("male"), 20);
    let subject = AlwaysEqual;
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn build_user_short(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 10,
    }
}

struct Color(i32, i32, i32);
struct Person(String, String, i8);

struct AlwaysEqual;
// struct User2 {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }
