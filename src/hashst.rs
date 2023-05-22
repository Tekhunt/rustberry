use std::collections::HashSet;

pub fn create_set() {
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(3);
    set.insert(2);
    set.insert(5);
    set.insert(4);

    {
        let mut two = set.get(&2);
        println!("if two exist get it: {:?}", two);

    }

    
    set.remove(&2);

    println!("{:?}", set);
    for val in set.iter() {
        if val == &2 {
            println!("2 exists");
        } else {
            println!("Oops!");
            break;
        }
    }

    if set.contains(&3) {
        println!("3 exists")
    }

    // Create HashSet with default set of values using from() method
    let mut numbers = HashSet::from([2, 7, 8, 10]);
    println!("Default set: {:?}", numbers);
    set.len();
    set.is_empty();
    numbers.drain();
    println!("Default set: {:?}", numbers);

    let result: HashSet<_> =  set.union(&numbers).collect();
    println!("Union: {:?}", result);

    let intersect: HashSet<_> = set.intersection(&numbers).collect();
    println!("intersect: {:?}", intersect);

    let diff: HashSet<_> = set.difference(&numbers).collect();
    println!("difference: {:?}", diff);

    let sym_diff: HashSet<_> = set.symmetric_difference(&numbers).collect();
    println!("symetric_diff: {:?}", sym_diff);

    // collection.iter()
    // collection.into_iter()
    // collection.iter_mut()

    let numbers: Vec<i32> = vec![1, 2, 3];
    let adt: Vec<i32> = numbers.iter().map(|i| i%2).collect();
    println!("Even numbers: {:?}", adt);




}