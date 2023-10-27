use llvm_ir::Module;

fn main() {
    let m = Module::from_bc_path("/Users/benpye/git/rtos/test.bc").unwrap();

    // let filename = "/Users/benpye/git/rtos/target/riscv32imac-unknown-none-elf/debug/test_app";
    // let bytes = std::fs::read(filename).unwrap();
    // let funcs = stack_sizes::analyze_executable(&bytes).unwrap();
    // println!("funcs: {:?}", funcs);
}
