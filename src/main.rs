mod hello;
mod tups;
mod structsman;
mod hashmp;
mod hashst;
mod unexpect;
extern crate rand;

fn main() {
    println!("Hello, world!");
    // hello::say_hello();

   let mut emma = structsman::User {
    name: String::from("Chizzy"),
    age: 30,
    height: 180.4,
    gads: structsman::Gadgets {
        tv: 2,
        phone: 2
    },
   };
   emma.age = 31;

   println!("{} {} is {}m tall!", emma.name, emma.age, emma.height);

   let structsman::User {name, age, height, gads} = emma;
   println!("{name}");

   let akuke = structsman::User {
    name: String::from("Akuke"),
    gads: structsman::Gadgets {
        tv: 2,
        phone: 2
    },
    ..emma
   
   };

   println!("{:?}", akuke.gads.tv);

   fn build_user(name: String, age: i32) -> structsman::User  {
    structsman::User  {
        name,
        age,
        height: 180.2,
        gads: structsman::Gadgets {
            tv: 1,
            phone: 1,
        },
    }
   }

   let serioto = build_user(String::from("Serioto"), 28);
   println!("{}", serioto.gads.phone);
   println!("Is serioto tall? {}: ", serioto.is_tall());
if serioto.compare_height(&akuke) == false {
    println!("You are shorter!")
} else {
    println!("Taller you are!")
}

println!("{}", akuke.age);

tups::runner();
let res = tups::add_tups((2,4));

println!("{}", res);

hashmp::hash_lib();

hashst::create_set();

fn get_user(username: &str) -> Option<&str> {
    if username.is_empty() {
        return None;
    }

    return Some(username);
}

// let user_option = get_user("Hari");
// let result = match user_option {
//     Some(user) => user,
//     None => "not found!",
// };

// let result = get_user("Hari").unwrap();
let result = get_user("Chizzy").expect("User not found");

// print the result
println!("user = {:?}", result);

let str = String::from("Hello, World!");
    
// Call function with reference String value
let len = unexpect::calculate_length(&str);

println!("The length of '{}' is {}.", str, len);

config::print();
config::select();

mod config {
    // items in modules by default have private visibility
    pub fn select() {
        println!("called config::select");
    }

    // use the `pub` keyword to override private visibility
    pub fn print() {
        println!("called config::print");
    }
}

// call public function create from sprite module which is inside player module 
// player::sprite::create();
use player::sprite::create;
create();

// nested module
pub mod player {
    pub mod sprite {
        pub fn create() {
            println!("called player::sprite::create");
        }
    }
}

let mut rng = rand::thread_rng();
use rand::Rng;

// simulate rolling a die
println!("roll = {}", rng.gen_range(1..=6));

// Command	Description
// cargo new	Create a new Rust project with basic directory structure
// cargo build	Build (compile) the current project and generate a binary executable
// cargo run	Build and run your current project (cargo build + run)
// cargo check	Build the current project without generating a binary executable
// cargo add	Add a new dependency and include it in Cargo.toml file
// cargo update	Update all dependencies of current project to latest version


}

