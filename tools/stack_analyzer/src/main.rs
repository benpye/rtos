use std::{fs::File, io::Write};

use either::Either;
use llvm_ir::{
    instruction::Call, Constant, ConstantRef, Function, Instruction, Module, Name, Operand,
};
use petgraph::{
    dot::Dot,
    prelude::{DiGraph, DiGraphMap, GraphMap},
};

struct CallGraph<'a> {
    module: &'a llvm_ir::Module,
    graph: DiGraphMap<&'a str, ()>,
}

impl<'a> CallGraph<'a> {
    pub fn new(module: &'a llvm_ir::Module) -> Self {
        Self {
            module,
            graph: Default::default(),
        }
    }

    pub fn calculate(&mut self) {
        for func in &self.module.functions {
            self.graph.add_node(&func.name);
        }

        for func in &self.module.functions {
            for bb in &func.basic_blocks {
                for instr in &bb.instrs {
                    if let Instruction::Call(call) = instr {
                        self.add_call_edge(func, call);
                    }
                }
            }
        }
    }

    fn add_call_edge(&mut self, func: &'a Function, call: &'a Call) {
        if let Either::Right(Operand::ConstantOperand(cref)) = &call.function {
            if let Constant::GlobalReference {
                name: Name::Name(name),
                ..
            } = cref.as_ref()
            {
                self.graph.add_edge(&func.name, &name, ());
            }
        }
    }
}

fn main() {
    // let module = Module::from_bc_path("/Users/benpye/git/rtos/test.bc").unwrap();
    // let mut cg = CallGraph::new(&module);
    // cg.calculate();

    // let dot = Dot::new(&cg.graph);
    // let mut dot_file = File::create("/Users/benpye/git/rtos/test.dot").unwrap();
    // dot_file.write_fmt(format_args!("{:?}", dot)).unwrap();

    // let mut g: DiGraphMap<&str, ()> = petgraph::graphmap::GraphMap::default();

    // for func in &m.functions {
    //     g.add_node(&func.name);
    // }

    // let mut add_call_edge = |g: &'a mut DiGraphMap<&str, ()>, func: &'a Function, call: &'a Call| {
    //     if let Either::Right(Operand::ConstantOperand(cref)) = &call.function {
    //         if let Constant::GlobalReference { name: Name::Name(name), .. } = cref.as_ref() {
    //             g.add_edge(&func.name, &name, ());
    //         }
    //     }
    //     // g.add_edge(&func.name, , ()).unwrap();
    // };

    // for func in &m.functions {
    //     for bb in &func.basic_blocks {
    //         for instr in &bb.instrs {
    //             if let Instruction::Call(call) = instr {
    //                 add_call_edge(&mut g, func, call);
    //             }
    //         }
    //     }
    // }

    let filename = "/Users/benpye/git/rtos/target/riscv32imac-unknown-none-elf/debug/test_app";
    let bytes = std::fs::read(filename).unwrap();
    let funcs = stack_sizes::analyze_executable(&bytes).unwrap();
    println!("funcs: {:?}", funcs);
}
