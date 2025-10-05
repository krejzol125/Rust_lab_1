use std::io;
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let result = loop  {
        println!("Podaj liczbę: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect(":(");

        let mut x:u64 = match input.trim().parse() {
            Err(_) => break true,
            Ok(num) => {
                if num == 0 {
                    break false;
                }
                num
            },
        };
        x += rand::thread_rng().gen_range(1..=5);
        println!("{}", x);
        
        let tab = tabulacja(x);
        let cort = check_coltez(tab);
        
        save_to_file(tab, cort);
    };
    if result {
        println!("Error");
    } else {
        println!("Użytkownik zakończył program")
    }
    let tp = fun (20);
    println!("{} : {}", tp.0, tp.1)
}

fn tabulacja(x:u64) -> [u64; 10] {
    let mut tab: [u64; 10] = [0; 10];
    let mut curr = 1;
    for item in &mut tab {
        curr *= x;
        *item = curr;
    }
    tab    
}

fn check_coltez(tab: [u64; 10]) -> [bool; 10] {
    let mut result: [bool; 10] = [false; 10];
    'tab: for i in 0..=9
    {
        let mut curr = tab[i];
        for _ in 1..=100 {
            if curr.is_multiple_of(2) {
                curr /= 2;
            } else {
                curr = 3 * curr + 1;
            }
            if curr == 1 {
                result[i] = true;
                continue 'tab;
            }
        }
        result[i] = false;
    }
    result
}

fn save_to_file(tab: [u64; 10], cort: [bool; 10]) {
    let mut file = File::create("xyz.txt").expect("Unable to create file");
    for i in 0..=9 {
        let line = format!("{} {}\n", tab[i], cort[i]);
        file.write_all(line.as_bytes()).expect("Unable to write data");
    }
}

fn fun(x: u64) -> (u64, bool) {
    let mut i = 1;
    let mut count = 0;
    let mut res = false;
    'label: loop{
        i += 1;
        let mut j = i;
        while j > 0
        {
            print!("#");
            j -= 1;
            if 1 == rand::thread_rng().gen_range(1..=100)
            {
                println!(" --- AWARIA --- ");
                break 'label;
            }
            else {
                count += 1;
            }
        }
        if i == x {
            res = true;
            break;

        }
        println!();
    }
    (count, res)
}