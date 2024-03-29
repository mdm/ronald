use std::io::Write;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take_while, take_while1, take_while_m_n},
    combinator::{map_res, opt},
    sequence::{delimited, pair, separated_pair}
};

use crate::{cpu::{Cpu, Register16}, memory::Memory};

#[derive(Debug)]
enum Command {
    ToggleBreakpoint(u16),
    ShowCpuRegisters,
    Step(u16),
    Continue,
    Disassemble(u16),
}

impl Command {
    fn parse(input: &str) -> IResult<&str, Command> {
        alt((
            parse_toggle_breakpoint,
            parse_show_cpu_registers,
            parse_step,
            parse_continue,
            parse_disassemble,
        ))(input)
    }
}

fn is_decimal_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn from_decimal_str(input: &str) -> Result<u16, std::num::ParseIntError> {
    input.parse::<u16>()
}

fn parse_decimal(input: &str) -> IResult<&str, u16> {
    map_res(
        take_while_m_n(1, 5, is_decimal_digit),
        from_decimal_str
    )(input)
}

fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

fn from_hex_str(input: &str) -> Result<u16, std::num::ParseIntError> {
    u16::from_str_radix(input, 16)
}

fn parse_hex(input: &str) -> IResult<&str, u16> {
    let (input, _) = tag("0x")(input)?;
    map_res(
        take_while_m_n(1, 4, is_hex_digit),
        from_hex_str
    )(input)
}

fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}

fn parse_toggle_breakpoint(input: &str) -> IResult<&str, Command> {
    let (input, (_, address)) = delimited(
        take_while(is_whitespace),
        separated_pair(
            alt((tag("breakpoint"), tag("break"), tag("b"))),
            take_while1(is_whitespace),
            alt((parse_hex, parse_decimal))
        ),
        take_while(is_whitespace)
    )(input)?;

    Ok((input, Command::ToggleBreakpoint(address)))
}

fn parse_show_cpu_registers(input: &str) -> IResult<&str, Command> {
    let (input, _) = alt((tag("registers"), tag("reg"), tag("r")))(input)?;

    Ok((input, Command::ShowCpuRegisters))
}

fn parse_step(input: &str) -> IResult<&str, Command> {
    let (input, _) = take_while(is_whitespace)(input)?;
    let (input, _) = alt((tag("step"), tag("s")))(input)?;

    let (input, argument) = opt(pair(
        take_while1(is_whitespace),
        alt((parse_hex, parse_decimal))
    ))(input)?;

    let (input, _) = take_while(is_whitespace)(input)?;

    match argument {
        Some((_, count)) => Ok((input, Command::Step(count))),
        None => Ok((input, Command::Step(0))),
    }
}

fn parse_continue(input: &str) -> IResult<&str, Command> {
    let (input, _) = alt((tag("continue"), tag("cont"), tag("c")))(input)?;

    Ok((input, Command::Continue))
}

fn parse_disassemble(input: &str) -> IResult<&str, Command> {
    let (input, _) = alt((tag("disassemble"), tag("dis"), tag("d")))(input)?;

    Ok((input, Command::Disassemble(10)))
}


pub struct Debugger {
    breakpoints: Vec<u16>,
    countdown: Option<u16>,
}

impl Debugger {
    pub fn new() -> Self {
        Debugger {
            breakpoints: Vec::new(),
            countdown: None,
        }
    }

    pub fn activate(&mut self) {
        self.countdown = Some(0);
    }

    pub fn is_active(&mut self, cpu: &Cpu) -> bool {
        let address = cpu.registers.read_word(&Register16::PC);
        if self.breakpoint_at(address) {
            return true;
        }

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

    pub fn run_command_shell(&mut self, cpu: &mut Cpu, memory: &Memory) {
        let address = cpu.registers.read_word(&Register16::PC) as usize;
        let (instruction, _) = cpu.decoder.decode_at(memory, address);
        println!("{:#06x}: {}", address, &instruction);

        loop {
            print!("> ");
            std::io::stdout().flush().unwrap(); // TODO: is the a better way to handle the result?

            let mut input = String::new();
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
                                
                                println!("Active breakpoints:");
                                for breakpoint in &self.breakpoints {
                                    println!("{:#06x}", breakpoint);
                                }
                            }
                            Command::ShowCpuRegisters => {
                                cpu.print_state();
                            }
                            Command::Step(skip) => {
                                self.countdown = Some(skip);
                                break;
                            }
                            Command::Continue => {
                                break;
                            }
                            Command::Disassemble(count) => {
                                let mut address = cpu.registers.read_word(&Register16::PC) as usize;
                                for _ in 0..count {
                                    let (instruction, next_adress) = cpu.decoder.decode_at(memory, address);
                                    println!("{:#06x}: {}", address, &instruction);
                                    address = next_adress;                    
                                }
                            }
                        }
                    }
                }
                Err(error) => {
                    log::error!("Error reading from stdin: {}", error);
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