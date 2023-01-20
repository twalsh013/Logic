use crate::gates::Gates::*;
use clap::Parser;

mod gates;

//use gates::{FiveLogic,Gate};
#[derive(Parser,Debug)]
#[command(name = "Logic Sim")]
#[command(author = "Taylor Walsh")]
#[command(version = "0.1.0")]
#[command(about = "Simulates boolean combinational logic circuits", long_about = None)]
struct Args{
    filename: String,

    verbose: Option<bool>,
    #[clap(short, long, value_parser, use_value_delimiter = true)]
    inputvec: Vec<u8>,

    faultlist: Option<String>,
}

fn main() {

    let cli = Args::parse();
    
    

    //let mut and1 = ANDGate::new();

    //and1.input_a = FiveLogic::X;
    //and1.input_b = FiveLogic::X;

    //and1.eval();

    let filename = cli.filename;

    let (mut gates, mut wires, inputs, outputs) = gates::parsegates(&filename);

    let mode = {
        cli.verbose.unwrap_or(false)
    };


    if mode {
        println!("");

        for g in &gates.gatestack[..] {
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

        for w in &wires {
            println!("Net {} has the following gates as fanouts:",w.0);
            for f in &w.1.fanout[..] {
                print!("{} ",f);
            }
            println!("");
            println!("");
        }

        println!("Following nets are primary inputs:");
        for i in &inputs[..] {
            print!("{} ",i);
        }
        println!("");
        println!("");

        println!("Following nets are primary outputs:");
        for o in &outputs[..] {
            print!("{} ",o);
        }
        println!("");
    }

    let faultmatrix: gates::FaultMatrix;

    match cli.faultlist {
        Some(flist) => {
            faultmatrix = gates::parsefaults(&flist,wires.len());
            gates::logic(&mut gates, &mut wires, inputs, outputs, cli.inputvec, true, Some(faultmatrix));
        },
        None => {
            gates::logic(&mut gates, &mut wires, inputs, outputs, cli.inputvec, false, None);                 
        }
    }    
}

