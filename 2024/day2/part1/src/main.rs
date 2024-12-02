use std::fs::read_to_string;
use std::thread::current;

fn main() {
    //  read file input
    let lines: Vec<String> = read_to_string("input.txt").unwrap().lines().map(String::from).collect();
    // current safe total count
    let mut safecount = 0;
    // for each line
    for i in lines {
        println!("{i}");
        // for each line, split into a Vec<String> of the values
        let mut frfrstr: Vec<String> = i.split_whitespace().map(|s| s.parse().unwrap()).collect();
        // convert that Vec<String> into Vec<Int>
        let mut frfr: Vec<i64> = Vec::new();
        for j1 in frfrstr {
            let j2: i64 = j1.parse::<i64>().unwrap();
            frfr.push(j2);
        }

        // for each line, check if safe. Start false. If it ends up being fine then it'll be set to true.
        let mut safemini = false;

        // sorted regular and reverse to check condition 1
        let mut sort1 = frfr.clone();
        sort1.sort();
        let mut sort2 = sort1.clone();
        sort2.reverse();
        if frfr == sort1 || frfr == sort2 {

            let mut mode = 0;
            let tempval0 = frfr[*&0];
            let tempval1 = frfr[*&1];
            if tempval0 < tempval1 {
                println!("increasing mode");
                mode = 1;
            }
            else if tempval0 > tempval1 {
                println!("decreasing mode");
                mode = -1;
            }
            else if tempval0 == tempval1 {
                println!("its equal and so its discarded");
                continue;
            }

            // condition 2 but only if cond 1 passes
            // starts true, becomes invalid
            let mut currentcheckstate = true;
            'fr1: for i2 in 1..frfr.len() {
                let realdiff = frfr[*&i2-1] - (frfr[*&i2]);
                if mode == 1 {
                    if realdiff > -1 || realdiff < -3 {
                        println!("realdiff {realdiff} plus");
                        currentcheckstate = false;
                        println!("abs diff averted");
                    }
                }
                else if mode == -1 {
                    if realdiff > 3 || realdiff < 1 {
                        println!("realdiff {realdiff} minus");
                        currentcheckstate = false;
                        println!("abs diff averted");
                    }
                }
            }
            if currentcheckstate {
                safemini = true;
                println!("allowed")
            }


        }

        if safemini {
            safecount += 1
        }


    }
    println!("{safecount}")
}
