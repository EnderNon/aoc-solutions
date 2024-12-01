fn main() {


    let mut list1: Vec<i32> = serde_json::from_reader(std::fs::File::open("l1.json").unwrap()).unwrap();
    let mut list2: Vec<i32> = serde_json::from_reader(std::fs::File::open("l2.json").unwrap()).unwrap();

    list1.sort();
    list2.sort();

    if list1.len() != list2.len() {
        panic!("your lists are broken. The length is not equal.")
    }

    let mut totaldiff: i64 = 0;
    for i in 0..list1.len() {
        let int1 = list1[i];
        let int2 = list2[i];
        if int1 > int2 {
            totaldiff += (int1 - int2) as i64;
        }
        else if int1 < int2 {
            totaldiff += (int2 - int1) as i64;
        }
    }

    println!("{totaldiff}")
}
