mod hello;
mod tups;


struct User {
    name: String,
    age: i32,
    height: f32,
    gads: Gadgets
}

impl User {
    fn is_tall(&self) -> bool {
        self.height >= 180.0
    }

    fn compare_height(&self, other: &User) -> bool {
        self.height > other.height
    }
}

struct Gadgets {
    tv: i32,
    phone: i32
}

fn main() {
    println!("Hello, world!");
    // hello::say_hello();

   let mut emma = User {
    name: String::from("Chizzy"),
    age: 30,
    height: 180.4,
    gads: Gadgets {
        tv: 2,
        phone: 2
    },
   };
   emma.age = 31;

   println!("{} {} is {}m tall!", emma.name, emma.age, emma.height);

   let User {name, age, height, gads} = emma;
   println!("{name}");

   let akuke = User {
    name: String::from("Akuke"),
    gads: Gadgets {
        tv: 2,
        phone: 2
    },
    ..emma
   
   };

   println!("{:?}", akuke.gads.tv);

   fn build_user(name: String, age: i32) -> User {
    User {
        name,
        age,
        height: 180.2,
        gads: Gadgets {
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

