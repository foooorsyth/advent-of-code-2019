use std::fs;
use std::collections::HashMap;
use std::io;

macro_rules! scanline {
    ($x:expr) => {
       io::stdin().read_line(&mut $x).unwrap();
    };
}

pub fn execute(input_file: &'static str,
            output: (/* true = value at pos, false = last_output */ bool, /* pos */ usize)) -> std::io::Result<i32> {
    return execute_with_overwrite(input_file, output, &None);
}

pub fn execute_with_overwrite(input_file: &'static str, output: (bool, usize), overwrite: &Option<&HashMap<usize, i32>>) -> std::io::Result<i32> {
    let input_string: String = fs::read_to_string(input_file)?;
    let mut input_vec: Vec<i32> = input_string.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    if overwrite.is_some() {
        let ow = overwrite.unwrap();
        for kvp in ow {
            input_vec[*kvp.0] = *kvp.1;
        }
    }
    let mut instr_ptr: usize = 0;
    let mut last_output = None;
    let input_len = input_vec.len();
    while instr_ptr < input_len {
        let should_continue = instruction(&mut input_vec, &mut instr_ptr, &mut last_output)?;
        if !should_continue {
            if output.0 {
                return Ok(input_vec[output.1])
            } else {
                return Ok(last_output.unwrap())
            }
        }
    }
    panic!("wtf")
}

fn instruction(data: &mut Vec<i32>, instr_ptr: &mut usize, last_output: &mut Option<i32>) -> std::io::Result<bool> {
    let opcode = read_opcode(&data[*instr_ptr]);
    match opcode {
        1 => { 
            // Add
            two_param_op_assign(data, *instr_ptr, |a: &i32, b: &i32| -> i32 { *a + *b });
            *instr_ptr += 4;
            Ok(true)
        }
        2 => {
            // Mult
            two_param_op_assign(data, *instr_ptr, |a: &i32, b: &i32| -> i32 { *a * *b });
            *instr_ptr += 4;
            Ok(true)
        }
        3 => {
            // Take input and assign at position
            let mut input_str = String::new();
            println!("(3) Input opcode. Provide input");
            scanline!(input_str);
            input_str.pop(); // removes newline
            let input = input_str.parse::<i32>().unwrap();
            let assign_index = data[*instr_ptr + 1] as usize;
            data[assign_index] = input;
            *instr_ptr += 2;
            Ok(true)
        }
        4 => {
            // Output value at param position/immediate
            let mode0 = read_mode(&data[*instr_ptr], &0);
            let param0 = if mode0 == 0 { data[data[*instr_ptr + 1] as usize] } else { data[*instr_ptr + 1] };
            println!("(4) Output opcode: {}", param0);
            *last_output = Some(param0);
            *instr_ptr += 2;
            Ok(true)
        }
        99 => Ok(false),
        _ => {
            println!("Illegal opcode ({})", opcode);
            panic!("wtf") 
        }
    }
}

fn two_param_op_assign(data: &mut Vec<i32>, index: usize, op: impl Fn(&i32, &i32) -> i32) {
    let mode0 = read_mode(&data[index], &0);
    let mode1 = read_mode(&data[index], &1);
    let param0 = if mode0 == 0 { data[data[index + 1] as usize] } else { data[index + 1] };
    let param1 = if mode1 == 0 { data[data[index + 2] as usize] } else { data[index + 2] };
    let assign_index = data[index + 3] as usize;
    data[assign_index] = op(&param0, &param1);
}

fn read_opcode(val: &i32) -> i32 {
    return val - dig(val, &2) * 10i32.pow(2) - dig(val, &3) * 10i32.pow(3) - dig(val, &4) * 10i32.pow(4)
}

fn read_mode(val: &i32, param: &i32) -> i32 {
    return dig(val, &(*param + 2));
}

pub fn dig(val: &i32, pwr: &i32) -> i32 {
    return val / 10i32.pow(*pwr as u32) % 10
}