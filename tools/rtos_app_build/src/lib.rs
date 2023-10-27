use core::panic;
use std::collections::{BTreeMap, HashSet};

use handlebars::Handlebars;
use proc_macro2::Span;
use quote::quote;
use serde::{Deserialize, Serialize};
use syn::Ident;
use xshell::cmd;

#[derive(Debug, Serialize, Deserialize)]
struct MemoryRange {
    base: u32,
    size: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Peripheral {
    memory: MemoryRange,
    #[serde(default)]
    interrupts: BTreeMap<String, usize>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeviceConfig {
    family: String,
    flash: MemoryRange,
    ram: MemoryRange,
    peripherals: BTreeMap<String, Peripheral>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    target: TargetConfig,
    kernel: KernelConfig,
    tasks: BTreeMap<String, TaskConfig>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
struct MemoryConfig {
    stack: u32,
    data: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct TargetConfig {
    device: String,
    clock: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct KernelConfig {
    memory: MemoryConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct TaskConfig {
    priority: u8,
    #[serde(default)]
    boot: bool,
    memory: MemoryConfig,
    #[serde(default)]
    peripherals: Vec<String>,
}

#[derive(Debug, Serialize)]
struct MemoryRegion {
    base: u32,
    size: u32,
}

#[derive(Debug, Serialize)]
struct Interrupt {
    name: String,
    num: usize,
}

#[derive(Debug, Serialize)]
struct Task {
    name: String,
    priority: u8,
    boot: bool,
    base_address: Option<u32>,
    memory_config: MemoryConfig,
    memory_regions: Vec<MemoryRegion>,
    interrupts: Vec<Interrupt>,
}

#[derive(Debug, Serialize)]
struct AppConfig {
    kernel: KernelConfig,
    tasks: Vec<Task>,
    device: DeviceConfig,
    feature_assertions: Vec<String>,
}

pub fn build() {
    let out_dir: std::path::PathBuf = std::env::var_os("OUT_DIR").unwrap().into();
    let manifest_dir: std::path::PathBuf = std::env::var_os("CARGO_MANIFEST_DIR").unwrap().into();
    let llvm_plugin = std::env::var_os("CARGO_CDYLIB_FILE_RTOS_LLVM_PLUGIN").unwrap();

    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir.join("build.rs").to_str().unwrap()
    );

    let config_file_path = manifest_dir.join("app.toml");
    println!(
        "cargo:rerun-if-changed={}",
        config_file_path.to_str().unwrap()
    );

    let config_file = std::fs::read_to_string(config_file_path).unwrap();
    let config: Config = toml::from_str(&config_file).unwrap();

    let device_path = manifest_dir.join("../../device");
    println!("cargo:rerun-if-changed={}", device_path.to_str().unwrap());

    let device_config_path = device_path.join(&config.target.device).join("device.toml");
    let device_config_file = std::fs::read_to_string(device_config_path).unwrap();
    let device_config: DeviceConfig = toml::from_str(&device_config_file).unwrap();

    let total_kernel_memory = config.kernel.memory.stack + config.kernel.memory.data;

    if (total_kernel_memory & total_kernel_memory.wrapping_sub(1)) != 0 {
        panic!("Total kernel memory (stack + data) must be a power of two");
    }

    let mut claimed_peripherals = HashSet::new();
    let mut tasks: Vec<Task> = config
        .tasks
        .iter()
        .map(|(task_name, task_config)| {
            let mut memory_regions = Vec::new();
            let mut interrupts = Vec::new();

            let total_memory = task_config.memory.stack + task_config.memory.data;
            if (total_memory & total_memory.wrapping_sub(1)) != 0 {
                panic!(
                    "Total memory assigned to '{task_name}' (stack + data) must be a power of two"
                );
            }

            for peripheral_name in &task_config.peripherals {
                if !claimed_peripherals.insert(peripheral_name) {
                    panic!("Peripheral '{peripheral_name}' may only be claimed by one task");
                }

                let peripheral = device_config.peripherals.get(peripheral_name).unwrap();

                memory_regions.push(MemoryRegion {
                    base: peripheral.memory.base,
                    size: peripheral.memory.size,
                });

                for (interrupt_name, interrupt_num) in &peripheral.interrupts {
                    interrupts.push(Interrupt {
                        name: interrupt_name.clone(),
                        num: *interrupt_num,
                    });
                }
            }

            Task {
                name: task_name.clone(),
                priority: task_config.priority,
                boot: task_config.boot,
                base_address: None,
                memory_config: task_config.memory,
                memory_regions,
                interrupts,
            }
        })
        .collect();

    // Sorts from smallest to largest allocation, then reverse to get largest
    // to smallest.
    tasks.sort_unstable_by_key(|t| t.memory_config.data + t.memory_config.stack);
    tasks.reverse();

    let mut base_address = device_config.ram.base;

    // Assume the kernel stack should be first, this should protect against
    // undetected stack overflows.
    let _kernel_stack_base = base_address;
    base_address += config.kernel.memory.stack;
    let _kernel_data_base = base_address;
    base_address += config.kernel.memory.data;

    let mut allocations_remaining = tasks.len();
    'outer: while allocations_remaining > 0 {
        let npot = 2_u32.pow(base_address.trailing_zeros());

        // First try to find the task with the largest memory allocation that
        // is at most npot.
        for task in tasks.iter_mut() {
            if task.base_address.is_some() {
                continue;
            }

            if npot >= (task.memory_config.data + task.memory_config.stack) {
                task.base_address = Some(base_address);
                base_address += task.memory_config.data + task.memory_config.stack;
                allocations_remaining -= 1;
                continue 'outer;
            }
        }

        // Otherwise find the task with the smallest memory allocation larger
        // than npot, and align up the base address.
        for task in tasks.iter_mut().rev() {
            if task.base_address.is_some() {
                continue;
            }

            let npot = task.memory_config.data + task.memory_config.stack;
            base_address = (base_address + npot - 1) & !(npot - 1);
            task.base_address = Some(base_address);
            base_address += npot;
            allocations_remaining -= 1;
            continue 'outer;
        }
    }

    let memory_used = base_address - device_config.ram.base;
    if memory_used > device_config.ram.size {
        panic!(
            "Out of memory, needed {memory_used} bytes but only {} bytes available",
            device_config.ram.size
        );
    }

    tasks.sort_unstable_by_key(|t| t.base_address.unwrap());

    #[repr(u8)]
    #[derive(Debug, Clone, Copy)]
    #[allow(unused)]
    enum PmpMode {
        Off = 0,
        TopOfRange = 1,
        NaturallyAlignedFourByte = 2,
        NaturallyAlignedPowerTwo = 3,
    }

    let pmp_addr = |base: u32, size: u32| match size {
        0 => (PmpMode::Off, 0),
        2 => panic!("A two byte PMP region is not permitted"),
        4 => (PmpMode::NaturallyAlignedFourByte, base),
        _ => (
            PmpMode::NaturallyAlignedPowerTwo,
            (base >> 2) | ((size >> 3) - 1),
        ),
    };

    let pmp_cfg = |l: bool, a: PmpMode, x: bool, w: bool, r: bool| {
        ((r as u8) << 0) | ((w as u8) << 1) | ((x as u8) << 2) | ((a as u8) << 3) | ((l as u8) << 7)
    };

    let task_tokens = tasks.iter().map(|task| {
        if task.memory_regions.len() > 2 {
            panic!(
                "Only two MMIO regions supported per task, {} has {}",
                task.name,
                task.memory_regions.len()
            );
        }

        let mmio0 = task
            .memory_regions
            .get(0)
            .map_or((0, 0), |r| (r.base, r.size));
        let mmio1 = task
            .memory_regions
            .get(0)
            .map_or((0, 0), |r| (r.base, r.size));

        let pmp = [
            pmp_addr(device_config.flash.base, device_config.flash.size),
            pmp_addr(
                task.base_address.unwrap(),
                task.memory_config.stack + task.memory_config.data,
            ),
            pmp_addr(mmio0.0, mmio0.1),
            pmp_addr(mmio1.0, mmio1.1),
        ];

        let pmp_cfg = ((pmp_cfg(false, pmp[0].0, true, false, true) as u32) << 0)
            | ((pmp_cfg(false, pmp[1].0, false, true, true) as u32) << 8)
            | ((pmp_cfg(false, pmp[2].0, true, true, true) as u32) << 16)
            | ((pmp_cfg(false, pmp[3].0, true, true, true) as u32) << 24);

        let pmp_addr = [pmp[0].1, pmp[1].1, pmp[2].1, pmp[3].1];

        let start_symbol = format!("_start.{}", task.name);
        let priority = task.priority;

        let mut flags = quote! { ::kernel_types::task::Flags::empty() };
        if task.boot {
            flags = quote! { #flags.union( ::kernel_types::task::Flags::BOOT ) };
        }

        quote! {
            ::kernel_types::task::TaskDescriptor {
                init_pc: ::kernel_types::link_const!(#start_symbol),
                priority: #priority,
                flags: #flags,
                arch: ::kernel_types::arch::riscv::ArchTaskDescriptor {
                    pmp_addr: [
                        #(#pmp_addr),*
                    ],
                    pmp_cfg: #pmp_cfg,
                },
            }
        }
    });

    let task_id_tokens = tasks.iter().enumerate().map(|(idx, task)| {
        let symbol = format!("rtos.constant.{}.task_id", task.name);
        let idx = idx as u8;
        let ident = Ident::new(
            &format!("TASK_ID_{}", task.name.to_uppercase()),
            Span::call_site(),
        );
        quote! {
            #[link_section = ".rtos.must_optimise"]
            #[export_name = #symbol]
            static #ident: u8 = #idx;
        }
    });

    const TIME_US_PER_S: u64 = 1_000_000;

    let tick_frequency = config.target.clock;
    let us_per_tick = (((TIME_US_PER_S as u128) << 64) / (tick_frequency as u128)) as u64;

    let mut interrupt_descriptors = BTreeMap::new();
    for (task_id, task) in tasks.iter().enumerate() {
        let task_id = task_id as u8;
        for (idx, interrupt) in task.interrupts.iter().enumerate() {
            let notification = 1u32 << idx;
            interrupt_descriptors.insert(interrupt.num, (task_id, notification));
        }
    }

    let interrupt_min = interrupt_descriptors
        .first_key_value()
        .map_or(0, |(k, _)| *k);
    let interrupt_max = interrupt_descriptors.last_key_value().map(|(k, _)| *k);

    let interrupt_count =
        interrupt_max.map_or(0, |interrupt_max| interrupt_max - interrupt_min + 1);

    let interrupt_tokens = (0..interrupt_count).map(|i| {
        let interrupt_num = interrupt_min + i;
        interrupt_descriptors.get(&interrupt_num).map_or_else(
            || {
                quote! {
                    ::kernel_types::task::InterruptDescriptor::none()
                }
            },
            |(task_id, notification)| {
                quote! {
                    ::kernel_types::task::InterruptDescriptor::new(#task_id, #notification)
                }
            },
        )
    });

    let task_count = config.tasks.len();
    let app_code = quote! {
        // This panic handler is unused and exists only to ensure that the app
        // crate builds successfully.
        #[panic_handler]
        #[cfg(target_os = "none")]
        fn panic(_: &::core::panic::PanicInfo) -> ! { loop { } }

        #[::rtos_macros::rtos_export]
        static TIME_US_PER_TICK: u64 = #us_per_tick;

        #[::rtos_macros::rtos_export]
        static TIME_TICK_FREQUENCY: u32 = #tick_frequency;

        #[::rtos_macros::rtos_export]
        static TASK_DESCRIPTOR_TABLE: [::kernel_types::task::TaskDescriptor; #task_count] = [
            #(#task_tokens),*
        ];

        #[::rtos_macros::rtos_export]
        static INTERRUPT_MIN: usize = #interrupt_min;

        #[::rtos_macros::rtos_export]
        static INTERRUPT_COUNT: usize = #interrupt_count;

        #[::rtos_macros::rtos_export]
        static INTERRUPT_DESCRIPTOR_TABLE: [::kernel_types::task::InterruptDescriptor; #interrupt_count] = [
            #(#interrupt_tokens),*
        ];

        #(#task_id_tokens)*
    };

    let app_code_syn_file: syn::File = syn::parse2(app_code.into()).unwrap();
    let app_code_pretty = prettyplease::unparse(&app_code_syn_file);

    let sh = xshell::Shell::new().unwrap();

    let app_code_path = out_dir.join("app.rs");
    sh.write_file(&app_code_path, app_code_pretty).unwrap();

    let mut reg = Handlebars::new();
    reg.register_escape_fn(&handlebars::no_escape);
    reg.register_templates_directory(".ld.in", device_path)
        .unwrap();

    let mut feature_assertions = Vec::new();
    feature_assertions.push(format!("num_tasks_{}", config.tasks.len()));
    feature_assertions.push(format!("family_{}", device_config.family));

    let app_config = AppConfig {
        kernel: config.kernel,
        tasks,
        device: device_config,
        feature_assertions,
    };

    let linker_file_out_path = out_dir.join("build.ld");
    {
        let linker_file_out = std::fs::File::create(&linker_file_out_path).unwrap();
        reg.render_to_write(
            format!("{}/device", config.target.device).as_str(),
            &app_config,
            linker_file_out,
        )
        .unwrap();
    }

    println!(
        "cargo:rustc-link-arg=--Map={}",
        out_dir.join("map.txt").to_str().unwrap()
    );
    println!("cargo:rustc-link-arg=--icf=safe");
    println!(
        "cargo:rustc-link-arg=--script={}",
        linker_file_out_path.to_str().unwrap()
    );

    println!("cargo:rustc-link-arg=--mllvm=-enable-merge-functions");
    println!("cargo:rustc-link-arg=--mllvm=-mergefunc-use-aliases");
    println!("cargo:rustc-link-arg=--mllvm=-ir-outliner");

    println!("cargo:rustc-link-arg=--lto-whole-program-visibility");
    println!("cargo:rustc-link-arg=--lto-partitions=1");

    println!("cargo:rustc-link-arg=--mllvm=--stack-size-section");
    println!("cargo:rustc-link-arg=--mllvm=--lto-embed-bitcode=optimized");

    let obj_dir = out_dir.join("obj");
    sh.remove_path(&obj_dir).unwrap();

    let task_names = config.tasks.iter().map(|t| t.0);
    for lib in vec!["kernel".to_string()].iter().chain(task_names) {
        let _env_guard = sh.push_env("RTOS_COMPONENT", lib);

        let lib_in =
            std::env::var_os(format!("CARGO_STATICLIB_FILE_{}", lib.to_uppercase())).unwrap();

        let lib_dir = obj_dir.join(lib);
        sh.create_dir(&lib_dir).unwrap();

        cmd!(sh, "llvm-ar x --output {lib_dir} {lib_in}")
            .run()
            .unwrap();

        for obj_file in sh.read_dir(&lib_dir).unwrap() {
            cmd!(sh, "opt --load-pass-plugin={llvm_plugin} --passes=set-small-data-limit,rtos-rewrite-symbols -o {obj_file}.out {obj_file}").run().unwrap();
            println!("cargo:rustc-link-arg={}.out", obj_file.to_str().unwrap());
        }
    }

    println!("cargo:rustc-cfg=target_device=\"qemu-riscv32\"");
}
