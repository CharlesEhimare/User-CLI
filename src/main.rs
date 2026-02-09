#[derive(Debug)]
struct User {
    name: String,
    id: u32,
    login_count: u32,
    role:Role,
    active: bool,
}

  
  impl User {
    fn login(&mut self) {
        self.login_count += 1;
    }

    fn display_info(&self) {
       println!("User Info: Name: {}, ID: {}, Role: {}, Login Count: {}, Active: {}", self.name, self.id, self.role, self.login_count, self.active);
    }
}
impl User{
        fn access_admin(&self) -> bool {
            self.role == "Admin" && self.active
        }  
    }
fn main(){
    let mut user1 = User{
        name: String::from("Charles"),
        id: 1, 
        role:String::from("Admin"),
        login_count: 5,  
        active: true,
    };
    user1.display_info();
    user1.login();
    user1.display_info();
    println!("User has admin access: {}", user1.access_admin());
    println!("Role: {:?}", user1.role);
    }


enum Role {
    Admin,
    User,
    Guest,
}
let mut user1 = User {
    name: String::from("Charles"),
    id: 1,
    role: Role::Admin,
    login_count: 5,
    active: true,
};
 impl User{
    fn access_admin(&self) -> bool{
        match self.role{
            Role::Admin => self.active,
            _ => false,
        }
    }
 }
 #[derive(Debug)]
 enum Role {
    Admin,
    User,
    Guest,
 }


    




