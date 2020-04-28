use structopt::StructOpt;

use wasmer_runtime_core::loader::Instance;
use wasmer_singlepass_backend::SinglePassCompiler;

use std::os::unix::net::{UnixListener, UnixStream};

#[derive(Debug, StructOpt)]
#[structopt(name = "kernel", about = "Kernel-mode WebAssembly service.")]
pub enum Kernel {
    #[structopt(name = "listen")]
    Listen(Listen),
}

#[derive(Debug, StructOpt)]
pub struct Listen {
    #[structopt(long = "socket")]
    socket: String,
}

const CMD_RUN_CODE: u32 = 0x901;
const CMD_READ_MEMORY: u32 = 0x902;
const CMD_WRITE_MEMORY: u32 = 0x903;

fn handle_client(mut stream: UnixStream) {
    use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
    use std::io::{Read, Write};
    let binary_size = stream.read_u32::<LittleEndian>().unwrap();
    if binary_size > 1048576 * 16 {
        println!("binary too large");
        return;
    }
    let mut wasm_binary: Vec<u8> = Vec::with_capacity(binary_size as usize);
    unsafe { wasm_binary.set_len(binary_size as usize) };
    stream.read_exact(&mut wasm_binary).unwrap();
    use wasmer_runtime_core::backend::{CompilerConfig, MemoryBoundCheckMode};
    let module = wasmer_runtime::compile_with_config_with(
        &wasm_binary[..],
        CompilerConfig {
            symbol_map: None,
            memory_bound_check_mode: MemoryBoundCheckMode::Disable,
            enforce_stack_check: true,
            track_state: false,
            features: Default::default(),
            ..Default::default()
        },
        &SinglePassCompiler::new(),
    )
    .unwrap();

    let mut import_object = wasmer_runtime_core::import::ImportObject::new();
    import_object.allow_missing_functions = true; // Import initialization might be left to the loader.
    let instance = module.instantiate(&import_object).unwrap();
    let mut ins = instance.load(wasmer_kernel_loader::KernelLoader).unwrap();

    loop {
        let cmd = stream.read_u32::<LittleEndian>().unwrap();
        match cmd {
            CMD_RUN_CODE => {
                let func_name_len = stream.read_u32::<LittleEndian>().unwrap();
                if func_name_len > 32 {
                    println!("function name too long");
                    return;
                }
                let mut func_name: Vec<u8> = Vec::with_capacity(func_name_len as usize);
                unsafe { func_name.set_len(func_name_len as usize) };
                stream.read_exact(&mut func_name).unwrap();
                let func_name = ::std::str::from_utf8(&func_name).unwrap();
                let arg_count = stream.read_u32::<LittleEndian>().unwrap();
                if arg_count > 0 {
                    println!("Too many arguments");
                    return;
                }
                use wasmer_runtime::Value;
                let mut args: Vec<Value> = Vec::with_capacity(arg_count as usize);
                for _ in 0..arg_count {
                    args.push(Value::I64(stream.read_u64::<LittleEndian>().unwrap() as _));
                }

                let index = instance.resolve_func(func_name).unwrap();
                let ret = ins.call(index, &args);
                match ret {
                    Ok(x) => {
                        stream.write_u32::<LittleEndian>(1).unwrap();
                        stream.write_u128::<LittleEndian>(x).unwrap();
                    }
                    Err(e) => {
                        println!("Execution error: {:?}", e);
                        stream.write_u32::<LittleEndian>(0).unwrap();
                    }
                }
            }
            CMD_READ_MEMORY => {
                let offset = stream.read_u32::<LittleEndian>().unwrap();
                let len = stream.read_u32::<LittleEndian>().unwrap();
                if len > 1048576 * 16 {
                    println!("memory size too large");
                    return;
                }
                let buf = ins.read_memory(offset, len).unwrap();
                stream.write_all(&buf).unwrap();
            }
            CMD_WRITE_MEMORY => {
                let offset = stream.read_u32::<LittleEndian>().unwrap();
                let len = stream.read_u32::<LittleEndian>().unwrap();
                if len > 1048576 * 16 {
                    println!("memory size too large");
                    return;
                }
                let mut buf: Vec<u8> = Vec::with_capacity(len as usize);
                unsafe { buf.set_len(len as usize) };
                stream.read_exact(&mut buf).unwrap();
                ins.write_memory(offset, len, &buf).unwrap();
            }
            _ => {
                println!("Unknown command");
                return;
            }
        }
    }
}

fn run_listen(opts: &Listen) {
    let listener = UnixListener::bind(&opts.socket).unwrap();
    use std::thread;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| {
                        handle_client(stream);
                    })) {
                        Ok(()) => {}
                        Err(_) => {}
                    }
                });
            }
            Err(err) => {
                panic!("{:?}", err);
            }
        }
    }
}

impl Kernel {
    pub fn execute(&self) {
        match &self {
            Kernel::Listen(listen) => {
                run_listen(listen);
            }
        }
    }
}
