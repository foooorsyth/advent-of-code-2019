use std::collections::VecDeque;
use std::fs;

#[derive(PartialEq)]
pub enum CPUState {
    Ready,
    Running,
    WaitingForInput,
    Halted,
}

const ZERO_BUFFER_MULTIPLE: i32 = 1;

pub struct IntCodeCPU {
    data: Vec<i32>,
    data_end: usize,
    instr_ptr: usize,
    relative_base: usize,
    input: VecDeque<i32>,
    pub last_output: Option<i32>,
    pub state: CPUState,
    debug_log: bool,
}

impl IntCodeCPU {
    pub fn new() -> IntCodeCPU {
        return IntCodeCPU {
            data: Vec::new(),
            data_end: 0,
            instr_ptr: 0,
            relative_base: 0,
            input: VecDeque::new(),
            last_output: None,
            state: CPUState::Ready,
            debug_log: false,
        };
    }

    pub fn read_data_from_file(data_file: &'static str) -> std::io::Result<Vec<i32>> {
        let data_string: String = fs::read_to_string(data_file)?;
        let data: Vec<i32> = data_string
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        return Ok(data);
    }

    pub fn read_data_file(&mut self, data_file: &'static str) -> std::io::Result<()> {
        self.set_data(IntCodeCPU::read_data_from_file(data_file)?);
        Ok(())
    }

    pub fn set_data(&mut self, data: Vec<i32>) {
        self.data = data;
        let len = self.data.len();
        self.data_end = len - 1;
        let mut zero_buffer: Vec<i32> = vec![0, ZERO_BUFFER_MULTIPLE * len as i32];
        self.data.append(&mut zero_buffer);
    }

    pub fn get_data_at(&mut self, index: usize) -> i32 {
        return self.data[index];
    }

    pub fn set_data_at(&mut self, index: usize, value: i32) {
        self.data[index] = value;
        if index > self.data_end {
            self.data_end = index;
        }
    }

    pub fn enqueue_input(&mut self, input: i32) {
        self.input.push_back(input);
    }

    pub fn has_input(&mut self) -> bool {
        return self.input.len() > 0;
    }

    pub fn reset(&mut self, data: Vec<i32>) {
        self.set_data(data);
        self.instr_ptr = 0;
        self.relative_base = 0;
        self.input.clear();
        self.last_output = None;
        self.state = CPUState::Ready;
    }

    fn dequeue_input(&mut self) -> i32 {
        return self.input.pop_front().unwrap();
    }

    #[allow(dead_code)]
    pub fn set_debug_log(&mut self, debug: bool) {
        self.debug_log = debug;
    }

    pub fn execute(&mut self) {
        self.state = CPUState::Running;
        let input_len = self.data.len();
        while self.instr_ptr < input_len {
            if self.instruction() {
                return;
            }
        }
        panic!("wtf")
    }

    fn instruction(&mut self) -> bool {
        let opcode = IntCodeCPU::read_opcode(&self.data[self.instr_ptr]);
        match opcode {
            1 => {
                // Add
                self.two_param_op_assign(|a: &i32, b: &i32| -> i32 { *a + *b });
                false
            }
            2 => {
                // Mult
                self.two_param_op_assign(|a: &i32, b: &i32| -> i32 { *a * *b });
                false
            }
            3 => {
                // Take input and assign at position
                if self.debug_log {
                    println!("(3) Input opcode. Provide input");
                }
                if self.has_input() {
                    let input_val = self.dequeue_input();
                    if self.debug_log {
                        println!("(3) Using provided input: {}", input_val);
                    }
                    let assign_index = self.data[self.instr_ptr + 1] as usize;
                    self.set_data_at(assign_index, input_val);
                    self.instr_ptr += 2;
                    false
                } else {
                    if self.debug_log {
                        println!("(3) No input available. Pausing and waiting for input...");
                    }
                    self.state = CPUState::WaitingForInput;
                    true
                }
            }
            4 => {
                // Output value at param position/immediate
                let mode0 = IntCodeCPU::read_mode(&self.data[self.instr_ptr], &0);
                let param0 = self.read_param(mode0, 0);
                if self.debug_log {
                    println!("(4) Output opcode: {}", param0);
                }
                self.last_output = Some(param0);
                self.instr_ptr += 2;
                false
            }
            5 => {
                // Jump if true
                self.jump_if(true);
                false
            }
            6 => {
                // Jump if false
                self.jump_if(false);
                false
            }
            7 => {
                // Less than
                self.two_param_op_assign(|a: &i32, b: &i32| -> i32 {
                    return if a < b { 1 } else { 0 };
                });
                false
            }
            8 => {
                // Equals
                self.two_param_op_assign(|a: &i32, b: &i32| -> i32 {
                    return if a == b { 1 } else { 0 };
                });
                false
            }
            9 => {
                // Adjust relative base

                false
            }
            99 => {
                // Halt
                self.state = CPUState::Halted;
                true
            }
            _ => {
                println!("Illegal opcode ({})", opcode);
                panic!("wtf")
            }
        }
    }

    fn read_two_params(&mut self) -> (i32, i32) {
        let mode0 = IntCodeCPU::read_mode(&self.data[self.instr_ptr], &0);
        let mode1 = IntCodeCPU::read_mode(&self.data[self.instr_ptr], &1);
        let param0 = self.read_param(mode0, 0);
        let param1 = self.read_param(mode1, 1);
        return (param0, param1);
    }

    fn read_two_params_with_assign(&mut self) -> (i32, i32, usize) {
        let (p0, p1) = self.read_two_params();
        let assign_index = self.data[self.instr_ptr + 3] as usize;
        return (p0, p1, assign_index);
    }

    fn two_param_op_assign(&mut self, op: impl Fn(&i32, &i32) -> i32) {
        let (param0, param1, assign_index) = self.read_two_params_with_assign();
        self.set_data_at(assign_index, op(&param0, &param1));
        self.instr_ptr += 4;
    }

    fn jump_if(&mut self, tf: bool) {
        let (param0, param1) = self.read_two_params();
        let zero = param0 == 0;
        if tf {
            self.instr_ptr = if !zero {
                param1 as usize
            } else {
                self.instr_ptr + 3
            }
        } else {
            self.instr_ptr = if zero {
                param1 as usize
            } else {
                self.instr_ptr + 3
            }
        }
    }

    fn read_opcode(val: &i32) -> i32 {
        return val
            - IntCodeCPU::dig(val, &2) * 10i32.pow(2)
            - IntCodeCPU::dig(val, &3) * 10i32.pow(3)
            - IntCodeCPU::dig(val, &4) * 10i32.pow(4);
    }

    fn read_mode(val: &i32, param: &i32) -> i32 {
        return IntCodeCPU::dig(val, &(*param + 2));
    }

    fn read_param(&mut self, mode: i32, pos: usize) -> i32 {
        match mode {
            0 => {
                self.data[self.data[self.instr_ptr + pos + 1] as usize]
            }
            1 => {
                self.data[self.instr_ptr + pos + 1]
            }
            2 => {
                self.data[self.relative_base + self.data[self.instr_ptr + pos + 1] as usize]
            }
            _ => { panic!("Illegal mode") }
        }
    }

    pub fn dig(val: &i32, pwr: &i32) -> i32 {
        return val / 10i32.pow(*pwr as u32) % 10;
    }
}
