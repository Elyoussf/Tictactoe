use std::io;
use std::collections::HashMap;

fn main() {
    let mut table: [[char; 3]; 3] = [[' '; 3]; 3]; // table 3 * 3

    println!("Note: The numerotation used is as presented in old phones 1 --> 9");
    println!("Who starts, X or O? Type one:");

    let mut XorO = String::new();
    io::stdin().read_line(&mut XorO).unwrap();
    let mut XorO = XorO.trim().to_uppercase();

    while XorO != "X" && XorO != "O" {
        println!("The only characters accepted are X and O");
        io::stdin().read_line(&mut XorO).unwrap();
    }

    let mut XO_OX = HashMap::new();
    XO_OX.insert("X".to_string(), "O".to_string());
    XO_OX.insert("O".to_string(), "X".to_string());

    let mut getTurn = HashMap::new();
    getTurn.insert(0, XorO.clone());
    getTurn.insert(1, XO_OX.get(&XorO).unwrap().clone());

    let mut turn = 0;

    loop {
        render_table(&table); // Render the table before each move
        
        if let Some(winner) = checkForAwinner(&table) {
            println!("Hey Folks! We have a winner! Let's celebrate!");
            println!("The winner is {}", winner);
            break;
        }

        if !notFinishedYet(&table) {
            println!("The Game has been finished with no winner");
            break;
        }

        println!("Enter the cell number where you want to put {}:", getTurn.get(&turn).unwrap());
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let number: i32 = match input.trim().parse() {
            Ok(num) if num > 0 && num < 10 => num,
            _ => {
                println!("Invalid number, please choose between 1-9");
                continue;
            }
        };

        let (x, y) = translate_number_to_position(number);
        if table[x][y] == ' ' {
            table[x][y] = getTurn.get(&turn).unwrap().chars().next().unwrap();
            turn = (turn + 1) % 2;
        } else {
            println!("The chosen cell is already filled!");
        }
    }
}

fn render_table(table: &[[char; 3]; 3]) {
    println!("Current Board:");
    for row in table.iter() {
        println!("+---+---+---+");
        for &cell in row.iter() {
            print!("| {} ", cell);
        }
        println!("|");
    }
    println!("+---+---+---+");
}

fn translate_number_to_position(number: i32) -> (usize, usize) {
    let x = (number - 1) / 3;
    let y = (number - 1) % 3;
    (x as usize, y as usize)
}

fn notFinishedYet(table: &[[char; 3]; 3]) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            if table[i][j] == ' ' {
                return true;
            }
        }
    }
    false
}

fn checkForAwinner(table: &[[char; 3]; 3]) -> Option<char> {
    // Check vertically
    for i in 0..3 {
        if table[0][i] == table[1][i] && table[1][i] == table[2][i] && table[0][i] != ' ' {
            return Some(table[0][i]);
        }
    }
    // Check diagonally
    if (table[0][0] == table[1][1] && table[1][1] == table[2][2] && table[0][0] != ' ')
        || (table[0][2] == table[1][1] && table[1][1] == table[2][0] && table[0][2] != ' ')
    {
        return Some(table[1][1]);
    }
    // Check horizontally
    for i in 0..3 {
        if table[i][0] == table[i][1] && table[i][1] == table[i][2] && table[i][0] != ' ' {
            return Some(table[i][0]);
        }
    }
    None
}
