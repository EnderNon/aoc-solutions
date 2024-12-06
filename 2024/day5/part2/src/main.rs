use std::fs;

fn main() {
    let mut lines: Vec<String> = Vec::new();
    // offset so the vec lines can actually start at 1 and not curse me
    lines.push("a".parse().unwrap());
    for i in fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from) {
        lines.push(i);
    };
    let lineslen = lines.len();


    // get the line which is empty to split into the First and Second bit
    let mut sep = 0;
    for i in 1..lines.len() {
        if lines[i].trim() == "" {
            println!("the line empty is {}",i);
            sep = i;
        }
    };

    let mut rul = Vec::new();
    for i in 1..sep {
        rul.push(lines[i].clone())
    };
    println!("rul {:?}",rul);

    let mut seq = Vec::new();
    for i in sep+1..lineslen {
        seq.push(lines[i].clone());
    }
    println!("seq {:?}",seq);


    let mut notsofinalvec: Vec<Vec<i32>> = Vec::new();
    'l1: for seq1 in seq {
        let seqvec: Vec<i32> = seq1.split(",").map(|a| a.parse().unwrap()).collect();
        let mut allow = true;
        'swaptest: for rul2 in rul.clone() {

            let rulez: Vec<String> = rul2.split("|").map(|a| a.to_string()).collect();
            let r1: i32 = rulez[0].clone().parse().unwrap();
            let r2: i32 = rulez[1].clone().parse().unwrap();

            if seqvec.contains(&r1) && seqvec.contains(&r2) {
                if let Some(pos1) = seqvec.iter().position(|a| a == &r1) {
                    if let Some(pos2) = seqvec.iter().position(|a| a == &r2) {
                        if pos1 > pos2 {
                            println!("allowed for {:?}, continued!!!!",seq1);
                            allow = false;
                            break 'swaptest;
                        }
                    }
                }
            }
        }
        if !allow {
            notsofinalvec.push(seqvec)
        }
    }
    for i in &notsofinalvec {
        println!("finalvec: {:?}",i)
    }

    let mut total = 0;

    println!("not so final vec {:?}",notsofinalvec);


    let truefinalvec = Vec::new();

    fn test_pass {

    }

    for i in notsofinalvec {

    }
}
