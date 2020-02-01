use crate::intcode::{CPUState, IntCodeCPU};

macro_rules! scanline {
    ($x:expr) => {
        std::io::stdin().read_line(&mut $x).unwrap();
    };
}

pub fn part1(interactive: bool, show_output: bool) -> std::io::Result<String> {
    let mut cpu = IntCodeCPU::new();
    cpu.read_data_file("input/d25.txt")?;
    let mut res = "".to_owned();
    if interactive {
        let mut output_str = "".to_owned();
        loop {
            cpu.execute();
            if cpu.state == CPUState::WaitingForInput {
                if cpu.has_output() {
                    output_str = cpu.read_ascii_output();
                    print!("{}", output_str);
                    cpu.output.clear();
                }
                if output_str.trim().contains("Command") {
                    let mut input_str = String::new();
                    scanline!(input_str);
                    input_str.pop();
                    // counter-strike controls
                    match input_str.as_str() {
                        "w" => {
                            cpu.enqueue_ascii("north\n".to_owned());
                        }
                        "a" => {
                            cpu.enqueue_ascii("west\n".to_owned());
                        }
                        "s" => {
                            cpu.enqueue_ascii("south\n".to_owned());
                        }
                        "d" => {
                            cpu.enqueue_ascii("east\n".to_owned());
                        }
                        "i" => {
                            cpu.enqueue_ascii("inv\n".to_owned());
                        }
                        _ => {
                            if input_str.starts_with("drop ") || input_str.starts_with("take ") {
                                cpu.enqueue_ascii(format!("{}{}", input_str, "\n".to_owned()));
                            } else {
                                println!("unknown command");
                            }
                        }
                    }
                }
            } else if cpu.state == CPUState::Halted {
                break;
            }
        }
    } else {
        let get_all = vec![
            "north",
            "take astronaut ice cream",
            "south",
            "west",
            "take mouse",
            "north",
            "take ornament",
            "west",
            "north",
            "take easter egg",
            "east",
            "take hypercube",
            "north",
            "east",
            "take prime number",
            "west",
            "south",
            "west",
            "north",
            "west",
            "north",
            "take wreath",
            "south",
            "east",  // obs
            "south", // nav
            "south", // arcade
            "west",  // warp
            "take mug",
            "west",
            "inv",
        ];
        let items = vec![
            "ornament",
            "hypercube",
            "mug",
            "prime number",
            "astronaut ice cream",
            "mouse",
            "wreath",
            "easter egg",
        ];
        get_all
            .iter()
            .map(|cmd| cmd.to_owned())
            .for_each(|cmd| cpu.enqueue_ascii(format!("{}{}", cmd.to_owned(), "\n".to_owned())));
        // save me itertools
        'outer: for a in 0..2 {
            take_if(&mut cpu, a, &items, 0);
            for b in 0..2 {
                take_if(&mut cpu, b, &items, 1);
                for c in 0..2 {
                    take_if(&mut cpu, c, &items, 2);
                    for d in 0..2 {
                        take_if(&mut cpu, d, &items, 3);
                        for e in 0..2 {
                            take_if(&mut cpu, e, &items, 4);
                            for f in 0..2 {
                                take_if(&mut cpu, f, &items, 5);
                                for g in 0..2 {
                                    take_if(&mut cpu, g, &items, 6);
                                    for h in 0..2 {
                                        take_if(&mut cpu, h, &items, 7);
                                        cpu.enqueue_ascii("north\n".to_owned());
                                        cpu.execute();
                                        if cpu.has_output() {
                                            let output_str = cpu.read_ascii_output();
                                            if show_output {
                                                print!("{}", output_str);
                                            }
                                            cpu.output.clear();
                                            if cpu.state == CPUState::Halted {
                                                let parts: Vec<String> = output_str
                                                    .split(' ')
                                                    .map(|s| s.to_string())
                                                    .collect();
                                                for i in 0..parts.len() {
                                                    if parts[i] == "typing".to_owned() {
                                                        res = parts[i + 1].clone();
                                                        break;
                                                    }
                                                }
                                                break 'outer;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if res == "".to_owned() {
        panic!("unexpected")
    }
    Ok(res)
}

fn take_if(cpu: &mut IntCodeCPU, bit: i32, all: &Vec<&str>, index: usize) {
    if bit == 1 {
        cpu.enqueue_ascii(format!("{}{}{}", "take ", all[index], "\n".to_owned()));
    } else {
        cpu.enqueue_ascii(format!("{}{}{}", "drop ", all[index], "\n".to_owned()));
    }
}
