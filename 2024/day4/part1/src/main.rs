use std::fs::read_to_string;

fn main() {
    let mut pointcounter = 0;

    let lines: Vec<String> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    let mut frfrvec: Vec<Vec<char>> = Vec::new();
    println!("ye the lines are {:?}", lines);

    // make y coord offset by 1
    let lenxold = lines[1].chars().count();
    let mut lenvec: Vec<char> = Vec::new();
    for i in 0..lenxold {
        lenvec.push('0');
    }
    frfrvec.push(lenvec);

    // main section where you push stuff
    for ministr in lines.clone() {
        let mut pushvec = Vec::new();
        pushvec.push('0'); // make x coord offset by 1
        for minichar in ministr.chars() {
            pushvec.push(minichar);
        }
        frfrvec.push(pushvec);
    }
    println!("{:?}", frfrvec);
    let lenx = lines[1].chars().count();
    let leny = frfrvec.clone().len();

    for y in 1..leny {
        let widthup = y >= 2;
        let widthdown = (leny - y) >= 2;

        for x in 1..lenx+1 {
            // check if enough space left and right

            let widthleft = x >= 2;

            let widthright = (lenx - x) >= 1;

            // search algorithm starts here
            // check if the char is X
            if frfrvec[y][x] == 'A' {
                println!("x is {x}");
                println!("y is {y}");
                println!("width up: {}", widthup);
                println!("width down: {}", widthdown);
                println!("width left: {}", widthleft);
                if widthup && widthleft && widthdown && widthright {
                    // check if start from top
                    if frfrvec[y - 1][x - 1] == 'M' // up left
                        && frfrvec[y - 1][x + 1] == 'M' // up right
                        && frfrvec[y + 1][x - 1] == 'S' // down left
                        && frfrvec[y + 1][x + 1] == 'S' // down right
                    {
                        pointcounter += 1;
                        println!("successful up");
                    }
                    // check if start from left
                    if frfrvec[y - 1][x - 1] == 'M' // up left
                        && frfrvec[y - 1][x + 1] == 'S' // up right
                        && frfrvec[y + 1][x - 1] == 'M' // down left
                        && frfrvec[y + 1][x + 1] == 'S' // down right
                    {
                        pointcounter += 1;
                        println!("successful left");
                    }
                    // check if start from bottom
                    if frfrvec[y - 1][x - 1] == 'S' // up left
                        && frfrvec[y - 1][x + 1] == 'S' // up right
                        && frfrvec[y + 1][x - 1] == 'M' // down left
                        && frfrvec[y + 1][x + 1] == 'M' // down right
                    {
                        pointcounter += 1;
                        println!("successful down");
                    }
                    // check if start from right
                    if frfrvec[y - 1][x - 1] == 'S' // up left
                        && frfrvec[y - 1][x + 1] == 'M' // up right
                        && frfrvec[y + 1][x - 1] == 'S' // down left
                        && frfrvec[y + 1][x + 1] == 'M' // down right
                    {
                        pointcounter += 1;
                        println!("successful right");
                    }
                }
            }
            println!("\n")
        }
    }

    println!("{pointcounter}");
}
