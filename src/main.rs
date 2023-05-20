mod hello;
mod tups;
mod structsman;

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




}

