use nom::IResult;

use crate::bus;
use crate::cpu;
use crate::memory;


enum Command {
    ToggleBreakpoint(u16),
    ShowCpuRegisters,
    Step(u16),
    Run,
    Disassemble(u16),
}

impl Command {
    fn parse(input: &str) -> IResult<&str, Command> {
        Ok((input, Command::Run))
    }
}

pub struct Debugger<M, B> {
    cpu: cpu::CPUShared<M, B>,
    breakpoints: Vec<u16>,
    countdown: Option<u16>,
}

impl<M, B> Debugger<M, B>
where
    M: memory::Read + memory::Write,
    B: bus::Bus,
{
    pub fn new_shared(cpu: cpu::CPUShared<M, B>) -> Debugger<M, B> {
        Debugger {
            cpu,
            breakpoints: Vec::new(),
            countdown: None,
        }
    }

    pub fn activate(&mut self) {
        self.countdown = Some(0);
    }

    pub fn is_active(&mut self) -> bool {
        match self.countdown {
            Some(countdown) => {
                if countdown == 0 {
                    self.countdown = None;
                    true
                } else {
                    self.countdown = Some(countdown - 1);
                    false
                }
            }
            None => false,
        }
    }

    pub fn run_command_shell(&mut self) {
        let address = self.cpu.borrow().registers.read_word(&cpu::Register16::PC) as usize;
        let (instruction, _) = self.cpu.borrow_mut().decoder.decode_at(address);
        println!("{:#06x}: {}", address, &instruction);

        let mut input = String::new();
        loop {
            match std::io::stdin().read_line(&mut input) {
                Ok(_) => {
                    if let Ok((_, command)) = Command::parse(&input) {
                        match command {
                            Command::ToggleBreakpoint(address) => {
                                if self.breakpoint_at(address) {
                                    self.remove_breakpoint(address);
                                } else {
                                    self.add_breakpoint(address);
                                }
                            }
                            Command::ShowCpuRegisters => {
                                self.cpu.borrow().print_state();
                            }
                            Command::Step(count) => {
                                self.countdown = Some(count);
                                break;
                            }
                            Command::Run => {
                                break;
                            }
                            Command::Disassemble(count) => {
                                let mut address = self.cpu.borrow().registers.read_word(&cpu::Register16::PC) as usize;
                                for _ in 0..count {
                                    let (instruction, next_adress) = self.cpu.borrow_mut().decoder.decode_at(address);
                                    println!("{:#06x}: {}", address, &instruction);
                                    address = next_adress;                    
                                }
                            }
                        }
                    }
                    unimplemented!();
                }
                Err(error) => {
                    println!("Error reading from stdin: {}", error);
                    break;
                }
            }
        }
    }

    fn breakpoint_at(&self, address: u16) -> bool {
        for breakpoint in &self.breakpoints {
            if *breakpoint == address {
                return true;
            }
        }
        
        false
    }
    
    fn add_breakpoint(&mut self, address: u16) {
        self.breakpoints.push(address);
    }

    fn remove_breakpoint(&mut self, address: u16) {
        self.breakpoints = self.breakpoints.iter().filter(|breakpoint| **breakpoint != address ).copied().collect();
    }
}