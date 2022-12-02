use std::collections::VecDeque;

use super::Intcodes;

pub enum State {
    Running,
    Stopped,
    InputWait,
}

pub struct Computer {
    code: Intcodes,
    ip: usize,
    state: State,
    inq: VecDeque<isize>,
    outq: VecDeque<isize>,
}

impl Computer {
    pub fn load(code: Intcodes) -> Self {
        let inq = VecDeque::new();
        let outq = VecDeque::new();
        Self {
            code,
            ip: 0,
            state: State::Stopped,
            inq,
            outq,
        }
    }

    pub fn dump(self) -> Intcodes {
        self.code
    }

    pub fn status(self) -> State {
        self.state
    }

    pub fn input(mut self, value: isize) -> Result<Computer, String> {
        self.inq.push_back(value);
        self.state = State::Running;
        self.run()
    }

    pub fn output(&mut self) -> Option<isize> {
        self.outq.pop_front()
    }

    pub fn start(mut self) -> Result<Computer, String> {
        self.state = State::Running;
        self.run()
    }

    fn run(mut self) -> Result<Computer, String> {
        // let code = &mut self.code;
        // let ip = &mut self.ip;
        loop {
            let opcode = self.code[self.ip];
            match opcode % 100 {
                1 => {
                    // addition
                    let (a, b, c) = self.get_params3();
                    // let target = code[*ip + 3];
                    *c = a + b;
                    self.ip += 4
                }

                2 => {
                    // multiplication
                    let (a, b, c) = self.get_params3();
                    *c = a * b;
                    self.ip += 4
                }

                3 => {
                    //input
                    let input = match self.inq.pop_front() {
                        Some(i) => i,
                        None => {
                            self.state = State::InputWait;
                            return Ok(self);
                        }
                    };
                    let target = self.code[self.ip + 1] as usize;
                    self.code[target] = input;
                    self.ip += 2;
                }

                4 => {
                    let a = self.get_param1();
                    self.outq.push_back(a);
                    self.ip += 2;
                }

                5 => {
                    // jump-if-true
                    let (a, b) = self.get_params2();
                    if a > 0 {
                        self.ip = b as usize
                    } else {
                        self.ip += 3
                    }
                }

                6 => {
                    // jump-if-false
                    let (a, b) = self.get_params2();
                    if a == 0 {
                        self.ip = b as usize
                    } else {
                        self.ip += 3
                    }
                }

                7 => {
                    // less than
                    let (a, b, c) = self.get_params3();
                    *c = if a < b { 1 } else { 0 };
                    self.ip += 4;
                }

                8 => {
                    // equal
                    let (a, b, c) = self.get_params3();
                    *c = if a == b { 1 } else { 0 };
                    self.ip += 4;
                }

                99 => {
                    self.state = State::Stopped;
                    return Ok(self);
                }
                _ => {
                    return Err(format!(
                        "Unknown operand: {} at byte: {}",
                        self.code[self.ip], self.ip
                    ))
                }
            }
        }
    }

    fn get_params2(&mut self) -> (isize, isize) {
        let a = match self.code[self.ip] / 100 % 10 {
            0 => self.code[self.code[self.ip + 1] as usize],
            1 => self.code[self.ip + 1],
            _ => 0,
        };
        let b = match self.code[self.ip] / 1000 % 10 {
            0 => self.code[self.code[self.ip + 2] as usize],
            1 => self.code[self.ip + 2],
            _ => 0,
        };
        // let target = self.code[self.ip + 3] as usize;
        (a, b)
    }

    fn get_params3(&mut self) -> (isize, isize, &mut isize) {
        let a = match self.code[self.ip] / 100 % 10 {
            0 => self.code[self.code[self.ip + 1] as usize],
            1 => self.code[self.ip + 1],
            _ => 0,
        };
        let b = match self.code[self.ip] / 1000 % 10 {
            0 => self.code[self.code[self.ip + 2] as usize],
            1 => self.code[self.ip + 2],
            _ => 0,
        };
        let target = self.code[self.ip + 3] as usize;
        (a, b, &mut self.code[target])
    }

    fn get_param1(&mut self) -> isize {
        let a = match self.code[self.ip] / 100 % 10 {
            0 => self.code[self.code[self.ip + 1] as usize],
            1 => self.code[self.ip + 1],
            _ => 0,
        };
        a
    }
}
