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
        let widthup = y >= 4;
        let widthdown = (leny - y) >= 4;

        for x in 1..lenx+1 {
            // check if enough space left and right

            let widthleft = x >= 4;

            let widthright = (lenx - x) >= 3;

            // search algorithm starts here
            // check if the char is X
            if frfrvec[y][x] == 'X' {
                println!("x is {x}");
                println!("y is {y}");
                println!("width up: {}", widthup);
                println!("width down: {}", widthdown);
                println!("width left: {}", widthleft);
                // check if going upwards
                if widthup {
                    if frfrvec[y - 1][x] == 'M'
                        && frfrvec[y - 2][x] == 'A'
                        && frfrvec[y - 3][x] == 'S'
                    {
                        pointcounter += 1;
                        println!("successful up");
                    }
                }
                // check if going diag up left
                if widthup && widthleft {
                    if frfrvec[y - 1][x - 1] == 'M'
                        && frfrvec[y - 2][x - 2] == 'A'
                        && frfrvec[y - 3][x - 3] == 'S'
                    {
                        pointcounter += 1;
                        println!("successful up left");
                    }
                }
                // check if going left
                if widthleft {
                    if frfrvec[y][x - 1] == 'M'
                        && frfrvec[y][x - 2] == 'A'
                        && frfrvec[y][x - 3] == 'S'
                    {
                        pointcounter += 1;
                        println!("successful left");
                    }
                }
                // check if going diag down left
                if widthdown && widthleft {
                    if frfrvec[y + 1][x - 1] == 'M'
                        && frfrvec[y + 2][x - 2] == 'A'
                        && frfrvec[y + 3][x - 3] == 'S'
                    {
                        pointcounter += 1;
                        println!("successful down left");
                    }
                }
                // check if going down
                if widthdown {
                    if frfrvec[y + 1][x] == 'M'
                        && frfrvec[y + 2][x] == 'A'
                        && frfrvec[y + 3][x] == 'S'
                    {
                        pointcounter += 1;
                        println!("successful down");
                    }
                }
                // check if going diag down right
                if widthdown && widthright {
                    if frfrvec[y + 1][x + 1] == 'M'
                        && frfrvec[y + 2][x + 2] == 'A'
                        && frfrvec[y + 3][x + 3] == 'S'
                    {
                        pointcounter += 1;
                        println!("successful down right");
                    }
                }
                // check if going right
                if widthright {
                    if frfrvec[y][x + 1] == 'M'
                        && frfrvec[y][x + 2] == 'A'
                        && frfrvec[y][x + 3] == 'S'
                    {
                        pointcounter += 1;
                        println!("successful right");
                    }
                }
                // check if going diag right up
                if widthright && widthup {
                    if frfrvec[y - 1][x + 1] == 'M'
                        && frfrvec[y - 2][x + 2] == 'A'
                        && frfrvec[y - 3][x + 3] == 'S'
                    {
                        pointcounter += 1;
                        println!("successful up right");
                    }
                }
            }
            println!("\n")
        }
    }

    println!("{pointcounter}");
}
