use std::{env::args, process::exit, slice::Iter};

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() == 1 {}

    let mut expr: &[u8] = &[0];
    let mut cmd_ptr: usize = 0;
    let mut input: Iter<u8> = [0].iter();

    match args.len() {
        1 => {
            eprint!("Aucun argument donné. Utilisation: cerveaubaise <expr> [input]");
            exit(1);
        }
        2 => {
            expr = args[1].as_bytes();
        }
        3 => {
            expr = args[1].as_bytes();
            input = args[2].as_bytes().iter();
        }
        _ => (),
    }

    let data_array: &mut [u8] = &mut [0; 30000];
    let mut data_ptr: usize = 0;

    loop {
        if let Some(cmd) = expr.get(cmd_ptr) {
            match cmd {
                b'>' => data_ptr = (data_ptr + 1).clamp(0, 30000),
                b'<' => data_ptr = (data_ptr - 1).clamp(0, 30000),
                b'+' => data_array[data_ptr] += 1,
                b'-' => data_array[data_ptr] -= 1,
                b'.' => print!(
                    "{}",
                    char::from_u32(data_array[data_ptr] as u32).unwrap_or('?')
                ),
                b',' => data_array[data_ptr] = *input.next().unwrap_or(&0),
                b'[' => {
                    if data_array[data_ptr] == 0 {
                        find_matching_start_or_end(&mut cmd_ptr, expr, true);
                    }
                }
                b']' => {
                    if data_array[data_ptr] != 0 {
                        find_matching_start_or_end(&mut cmd_ptr, expr, false);
                    }
                }
                _ => (),
            }
        } else {
            break;
        }
        cmd_ptr += 1;
    }
}

fn find_matching_start_or_end(cmd_ptr: &mut usize, expr: &[u8], find_end: bool) {
    let change_ptr = |cmd_ptr: &mut usize, find_end: bool| {
        if find_end {
            *cmd_ptr += 1;
        } else {
            *cmd_ptr -= 1;
        }
    };
    let bracket = |opening_bracket: bool| -> u8 {
        if opening_bracket {
            return b'[';
        }
        b']'
    };

    while expr[*cmd_ptr] != bracket(!find_end) {
        change_ptr(cmd_ptr, find_end);
        if expr[*cmd_ptr] == bracket(find_end) {
            find_matching_start_or_end(cmd_ptr, expr, find_end);
        }
    }
    change_ptr(cmd_ptr, find_end);
}
