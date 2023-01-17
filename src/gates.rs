use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;
use std::str::FromStr;
use std::collections::HashMap;
use ndarray::Array2;


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FiveLogic {
    ZERO,
    ONE,
    D,
    Dnot,
    X,
    U,
}

pub enum Gates {
    AND(ANDGate),
    OR(ORGate),
    NAND(NANDGate),
    NOR(NORGate),
    INV(NOTGate),
    BUF(BUFGate),
}

pub enum WireType{
    PrimaryInput,
    PrimaryOutput,
    Net,
}

pub struct Wire{
    pub net: u32,
    pub fanout: Vec<u32>,
    pub wiretype: WireType,
    pub level: FiveLogic,
}

type FaultMatrix = Array2::<u8>;
const SA0: u8 = 0x01;
const SA1: u8 = 0x02;

fn invert(value: &FiveLogic) -> FiveLogic {
    if *value == FiveLogic::ONE {
        FiveLogic::ZERO
    } else if *value == FiveLogic::ZERO {
        FiveLogic::ONE
    } else if *value == FiveLogic::D {
        FiveLogic::Dnot
    } else if *value == FiveLogic::Dnot {
        FiveLogic::D
    } else if *value == FiveLogic::X {
        FiveLogic::X
    } else {
        FiveLogic::U
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ANDGate {
    pub input_a: FiveLogic,
    pub input_b: FiveLogic,
    pub output: FiveLogic,
    pub net_in_a: u32,
    pub net_in_b: u32,
    pub net_out: u32,
}

/*impl ANDGate {
    pub fn new() -> Self {
        ANDGate {
            input_a: FiveLogic::ZERO,
            input_b: FiveLogic::ZERO,
            output: FiveLogic::ZERO,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        }
    }
}*/

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ORGate {
    pub input_a: FiveLogic,
    pub input_b: FiveLogic,
    pub output: FiveLogic,
    pub net_in_a: u32,
    pub net_in_b: u32,
    pub net_out: u32,
}

/*impl ORGate {
    pub fn new() -> Self {
        ORGate {
            input_a: FiveLogic::ZERO,
            input_b: FiveLogic::ZERO,
            output: FiveLogic::ZERO,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        }
    }
}*/

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct NOTGate {
    pub input_a: FiveLogic,
    pub output: FiveLogic,
    pub net_in_a: u32,
    pub net_out: u32,
}

/*impl NOTGate {
    pub fn new() -> Self {
        NOTGate {
            input_a: FiveLogic::ZERO,
            output: FiveLogic::ZERO,
            net_in_a: 0,
            net_out: 0,
        }
    }
}*/

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct NANDGate {
    pub input_a: FiveLogic,
    pub input_b: FiveLogic,
    pub output: FiveLogic,
    pub net_in_a: u32,
    pub net_in_b: u32,
    pub net_out: u32,
}

/*impl NANDGate {
    pub fn new() -> Self {
        NANDGate {
            input_a: FiveLogic::ZERO,
            input_b: FiveLogic::ZERO,
            output: FiveLogic::ZERO,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        }
    }
}*/

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct NORGate {
    pub input_a: FiveLogic,
    pub input_b: FiveLogic,
    pub output: FiveLogic,
    pub net_in_a: u32,
    pub net_in_b: u32,
    pub net_out: u32,
}

/*impl NORGate {
    pub fn new() -> Self {
        NORGate {
            input_a: FiveLogic::ZERO,
            input_b: FiveLogic::ZERO,
            output: FiveLogic::ZERO,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        }
    }
}*/

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BUFGate {
    pub input_a: FiveLogic,
    pub output: FiveLogic,
    pub net_in_a: u32,
    pub net_out: u32,
}

/*impl BUFGate {
    pub fn new() -> Self {
        BUFGate {
            input_a: FiveLogic::ZERO,
            output: FiveLogic::ZERO,
            net_in_a: 0,
            net_out: 0,
        }
    }
}*/


pub trait Gate {
    fn eval(&mut self);
    //fn new(&self) -> Self;
}

pub struct GateStack {
    pub gatestack: Vec<Gates>,
}

impl Gate for ANDGate{
    fn eval(&mut self) {
        if self.input_a == FiveLogic::ZERO || 
           self.input_b == FiveLogic::ZERO || 
           self.input_a == FiveLogic::D && self.input_b == FiveLogic::Dnot ||
           self.input_a == FiveLogic::Dnot && self.input_b == FiveLogic::D {
            self.output = FiveLogic::ZERO;
        } else if self.input_a == FiveLogic::X || self.input_b == FiveLogic::X {
            self.output = FiveLogic::X;
        } else if self.input_a == FiveLogic::ONE {
            self.output = self.input_b;
        } else if self.input_b == FiveLogic::ONE {
            self.output = self.input_a; 
        } else if self.input_a == FiveLogic::D && self.input_b == FiveLogic::D {
            self.output = FiveLogic::D;
        } else if self.input_a == FiveLogic::Dnot && self.input_b == FiveLogic::Dnot {
            self.output = FiveLogic::Dnot;
        }  
    }
}

impl Gate for NANDGate{
    fn eval(&mut self) {
        if self.input_a == FiveLogic::ZERO || self.input_b == FiveLogic::ZERO ||
           self.input_a == FiveLogic::D && self.input_b == FiveLogic::Dnot ||
           self.input_a == FiveLogic::Dnot && self.input_b == FiveLogic::D {
            self.output = invert(&FiveLogic::ZERO);
        } else if self.input_a == FiveLogic::X || self.input_b == FiveLogic::X {
            self.output = invert(&FiveLogic::X);
        } else if self.input_a == FiveLogic::ONE {
            self.output = invert(&self.input_b);
        } else if self.input_b == FiveLogic::ONE {
            self.output = invert(&self.input_a); 
        } else if self.input_a == FiveLogic::D && self.input_b == FiveLogic::D {
            self.output = invert(&FiveLogic::D);
        } else if self.input_a == FiveLogic::Dnot && self.input_b == FiveLogic::Dnot {
            self.output = invert(&FiveLogic::Dnot);
        }  
    }
}

impl Gate for ORGate{
    fn eval(&mut self) {
        if self.input_a == FiveLogic::ONE || 
           self.input_b == FiveLogic::ONE ||
           self.input_a == FiveLogic::Dnot && self.input_b == FiveLogic::D ||
           self.input_a == FiveLogic::D && self.input_b == FiveLogic::Dnot {
            self.output = FiveLogic::ONE;
        } else if self.input_a == FiveLogic::X || self.input_b == FiveLogic::X {
            self.output = FiveLogic::X;
        } else if self.input_a == FiveLogic::ZERO {
            self.output = self.input_b;
        } else if self.input_b == FiveLogic::ZERO {
            self.output = self.input_a; 
        } else if self.input_a == FiveLogic::D && self.input_b == FiveLogic::D {
            self.output = FiveLogic::D;
        } else if self.input_a == FiveLogic::Dnot && self.input_b == FiveLogic::Dnot {
            self.output = FiveLogic::Dnot;
        }  
    }
}

impl Gate for NORGate{
    fn eval(&mut self) {
        if self.input_a == FiveLogic::ONE || 
           self.input_b == FiveLogic::ONE || 
           self.input_a == FiveLogic::D && self.input_b == FiveLogic::Dnot ||
           self.input_a == FiveLogic::Dnot && self.input_b == FiveLogic::D {
            self.output = invert(&FiveLogic::ONE);
        } else if self.input_a == FiveLogic::X || self.input_b == FiveLogic::X {
            self.output = invert(&FiveLogic::X);
        } else if self.input_a == FiveLogic::ZERO {
            self.output = invert(&self.input_b);
        } else if self.input_b == FiveLogic::ZERO {
            self.output = invert(&self.input_a); 
        } else if self.input_a == FiveLogic::D && self.input_b == FiveLogic::D {
            self.output = invert(&FiveLogic::D);
        } else if self.input_a == FiveLogic::Dnot && self.input_b == FiveLogic::Dnot {
            self.output = invert(&FiveLogic::Dnot);
        }  
    }
}

impl Gate for NOTGate{
    fn eval(&mut self) {
        self.output = invert(&self.input_a);
    }
}

impl Gate for BUFGate{
    fn eval(&mut self) {
        self.output = self.input_a;
    }
}

pub fn parsefaults(filename: &str, linecount: u32) -> FaultMatrix {
    let mut faults = Array2::<u8>::zeros((linecount as usize,linecount as usize));

    if let Ok(lines) = read_lines(filename){
        for info in lines.flatten() {
            
            let mut token = info.split_whitespace();

            let line = token.next();
 
            match line {
                None => {
                    println!("Bad line in faultlist");
                    panic!();
                }
                Some(num) => {
                    let linenum: u32 = FromStr::from_str(num).unwrap();
                    let faulttype: u8 = FromStr::from_str(token.next().unwrap()).unwrap();

                    if faulttype == 0 {
                        faults[[linenum as usize,linenum as usize]] = SA0;
                    }
                    else if faulttype == 1 {
                        faults[[linenum as usize,linenum as usize]] = SA1;
                    }
                }
            }
        }
    }

    faults
}


pub fn parsegates(filename: &str) -> (GateStack, HashMap<u32,Wire>, Vec<u32>,Vec<u32>) {
    let mut gates = GateStack {gatestack: vec![]};
    let mut instack: Vec<u32> = vec![];
    let mut outstack: Vec<u32> = vec![];
    let mut gatecount: u32 = 0;
    let mut wires: HashMap<u32,Wire> = HashMap::new();

    if let Ok(lines) = read_lines(filename){
        for line in lines.flatten() {
            let mut token = line.split_whitespace();
                
            let gatetype = token.next();

            match gatetype {
                None => {
                    println!("Error, no gate type");
                    return (gates, wires, instack, outstack)
                },
                Some(gateop) => {
                    match gateop {
                        "AND" | "OR" | "NAND" | "NOR" => {
                            let in1 = FromStr::from_str(token.next().unwrap()).unwrap();
                            let in2 = FromStr::from_str(token.next().unwrap()).unwrap();
                            let out = FromStr::from_str(token.next().unwrap()).unwrap();


                            if let std::collections::hash_map::Entry::Vacant(e) = wires.entry(in1) {
                                let thiswire = Wire{net: in1, fanout: vec![gatecount], wiretype: WireType::Net, level: FiveLogic::U};
                                e.insert(thiswire);
                            } else if let Some(thiswire) = wires.get_mut(&in1) {
                                thiswire.fanout.push(gatecount);
                            }

                            if let std::collections::hash_map::Entry::Vacant(e) = wires.entry(in2) {
                                 let thiswire = Wire{net: in2, fanout: vec![gatecount], wiretype: WireType::Net, level: FiveLogic::U};
                                 e.insert(thiswire);
                            } else if let Some(thiswire) = wires.get_mut(&in2) {
                                thiswire.fanout.push(gatecount);
                            }

                            if let std::collections::hash_map::Entry::Vacant(e) = wires.entry(out) {
                                let thiswire = Wire{net: out, fanout: vec![], wiretype: WireType::Net, level: FiveLogic::U};
                                e.insert(thiswire);
                            } else if let Some(thiswire) = wires.get_mut(&out) {
                                thiswire.fanout.push(gatecount);
                            }

                            gatecount += 1;

                            match gateop {
                                "AND" => {
                                    gates.gatestack.push(
                                        Gates::AND(ANDGate {
                                            net_in_a: in1,
                                            net_in_b: in2,
                                            net_out: out,
                                            input_a: FiveLogic::U,
                                            input_b: FiveLogic::U,
                                            output: FiveLogic::U,
                                        })
                                    )
                                },
                                "OR" => {
                                    gates.gatestack.push(
                                        Gates::OR(ORGate {
                                            net_in_a: in1,
                                            net_in_b: in2,
                                            net_out: out,
                                            input_a: FiveLogic::U,
                                            input_b: FiveLogic::U,
                                            output: FiveLogic::U,
                                        })
                                    )
                                },
                                "NAND" => {
                                    gates.gatestack.push(
                                        Gates::NAND(NANDGate {
                                            net_in_a: in1,
                                            net_in_b: in2,
                                            net_out: out,
                                            input_a: FiveLogic::U,
                                            input_b: FiveLogic::U,
                                            output: FiveLogic::U,
                                        })
                                    )
                                },
                                "NOR" => {
                                    gates.gatestack.push(
                                        Gates::NOR(NORGate {
                                            net_in_a: in1,
                                            net_in_b: in2,
                                            net_out: out,
                                            input_a: FiveLogic::U,
                                            input_b: FiveLogic::U,
                                            output: FiveLogic::U,
                                        })
                                    )
                                },
                                _ => {},
                            }
                        
                            //println!("{:?} with input nets {:?} and {:?}, output net {:?}",gateop,in1,in2,out);
                        },
                        "INV" | "BUF" => {
                            let in1 = FromStr::from_str(token.next().unwrap()).unwrap();
                            let out = FromStr::from_str(token.next().unwrap()).unwrap();
                            
                            if let std::collections::hash_map::Entry::Vacant(e) = wires.entry(in1) {
                                let thiswire = Wire{net: in1, fanout: vec![gatecount], wiretype: WireType::Net, level: FiveLogic::U};
                                e.insert(thiswire);
                            } else if let Some(thiswire) = wires.get_mut(&in1) {
                                thiswire.fanout.push(gatecount);
                            }

                            if let std::collections::hash_map::Entry::Vacant(e) = wires.entry(out) {
                                let thiswire = Wire{net: out, fanout: vec![], wiretype: WireType::Net, level: FiveLogic::U};
                                e.insert(thiswire);
                            } else if let Some(thiswire) = wires.get_mut(&out) {
                                thiswire.fanout.push(gatecount);
                            }
    
                            gatecount += 1;

                            match gateop {
                                "INV" => {
                                    gates.gatestack.push(
                                        Gates::INV(NOTGate {
                                            net_in_a: in1,
                                            net_out: out,
                                            input_a: FiveLogic::U,
                                            output: FiveLogic::U,
                                        })
                                    )
                                },
                                "BUF" => {
                                    gates.gatestack.push(
                                        Gates::BUF(BUFGate {
                                            net_in_a: in1,
                                            net_out: out,
                                            input_a: FiveLogic::U,
                                            output: FiveLogic::U,
                                        })
                                    )
                                },
                                _ => {},
                            }

                            //println!("{:?} with input net {:?}, output net {:?}",gateop,in1,out);
                        },
                        "INPUT" => {
                            for i in token {
                                let input = i.parse::<i32>().unwrap();
                                
                                let wirenum = input as u32;

                                if let Some(thiswire) = wires.get_mut(&wirenum) {
                                    thiswire.wiretype = WireType::PrimaryInput;
                                }
                                    
                                if input != -1 {
                                    instack.push(input as u32);
                                }
                            }
                        },
                        "OUTPUT" => {
                            for i in token {
                                let output = i.parse::<i32>().unwrap();
                                let wirenum = output as u32;

                                if let Some(thiswire) = wires.get_mut(&wirenum) {
                                    thiswire.wiretype = WireType::PrimaryOutput;
                                }

                                if output != -1 {
                                    outstack.push(output as u32);
                                }
                            }
                        },
                        _ => {
                            println!("Error, invalid gate entry");
                            return (gates, wires, instack, outstack)
                        }
                    }
                },
            }        
        }
    }

    (gates, wires, instack, outstack)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn evalline(currentwire: u32, gates: &mut GateStack, wires: &mut HashMap<u32,Wire>) {
    
    let line = wires.entry(currentwire)
                                .or_insert(Wire{net: currentwire, fanout: vec![], wiretype: WireType::Net, level: FiveLogic::U});

    let fanout = line.fanout.clone();

    for i in fanout {
        let gatetype = gates.gatestack.get_mut(i as usize).unwrap();

        match gatetype {
            Gates::AND(ref mut gate) => {
                let neta = gate.net_in_a;
                let netb = gate.net_in_b;
                let netout = gate.net_out;
                let output = gate.output;

                gate.input_a = wires.get(&neta).unwrap().level;
                gate.input_b = wires.get(&netb).unwrap().level;

                if gate.input_a != FiveLogic::U && gate.input_b != FiveLogic::U && output == FiveLogic::U {                   
                    
                    gate.eval();

                    let outnet = wires.entry(netout)
                                        .or_insert(Wire{net: gate.net_in_a, fanout: vec![], wiretype: WireType::Net, level: gate.output});

                    outnet.level = gate.output;

                    evalline(outnet.net, gates, wires);

                }     
            },
            Gates::NAND(ref mut gate) => {
                let neta = gate.net_in_a;
                let netb = gate.net_in_b;
                let netout = gate.net_out;
                let output = gate.output;

                gate.input_a = wires.get(&neta).unwrap().level;
                gate.input_b = wires.get(&netb).unwrap().level;

                if gate.input_a != FiveLogic::U && gate.input_b != FiveLogic::U && output == FiveLogic::U {                   
                    
                    gate.eval();

                    let outnet = wires.entry(netout)
                                        .or_insert(Wire{net: gate.net_in_a, fanout: vec![], wiretype: WireType::Net, level: gate.output});

                    outnet.level = gate.output;

                    evalline(outnet.net, gates, wires);
                }
            },
            Gates::OR(ref mut gate) => {
                let neta = gate.net_in_a;
                let netb = gate.net_in_b;
                let netout = gate.net_out;
                let output = gate.output;

                gate.input_a = wires.get(&neta).unwrap().level;
                gate.input_b = wires.get(&netb).unwrap().level;

                if gate.input_a != FiveLogic::U && gate.input_b != FiveLogic::U && output == FiveLogic::U {                   
                    
                    gate.eval();

                    let outnet = wires.entry(netout)
                                        .or_insert(Wire{net: gate.net_in_a, fanout: vec![], wiretype: WireType::Net, level: gate.output});

                    outnet.level = gate.output;

                    evalline(outnet.net, gates, wires);
                }
            },
            Gates::NOR(ref mut gate) => {
                let neta = gate.net_in_a;
                let netb = gate.net_in_b;
                let netout = gate.net_out;
                let output = gate.output;

                gate.input_a = wires.get(&neta).unwrap().level;
                gate.input_b = wires.get(&netb).unwrap().level;

                if gate.input_a != FiveLogic::U && gate.input_b != FiveLogic::U && output == FiveLogic::U {                   
                    
                    gate.eval();

                    let outnet = wires.entry(netout)
                                        .or_insert(Wire{net: gate.net_in_a, fanout: vec![], wiretype: WireType::Net, level: gate.output});

                    outnet.level = gate.output;

                    evalline(outnet.net, gates, wires);
                }
            },
            Gates::INV(ref mut gate) => {
                let neta = gate.net_in_a;
                let netout = gate.net_out;
                let output = gate.output;

                gate.input_a = wires.get(&neta).unwrap().level;
                
                if gate.input_a != FiveLogic::U || output == FiveLogic::U {                   
                    
                    gate.eval();

                    let outnet = wires.entry(netout)
                                        .or_insert(Wire{net: gate.net_in_a, fanout: vec![], wiretype: WireType::Net, level: gate.output});

                    outnet.level = gate.output;

                    evalline(outnet.net, gates, wires);
                }
            },
            Gates::BUF(ref mut gate) => {
                let neta = gate.net_in_a;
                let netout = gate.net_out;
                let output = gate.output;

                gate.input_a = wires.get(&neta).unwrap().level;
                
                if gate.input_a != FiveLogic::U || output == FiveLogic::U {                   
                    
                    gate.eval();

                    let outnet = wires.entry(netout)
                                        .or_insert(Wire{net: gate.net_in_a, fanout: vec![], wiretype: WireType::Net, level: gate.output});

                    outnet.level = gate.output;

                    evalline(outnet.net, gates, wires);
                }
            },
        }
    }
    

}

pub fn logic (gates: &mut GateStack, wires: &mut HashMap<u32, Wire>, inputs: Vec<u32>, outputs: Vec<u32>, inputvec: Vec<u8>) {
    //let mut m: usize = 0;

    for (m, ins) in inputs.iter().enumerate() {//ins in &inputs {
        let wire = wires.entry(*ins).or_insert(Wire{net: *ins, fanout: vec![], wiretype: WireType::Net, level: FiveLogic::X});

        match inputvec[m] {
            0 => wire.level = FiveLogic::ZERO,
            1 => wire.level = FiveLogic::ONE,
            _ => wire.level = FiveLogic::X, 
        }
        
        //m += 1;
    }

    for i in &inputs {
        evalline(*i, gates, wires);

        let mut terminate = true;

        for outs in &outputs {
            let outnet = wires.get(outs).unwrap();
            if outnet.level == FiveLogic::U {
                terminate = false;
            }
        }

        if terminate {
            break
        }

    }

    println!("");
    println!("Circuit outputs:");

    for o in &outputs {
        print!("{} ",o);
    }

    println!("");
    println!("");
    println!("Output vector:");

    for o in &outputs {
        let outnet = wires.get(o).unwrap();

        match outnet.level {
            FiveLogic::ONE => print!("1 "),
            FiveLogic::ZERO => print!("0 "),
            FiveLogic::X => print!("X "),
            FiveLogic::D => print!("D "),
            FiveLogic::Dnot => print!("d "),
            FiveLogic::U => print!("U ")
        }

        //println!("Net {:?} value is {:?}",*o,outnet.level);
    }
    println!("");

}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn and_0_b() {
        let mut gate = ANDGate {
            input_a: FiveLogic::ZERO,
            input_b: FiveLogic::ONE,
            output: FiveLogic::X,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,FiveLogic::ZERO);
        assert_ne!(gate.output,FiveLogic::X);
    }

    #[test]
    fn and_a_0() {
        let mut gate = ANDGate {
            input_a: FiveLogic::ONE,
            input_b: FiveLogic::ZERO,
            output: FiveLogic::X,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,FiveLogic::ZERO);
        assert_ne!(gate.output,FiveLogic::X);
    }

    #[test]
    fn and_1_b() {
        let mut gate = ANDGate {
            input_a: FiveLogic::ONE,
            input_b: FiveLogic::D,
            output: FiveLogic::X,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,gate.input_b);
        assert_ne!(gate.output,FiveLogic::X);
    }

    #[test]
    fn and_a_1() {
        let mut gate = ANDGate {
            input_a: FiveLogic::D,
            input_b: FiveLogic::ONE,
            output: FiveLogic::X,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,gate.input_a);
        assert_ne!(gate.output,FiveLogic::X);
    }    

    #[test]
    fn and_x_b() {
        let mut gate = ANDGate {
            input_a: FiveLogic::X,
            input_b: FiveLogic::ONE,
            output: FiveLogic::ONE,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,FiveLogic::X);
        assert_ne!(gate.output,FiveLogic::ONE);
    }

    #[test]
    fn and_x_0() {
        let mut gate = ANDGate {
            input_a: FiveLogic::X,
            input_b: FiveLogic::ZERO,
            output: FiveLogic::ONE,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,FiveLogic::ZERO);
        assert_ne!(gate.output,FiveLogic::ONE);
    }

    #[test]
    fn and_a_x() {
        let mut gate = ANDGate {
            input_a: FiveLogic::ONE,
            input_b: FiveLogic::X,
            output: FiveLogic::ONE,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,FiveLogic::X);
        assert_ne!(gate.output,FiveLogic::ONE);
    }

    #[test]
    fn and_0_x() {
        let mut gate = ANDGate {
            input_a: FiveLogic::ZERO,
            input_b: FiveLogic::X,
            output: FiveLogic::ONE,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,FiveLogic::ZERO);
        assert_ne!(gate.output,FiveLogic::ONE);
    }

    #[test]
    fn and_dnot_d() {
        let mut gate = ANDGate {
            input_a: FiveLogic::D,
            input_b: FiveLogic::Dnot,
            output: FiveLogic::X,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,FiveLogic::ZERO);
        assert_ne!(gate.output,FiveLogic::X);
    }

    #[test]
    fn and_dnot_dnot() {
        let mut gate = ANDGate {
            input_a: FiveLogic::Dnot,
            input_b: FiveLogic::Dnot,
            output: FiveLogic::X,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,FiveLogic::Dnot);
        assert_ne!(gate.output,FiveLogic::X);
    }

    #[test]
    fn and_d_d() {
        let mut gate = ANDGate {
            input_a: FiveLogic::D,
            input_b: FiveLogic::D,
            output: FiveLogic::X,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,FiveLogic::D);
        assert_ne!(gate.output,FiveLogic::X);
    }

    #[test]
    fn and_d_dnot() {
        let mut gate = ANDGate {
            input_a: FiveLogic::D,
            input_b: FiveLogic::Dnot,
            output: FiveLogic::X,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,FiveLogic::ZERO);
        assert_ne!(gate.output,FiveLogic::X);
    }

    #[test]
    fn not_0() {
        let mut gate: NOTGate = NOTGate {
            input_a: FiveLogic::ZERO,
            output: FiveLogic::X,
            net_in_a: 0,
            net_out: 0,
        };

        gate.eval();

        assert_eq!(gate.output,invert(&gate.input_a));
        assert_ne!(gate.output,gate.input_a);
    }

    #[test]
    fn not_1() {
        let mut gate: NOTGate = NOTGate {
            input_a: FiveLogic::ONE,
            output: FiveLogic::X,
            net_in_a: 0,
            net_out: 0,
        };

        gate.eval();

        assert_eq!(gate.output,invert(&gate.input_a));
        assert_ne!(gate.output,gate.input_a);
    }

    #[test]
    fn buf_0() {
        let mut gate: BUFGate = BUFGate {
            input_a: FiveLogic::ZERO,
            output: FiveLogic::X,
            net_in_a: 0,
            net_out: 0,
        };

        gate.eval();

        assert_eq!(gate.output,gate.input_a);
        assert_ne!(gate.output,invert(&gate.input_a));
    }

    #[test]
    fn buf_1() {
        let mut gate: BUFGate = BUFGate {
            input_a: FiveLogic::ONE,
            output: FiveLogic::X,
            net_in_a: 0,
            net_out: 0,
        };

        gate.eval();

        assert_eq!(gate.output,gate.input_a);
        assert_ne!(gate.output,invert(&gate.input_a));
    }

    #[test]
    fn nand_0_b() {
        let mut gate = NANDGate {
            input_a: FiveLogic::ZERO,
            input_b: FiveLogic::ONE,
            output: FiveLogic::X,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,invert(&FiveLogic::ZERO));
        assert_ne!(gate.output,FiveLogic::X);
    }

    #[test]
    fn nand_a_0() {
        let mut gate = NANDGate {
            input_a: FiveLogic::ONE,
            input_b: FiveLogic::ZERO,
            output: FiveLogic::X,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,invert(&FiveLogic::ZERO));
        assert_ne!(gate.output,FiveLogic::X);
    }

    #[test]
    fn nand_1_b() {
        let mut gate = NANDGate {
            input_a: FiveLogic::ONE,
            input_b: FiveLogic::D,
            output: FiveLogic::X,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,invert(&gate.input_b));
        assert_ne!(gate.output,FiveLogic::X);
    }

    #[test]
    fn nand_a_1() {
        let mut gate = NANDGate {
            input_a: FiveLogic::D,
            input_b: FiveLogic::ONE,
            output: FiveLogic::X,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,invert(&gate.input_a));
        assert_ne!(gate.output,FiveLogic::X);
    }    

    #[test]
    fn nand_x_b() {
        let mut gate = NANDGate {
            input_a: FiveLogic::X,
            input_b: FiveLogic::ONE,
            output: FiveLogic::ONE,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,invert(&FiveLogic::X));
        assert_ne!(gate.output,FiveLogic::ONE);
    }

    #[test]
    fn nand_x_0() {
        let mut gate = NANDGate {
            input_a: FiveLogic::X,
            input_b: FiveLogic::ZERO,
            output: FiveLogic::ONE,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,invert(&FiveLogic::ZERO));
        assert_ne!(gate.output,invert(&FiveLogic::ONE));
    }

    #[test]
    fn nand_a_x() {
        let mut gate = NANDGate {
            input_a: FiveLogic::ONE,
            input_b: FiveLogic::X,
            output: FiveLogic::ONE,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,invert(&FiveLogic::X));
        assert_ne!(gate.output,FiveLogic::ONE);
    }

    #[test]
    fn nand_0_x() {
        let mut gate = NANDGate {
            input_a: FiveLogic::ZERO,
            input_b: FiveLogic::X,
            output: FiveLogic::ONE,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,invert(&FiveLogic::ZERO));
        assert_ne!(gate.output,invert(&FiveLogic::ONE));
    }

    #[test]
    fn nand_dnot_d() {
        let mut gate = NANDGate {
            input_a: FiveLogic::D,
            input_b: FiveLogic::Dnot,
            output: FiveLogic::X,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,invert(&FiveLogic::ZERO));
        assert_ne!(gate.output,FiveLogic::X);
    }

    #[test]
    fn nand_dnot_dnot() {
        let mut gate = NANDGate {
            input_a: FiveLogic::Dnot,
            input_b: FiveLogic::Dnot,
            output: FiveLogic::X,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,invert(&FiveLogic::Dnot));
        assert_ne!(gate.output,FiveLogic::X);
    }

    #[test]
    fn nand_d_d() {
        let mut gate = NANDGate {
            input_a: FiveLogic::D,
            input_b: FiveLogic::D,
            output: FiveLogic::X,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,invert(&FiveLogic::D));
        assert_ne!(gate.output,FiveLogic::X);
    }

    #[test]
    fn nand_d_dnot() {
        let mut gate = NANDGate {
            input_a: FiveLogic::D,
            input_b: FiveLogic::Dnot,
            output: FiveLogic::X,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,invert(&FiveLogic::ZERO));
        assert_ne!(gate.output,FiveLogic::X);
    }

    #[test]
    fn or_0_b() {
        let mut gate = ORGate {
            input_a: FiveLogic::ZERO,
            input_b: FiveLogic::ONE,
            output: FiveLogic::X,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,gate.input_b);
        assert_ne!(gate.output,FiveLogic::X);
    }

    #[test]
    fn or_a_0() {
        let mut gate = ORGate {
            input_a: FiveLogic::ONE,
            input_b: FiveLogic::ZERO,
            output: FiveLogic::X,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,gate.input_a);
        assert_ne!(gate.output,FiveLogic::X);
    }

    #[test]
    fn or_1_b() {
        let mut gate = ORGate {
            input_a: FiveLogic::ONE,
            input_b: FiveLogic::D,
            output: FiveLogic::X,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,FiveLogic::ONE);
        assert_ne!(gate.output,FiveLogic::X);
    }

    #[test]
    fn or_a_1() {
        let mut gate = ORGate {
            input_a: FiveLogic::D,
            input_b: FiveLogic::ONE,
            output: FiveLogic::X,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,FiveLogic::ONE);
        assert_ne!(gate.output,FiveLogic::X);
    }    

    #[test]
    fn or_x_b() {
        let mut gate = ORGate {
            input_a: FiveLogic::X,
            input_b: FiveLogic::ONE,
            output: FiveLogic::ONE,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,FiveLogic::ONE);
        assert_ne!(gate.output,FiveLogic::X);
    }

    #[test]
    fn or_x_0() {
        let mut gate = ORGate {
            input_a: FiveLogic::X,
            input_b: FiveLogic::ZERO,
            output: FiveLogic::ONE,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,FiveLogic::X);
        assert_ne!(gate.output,FiveLogic::ONE);
    }

    #[test]
    fn or_a_x() {
        let mut gate = ORGate {
            input_a: FiveLogic::ONE,
            input_b: FiveLogic::X,
            output: FiveLogic::ONE,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,FiveLogic::ONE);
        assert_ne!(gate.output,FiveLogic::X);
    }

    #[test]
    fn or_0_x() {
        let mut gate = ORGate {
            input_a: FiveLogic::ZERO,
            input_b: FiveLogic::X,
            output: FiveLogic::ONE,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,FiveLogic::X);
        assert_ne!(gate.output,FiveLogic::ONE);
    }

    #[test]
    fn or_dnot_d() {
        let mut gate = ORGate {
            input_a: FiveLogic::D,
            input_b: FiveLogic::Dnot,
            output: FiveLogic::X,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,FiveLogic::ONE);
        assert_ne!(gate.output,FiveLogic::X);
    }

    #[test]
    fn or_dnot_dnot() {
        let mut gate = ORGate {
            input_a: FiveLogic::Dnot,
            input_b: FiveLogic::Dnot,
            output: FiveLogic::X,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,FiveLogic::Dnot);
        assert_ne!(gate.output,FiveLogic::X);
    }

    #[test]
    fn or_d_d() {
        let mut gate = ORGate {
            input_a: FiveLogic::D,
            input_b: FiveLogic::D,
            output: FiveLogic::X,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,FiveLogic::D);
        assert_ne!(gate.output,FiveLogic::X);
    }

    #[test]
    fn or_d_dnot() {
        let mut gate = ORGate {
            input_a: FiveLogic::D,
            input_b: FiveLogic::Dnot,
            output: FiveLogic::X,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,FiveLogic::ONE);
        assert_ne!(gate.output,FiveLogic::X);
    }

    #[test]
    fn nor_0_b() {
        let mut gate = NORGate {
            input_a: FiveLogic::ZERO,
            input_b: FiveLogic::ONE,
            output: FiveLogic::X,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,invert(&gate.input_b));
        assert_ne!(gate.output,invert(&FiveLogic::X));
    }

    #[test]
    fn nor_a_0() {
        let mut gate = NORGate {
            input_a: FiveLogic::ONE,
            input_b: FiveLogic::ZERO,
            output: FiveLogic::X,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,invert(&gate.input_a));
        assert_ne!(gate.output,invert(&FiveLogic::X));
    }

    #[test]
    fn nor_1_b() {
        let mut gate = NORGate {
            input_a: FiveLogic::ONE,
            input_b: FiveLogic::D,
            output: FiveLogic::X,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,invert(&FiveLogic::ONE));
        assert_ne!(gate.output,invert(&FiveLogic::X));
    }

    #[test]
    fn nor_a_1() {
        let mut gate = NORGate {
            input_a: FiveLogic::D,
            input_b: FiveLogic::ONE,
            output: FiveLogic::X,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,invert(&FiveLogic::ONE));
        assert_ne!(gate.output,invert(&FiveLogic::X));
    }    

    #[test]
    fn nor_x_b() {
        let mut gate = NORGate {
            input_a: FiveLogic::X,
            input_b: FiveLogic::ONE,
            output: FiveLogic::ONE,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,FiveLogic::ZERO);
        assert_ne!(gate.output,FiveLogic::ONE);
    }

    #[test]
    fn nor_x_0() {
        let mut gate = NORGate {
            input_a: FiveLogic::X,
            input_b: FiveLogic::ZERO,
            output: FiveLogic::ONE,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,FiveLogic::X);
        assert_ne!(gate.output,FiveLogic::ONE);
    }

    #[test]
    fn nor_a_x() {
        let mut gate = NORGate {
            input_a: FiveLogic::ONE,
            input_b: FiveLogic::X,
            output: FiveLogic::ONE,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,FiveLogic::ZERO);
        assert_ne!(gate.output,FiveLogic::ONE);
    }

    #[test]
    fn nor_0_x() {
        let mut gate = NORGate {
            input_a: FiveLogic::ZERO,
            input_b: FiveLogic::X,
            output: FiveLogic::ONE,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,invert(&FiveLogic::X));
        assert_ne!(gate.output,invert(&FiveLogic::ONE));
    }

    #[test]
    fn nor_dnot_d() {
        let mut gate = NORGate {
            input_a: FiveLogic::D,
            input_b: FiveLogic::Dnot,
            output: FiveLogic::X,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,invert(&FiveLogic::ONE));
        assert_ne!(gate.output,FiveLogic::X);
    }

    #[test]
    fn nor_dnot_dnot() {
        let mut gate = NORGate {
            input_a: FiveLogic::Dnot,
            input_b: FiveLogic::Dnot,
            output: FiveLogic::X,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,invert(&FiveLogic::Dnot));
        assert_ne!(gate.output,FiveLogic::X);
    }

    #[test]
    fn nor_d_d() {
        let mut gate = NORGate {
            input_a: FiveLogic::D,
            input_b: FiveLogic::D,
            output: FiveLogic::X,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,invert(&FiveLogic::D));
        assert_ne!(gate.output,FiveLogic::X);
    }

    #[test]
    fn nor_d_dnot() {
        let mut gate = NORGate {
            input_a: FiveLogic::D,
            input_b: FiveLogic::Dnot,
            output: FiveLogic::X,
            net_in_a: 0,
            net_in_b: 0,
            net_out: 0,
        };
        
        gate.eval();

        assert_eq!(gate.output,invert(&FiveLogic::ONE));
        assert_ne!(gate.output,FiveLogic::X);
    }

}