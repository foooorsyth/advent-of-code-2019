use std::collections::VecDeque;
use std::fs;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CPUState {
    Ready,
    Running,
    WaitingForInput,
    Halted,
}

const ZERO_BUFFER_MULTIPLE: usize = 2;

pub struct IntCodeCPU {
    mem: Vec<i64>,
    mem_end: usize,
    instr_ptr: usize,
    relative_base: usize,
    input: VecDeque<i64>,
    pub output: Vec<i64>,
    pub last_output: Option<i64>,
    pub state: CPUState,
    debug_log: bool,
}

impl IntCodeCPU {
    pub fn new() -> IntCodeCPU {
        return IntCodeCPU {
            mem: Vec::new(),
            mem_end: 0,
            instr_ptr: 0,
            relative_base: 0,
            input: VecDeque::new(),
            output: Vec::new(),
            last_output: None,
            state: CPUState::Ready,
            debug_log: false,
        };
    }

    pub fn snapshot(&self) -> IntCodeCPU {
        let lo: Option<i64>;
        if self.last_output.is_some() {
            lo = Some(self.last_output.unwrap());
        } else {
            lo = None;
        }
        return IntCodeCPU {
            mem: self.mem.clone(),
            mem_end: self.mem_end,
            instr_ptr: self.instr_ptr,
            relative_base: self.relative_base,
            input: self.input.clone(),
            output: self.output.clone(),
            last_output: lo,
            state: self.state,
            debug_log: self.debug_log,
        };
    }

    pub fn load_snapshot(&mut self, snap: IntCodeCPU) {
        let lo: Option<i64>;
        if snap.last_output.is_some() {
            lo = Some(snap.last_output.unwrap());
        } else {
            lo = None;
        }
        self.mem = snap.mem.clone();
        self.mem_end = snap.mem_end;
        self.instr_ptr = snap.instr_ptr;
        self.relative_base = snap.relative_base;
        self.input = snap.input.clone();
        self.output = snap.output.clone();
        self.last_output = lo;
        self.state = snap.state;
        self.debug_log = snap.debug_log;
    }

    pub fn read_data_from_file(data_file: &'static str) -> std::io::Result<Vec<i64>> {
        let data_string: String = fs::read_to_string(data_file)?;
        let data: Vec<i64> = data_string
            .split(",")
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        return Ok(data);
    }

    pub fn read_data_file(&mut self, data_file: &'static str) -> std::io::Result<()> {
        self.set_mem(IntCodeCPU::read_data_from_file(data_file)?);
        Ok(())
    }

    pub fn set_mem(&mut self, data: Vec<i64>) {
        self.mem = data;
        let len = self.mem.len();
        self.mem_end = len - 1;
        let mut zero_buffer: Vec<i64> = vec![0; ZERO_BUFFER_MULTIPLE * len];
        self.mem.append(&mut zero_buffer);
    }

    pub fn get_mem_at(&mut self, index: usize) -> i64 {
        return self.mem[index];
    }

    pub fn set_mem_at(&mut self, index: usize, value: i64) {
        self.mem[index] = value;
        if index > self.mem_end {
            self.mem_end = index;
        }
    }

    pub fn enqueue_input(&mut self, input: i64) {
        self.input.push_back(input);
    }

    pub fn has_input(&mut self) -> bool {
        return self.input.len() > 0;
    }

    pub fn reset(&mut self, data: Vec<i64>) {
        self.set_mem(data);
        self.instr_ptr = 0;
        self.relative_base = 0;
        self.input.clear();
        self.last_output = None;
        self.state = CPUState::Ready;
    }

    fn dequeue_input(&mut self) -> i64 {
        return self.input.pop_front().unwrap();
    }

    #[allow(dead_code)]
    pub fn set_debug_log(&mut self, debug: bool) {
        self.debug_log = debug;
    }

    pub fn execute(&mut self) {
        self.state = CPUState::Running;
        let input_len = self.mem.len();
        while self.instr_ptr < input_len {
            if self.instruction() {
                return;
            }
        }
        panic!("wtf")
    }

