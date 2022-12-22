use crate::gates::Gates::*;


mod gates;

//use gates::{FiveLogic,Gate};

fn main() {
    
    //let mut and1 = ANDGate::new();

    //and1.input_a = FiveLogic::X;
    //and1.input_b = FiveLogic::X;

    //and1.eval();

    let (gates, wires, inputs, outputs) = gates::parsegates();

    println!("");

    for g in gates.gatestack {
        match g {
            AND(gate) => println!("AND gate, in1:{} in2:{} out:{}",gate.net_in_a,gate.net_in_b,gate.net_out), 
            NAND(gate) => println!("NAND gate, in1:{} in2:{} out:{}",gate.net_in_a,gate.net_in_b,gate.net_out),
            OR(gate) => println!("OR gate, in1:{} in2:{} out:{}",gate.net_in_a,gate.net_in_b,gate.net_out),
            NOR(gate) => println!("NOR gate, in1:{} in2:{} out:{}",gate.net_in_a,gate.net_in_b,gate.net_out),
            INV(gate) => println!("INV gate, in1:{} out:{}",gate.net_in_a,gate.net_out),
            BUF(gate) => println!("BUF gate, in1:{} out:{}",gate.net_in_a,gate.net_out),
        }
    }
    println!("");
    println!("");

    for w in wires {
        println!("Net {} has the following gates as fanouts:",w.0);
        for f in w.1.fanout {
            print!("{} ",f);
        }
        println!("");
        println!("");
    }

    println!("Following nets are primary inputs:");
    for i in inputs {
        print!("{} ",i);
    }
    println!("");
    println!("");

    println!("Following nets are primary outputs:");
    for o in outputs {
        print!("{} ",o);
    }



}

