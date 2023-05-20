pub fn runner() {
 let coordinates: (i32, i32, i32) = (1,2,3);
 println!("{:?}", coordinates);
 println!("x = {:?}", coordinates.0);
 println!("y = {:?}", coordinates.1);
 println!("z = {:?}", coordinates.2);

}

pub fn add_tups((x,y): (i64, i64)) -> i64 {
    x+y
 }