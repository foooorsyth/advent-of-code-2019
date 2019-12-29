use std::fs;
use std::collections::HashMap;
use std::io;

macro_rules! scanline {
    ($x:expr) => {
       io::stdin().read_line(&mut $x).unwrap();
    };
}

#[allow(dead_code)]
pub fn execute(data_file: &'static str,
            output_pref: (/* true = value at pos, false = last_output */ bool, /* pos */ usize)) -> std::io::Result<i32> {
    return execute_impl(data_file, output_pref, None, None);
}

pub fn execute_with_overwrite(data_file: &'static str, output_pref: (bool, usize), overwrite: HashMap<usize, i32>) -> std::io::Result<i32> {
    return execute_impl(data_file, output_pref, Some(overwrite), None);
}

pub fn execute_with_input(data_file: &'static str, output_pref: (bool, usize), input_sequence: &Vec<i32>) -> std::io::Result<i32> {
    return execute_impl(data_file, output_pref, None, Some(input_sequence));
}

fn execute_impl(data_file: &'static str, output_pref: (bool, usize), 
        overwrite: Option<HashMap<usize, i32>>, 
        input_sequence: Option<&Vec<i32>>) -> std::io::Result<i32> {
    let data_string: String = fs::read_to_string(data_file)?;
    let mut data: Vec<i32> = data_string.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    if overwrite.is_some() {
        let ow = overwrite.unwrap();
        for kvp in ow {
            data[kvp.0] = kvp.1;
        }
    }
    let mut instr_ptr: usize = 0;
    let mut input_idx: usize = 0;
    let mut last_output = None;
    let input_len = data.len();
    while instr_ptr < input_len {
        let halt = instruction(&mut data, &mut instr_ptr, &mut last_output, &input_sequence, &mut input_idx)?;
        if halt {
            if output_pref.0 {
                return Ok(data[output_pref.1])
            } else {
                return Ok(last_output.unwrap())
            }
        }
    }
    panic!("wtf")
}

fn instruction(data: &mut Vec<i32>, instr_ptr: &mut usize, last_output: &mut Option<i32>, 
    input_sequence: &Option<&Vec<i32>>, input_idx: &mut usize) -> std::io::Result<bool> {
    let opcode = read_opcode(&data[*instr_ptr]);
    match opcode {
        1 => { 
            // Add
            two_param_op_assign(data, instr_ptr, |a: &i32, b: &i32| -> i32 { *a + *b });
            Ok(false)
        }
        2 => {
            // Mult
            two_param_op_assign(data, instr_ptr, |a: &i32, b: &i32| -> i32 { *a * *b });
            Ok(false)
        }
        3 => {
            // Take input and assign at position
            let input: i32;
            println!("(3) Input opcode. Provide input");
            if input_sequence.is_some() {
                let input_vec = input_sequence.unwrap();
                println!("(3) Using provided input: {}", input_vec[*input_idx]);
                input = input_vec[*input_idx];
                *input_idx += 1;
            } else {
                let mut input_str = String::new();
                scanline!(input_str);
                input_str.pop(); // removes newline
                input = input_str.parse::<i32>().unwrap();
            }
            let assign_index = data[*instr_ptr + 1] as usize;
            data[assign_index] = input;
            *instr_ptr += 2;
            Ok(false)
        }
        4 => {
            // Output value at param position/immediate
            let mode0 = read_mode(&data[*instr_ptr], &0);
            let param0 = if mode0 == 0 { data[data[*instr_ptr + 1] as usize] } else { data[*instr_ptr + 1] };
            println!("(4) Output opcode: {}", param0);
            *last_output = Some(param0);
            *instr_ptr += 2;
            Ok(false)
        }
        5 => {
            // Jump if true
            jump_if(data, instr_ptr, true);
            Ok(false)
        }
        6 => {
            // Jump if false
            jump_if(data, instr_ptr, false);
            Ok(false)
        }
        7 => {
            // Less than
            two_param_op_assign(data, instr_ptr, |a: &i32, b: &i32| -> i32 { 
                return if a < b { 1 } else { 0 }
            });
            Ok(false)
        }
        8 => {
            // Equals
            two_param_op_assign(data, instr_ptr, |a: &i32, b: &i32| -> i32 { 
                return if a == b { 1 } else { 0 }
            });
            Ok(false)
        }
        99 => Ok(true),
        _ => {
            println!("Illegal opcode ({})", opcode);
            panic!("wtf") 
        }
    }
}

fn read_two_params(data: &mut Vec<i32>, instr_ptr: &mut usize) -> (i32, i32) {
    let mode0 = read_mode(&data[*instr_ptr], &0);
    let mode1 = read_mode(&data[*instr_ptr], &1);
    let param0 = if mode0 == 0 { data[data[*instr_ptr + 1] as usize] } else { data[*instr_ptr + 1] };
    let param1 = if mode1 == 0 { data[data[*instr_ptr + 2] as usize] } else { data[*instr_ptr + 2] };
    return (param0, param1);
}

fn read_two_params_with_assign(data: &mut Vec<i32>, instr_ptr: &mut usize) -> (i32, i32, usize) {
    let (p0, p1) = read_two_params(data, instr_ptr);
    let assign_index = data[*instr_ptr + 3] as usize;
    return (p0, p1, assign_index);
}

fn two_param_op_assign(data: &mut Vec<i32>, instr_ptr: &mut usize, op: impl Fn(&i32, &i32) -> i32) {
    let (param0, param1, assign_index) = read_two_params_with_assign(data, instr_ptr);
    data[assign_index] = op(&param0, &param1);
    *instr_ptr += 4;
}

fn jump_if(data: &mut Vec<i32>, instr_ptr: &mut usize, tf: bool) {
    let (param0, param1) = read_two_params(data, instr_ptr);
    let zero = param0 == 0;
    if tf {
        *instr_ptr = if !zero { param1 as usize } else { *instr_ptr + 3 }
    } else {
        *instr_ptr = if zero { param1 as usize } else { *instr_ptr + 3 }
    }
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