pub fn reverse_string(input: &String) -> String {
    input.chars().rev().collect()
}

pub fn inspect_string(input: &String, digits: bool) -> (i32, String) {
    if !digits {
        return (input.len() as i32, String::from("char"));
    }

    (inspect_number(input), String::from("digit"))
}

fn inspect_number(input: &String) -> i32 {
    let mut count = 0;

    for c in input.chars() {
        if c.is_digit(10) {
            count += 1;
        }
    }

    count
}