fn main() {
    let mut list1: Vec<i32> =
        serde_json::from_reader(std::fs::File::open("l1.json").unwrap()).unwrap();
    let mut list2: Vec<i32> =
        serde_json::from_reader(std::fs::File::open("l2.json").unwrap()).unwrap();

    let mut simScoreTotal: i64 = 0;

    for i in &list1 {
        let mut totalsRight: i32 = 0;
        for j in &list2 {
            if i == j {
                totalsRight += 1;
            }
        }
        simScoreTotal += (totalsRight * i) as i64;
    }
    println!("{simScoreTotal}")
}
