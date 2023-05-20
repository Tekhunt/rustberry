use std::collections::HashMap;

pub fn hash_lib() {
    let mut info: HashMap<i32, String> = HashMap::new();
    info.insert(0, String::from("Chizzy"));
    info.insert(1, String::from("Tekhunt"));
    let tek = info.get(&1);

    println!("tek is: {:?}", tek);

    // &info.remove(&0);

    for (i, j) in &info{
        println!("Key is: {}, Value is: {}", i, j);
    }

    println!("{:?}", info);

    // hashmap methods
    println!("Length of info is: {}", info.len());
    println!("Is there value for position 0? {}", info.contains_key(&0));
    println!("Iterator: {:?}", info.iter());
    println!("Get values: {:?}", info.values());
    println!("Get keys: {:?}", info.keys());

    let mut new_info = info.clone();
    new_info.insert(2, String::from("Hunter"));
    println!("New info is: {:?}", new_info);


}