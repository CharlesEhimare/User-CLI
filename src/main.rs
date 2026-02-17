use std::io;

#[derive(Debug)]
enum Role {
    Admin,
    User,
    Guest,
}

impl Role {
    fn from_input(input: &str) -> Option<Role> {
        match input.trim().to_lowercase().as_str() {
            "admin" => Some(Role::Admin),
            "user" => Some(Role::User),
            "guest" => Some(Role::Guest),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct User {
    name: String,
    id: u32,
    login_count: u32,
    role: Role,
    active: bool,
}

impl User {
    fn login(&mut self) {
        self.login_count += 1;
    }

    fn display_info(&self) {
        println!(
            "User Info: Name: {}, ID: {}, Role: {:?}, Login Count: {}, Active: {}",
            self.name, self.id, self.role, self.login_count, self.active
        );
    }

    fn access_admin(&self) -> bool {
        match self.role {
            Role::Admin => self.active,
            _ => false,
        }
    }
}

fn main() {
    println!("Enter role (admin/user/guest):");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let role = match Role::from_input(&input) {
        Some(r) => r,
        None => {
            println!("Invalid role entered.");
            return;
        }
    };

    let mut user1 = User {
        name: String::from("Charles"),
        id: 1,
        role,
        login_count: 0,
        active: true,
    };

    user1.display_info();
    user1.login();
    user1.display_info();

    println!("User has admin access: {}", user1.access_admin());
}
