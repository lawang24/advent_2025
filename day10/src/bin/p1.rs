use std::{fs, cmp};

fn lights_encoder(light: &str) -> u64 {
    // process out []
    let light = &light[1..light.len()-1];
    let mut val = 0;
    for (i, c) in light.chars().enumerate(){
        if c == '#'{
            val |= 1 << i;
        }
    }
    val
}

fn buttons_encoder(buttons: &str) -> u64 {
    let buttons = &buttons[1..buttons.len()-1];
    let mut val = 0;
    for c in buttons.chars(){
        if c.is_ascii_digit() {
            val |= 1 << c.to_digit(10).unwrap();
        }
    }
    val
}

fn calculate_button_presses(buttons: &Vec<u64>, light: u64, idx: usize , curr: u64, presses: u64) -> u64 {

    if curr == light {
        return presses;
    }

    let mut answer = u64::MAX;

    for i in idx .. buttons.len(){
        // press
        answer = cmp::min(answer, calculate_button_presses(buttons, light, i+1, curr ^ buttons[i] , presses+1));
        // not press
        answer = cmp::min(answer, calculate_button_presses(buttons, light, i+1, curr , presses));
    }

    answer

}

fn main(){
    
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut total_presses = 0_u64;

    for line in contents.lines(){
        
        let mut items = line.split_whitespace();
        let light = items.next().unwrap();
        let _ = items.next_back().unwrap();
        let buttons: Vec<&str> = items.collect();    
        let button_encoded: Vec<u64> = buttons.iter().map(|x| buttons_encoder(x)).collect();    
        let light_encoded = lights_encoder(light);
        let presses = calculate_button_presses(&button_encoded, light_encoded, 0, 0, 0);
        total_presses += presses;
    }

    println!("{total_presses}");

}