    fn instruction(&mut self) -> bool {
        let instr = self.mem[self.instr_ptr];
        if self.debug_log {
            println!("instruction: {}", instr);
        }
        let opcode = IntCodeCPU::read_opcode(&instr);
        match opcode {
            1 => {
                // Add
                self.two_param_op_assign(|a: &i64, b: &i64| -> i64 { *a + *b });
                false
            }
            2 => {
                // Mult
                self.two_param_op_assign(|a: &i64, b: &i64| -> i64 { *a * *b });
                false
            }
            3 => {
                // Take input and assign at position
                if self.debug_log {
                    println!("(3) Input opcode. Provide input");
                }
                if self.has_input() {
                    let input_val = self.dequeue_input();
                    let assign_mode = IntCodeCPU::read_mode(&self.mem[self.instr_ptr], 0);
                    let assign_index = self.read_param(assign_mode, 0, true);
                    self.set_mem_at(assign_index as usize, input_val);
                    if self.debug_log {
                        println!(
                            "(3) Using provided input: {}, assigning to: {}",
                            input_val, assign_index
                        );
                    }
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
                let mode0 = IntCodeCPU::read_mode(&self.mem[self.instr_ptr], 0);
                let param0 = self.read_param(mode0, 0, false);
                if self.debug_log {
                    println!("(4) Output opcode: {}", param0);
                }
                self.output.push(param0);
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
                self.two_param_op_assign(|a: &i64, b: &i64| -> i64 {
                    return if a < b { 1 } else { 0 };
                });
                false
            }
            8 => {
                // Equals
                self.two_param_op_assign(|a: &i64, b: &i64| -> i64 {
                    return if a == b { 1 } else { 0 };
                });
                false
            }
            9 => {
                // Adjust relative base
                let mode0 = IntCodeCPU::read_mode(&self.mem[self.instr_ptr], 0);
                let param0 = self.read_param(mode0, 0, false);
                let new_relative_base = (self.relative_base as i64 + param0) as usize;
                if self.debug_log {
                    println!(
                        "(9) Adjusting relative base, old: {}, new: {}",
                        self.relative_base, new_relative_base
                    );
                }
                self.relative_base = new_relative_base;
                self.instr_ptr += 2;
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

    fn read_two_params(&mut self) -> (i64, i64) {
        let mode0 = IntCodeCPU::read_mode(&self.mem[self.instr_ptr], 0);
        let mode1 = IntCodeCPU::read_mode(&self.mem[self.instr_ptr], 1);
        let param0 = self.read_param(mode0, 0, false);
        let param1 = self.read_param(mode1, 1, false);
        return (param0, param1);
    }

    fn read_two_params_with_assign(&mut self) -> (i64, i64, usize) {
        let (p0, p1) = self.read_two_params();
        let assign_mode = IntCodeCPU::read_mode(&self.mem[self.instr_ptr], 2);
        let assign_index = self.read_param(assign_mode, 2, true);
        return (p0, p1, assign_index as usize);
    }

    fn two_param_op_assign(&mut self, op: impl Fn(&i64, &i64) -> i64) {
        let (param0, param1, assign_index) = self.read_two_params_with_assign();
        self.set_mem_at(assign_index, op(&param0, &param1));
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

    fn read_opcode(val: &i64) -> i64 {
        return val
            - IntCodeCPU::dig(val, &2) * 10i64.pow(2)
            - IntCodeCPU::dig(val, &3) * 10i64.pow(3)
            - IntCodeCPU::dig(val, &4) * 10i64.pow(4);
    }

    fn read_mode(val: &i64, param: i32) -> i64 {
        return IntCodeCPU::dig(val, &(param + 2));
    }

    fn read_param(&mut self, mode: i64, pos: usize, assign: bool) -> i64 {
        let value_in_data = self.mem[self.instr_ptr + pos + 1];
        if mode == 1 {
            return value_in_data;
        }
        let interior = match mode {
            0 => value_in_data,
            2 => (self.relative_base as i64) + value_in_data,
            _ => panic!("Illegal mode"),
        };

        return match assign {
            true => interior,
            false => self.mem[interior as usize],
        };
    }

    pub fn dig(val: &i64, pwr: &i32) -> i64 {
        return val / 10i64.pow(*pwr as u32) % 10;
    }
}
