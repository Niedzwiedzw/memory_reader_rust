extern crate winapi;
// Theese are types that Windows API expects:
use winapi::um::winnt::{HANDLE};
use winapi::shared::minwindef::{DWORD,
                                BOOL,
                                LPCVOID,
                                LPVOID,
};
use winapi::shared::basetsd::{SIZE_T};

// This is a permission flag for this, documented here:
// https://msdn.microsoft.com/pl-pl/library/windows/desktop/ms684880(v=vs.85).aspx
const PERMISSION: u32 = 0x0010 | 0x0020 | 0x0008 | 0x0200;

const PID: i64 = 9480;  // Example PID
const VAR_ADDRES: i64 = 0x44C56FF8DC;  // memory adres, taken from Cheat Engine.

// this one gets PID of a process and returns handler for it,
// needed for reading and writing to address
unsafe fn get_process_handle(pid: i64) -> HANDLE {
    winapi::um::processthreadsapi::OpenProcess(
        PERMISSION,
        false as BOOL,
        pid as DWORD,
    )
}


unsafe fn read_address(process: HANDLE, address: i64) -> u8 {
    let mut buffer = [0 as u8; 64];  // our buffer is an array of 64 bytes

    let size = 64 as SIZE_T;  // size of our buffer
    let mut number_of_bytes_read = 0 as SIZE_T;  // used for large data, for iterating over region
    winapi::um::memoryapi::ReadProcessMemory(
        process,  // our process handler
        address as LPCVOID,  // we must cast our function to the type Windows expects.
        (&mut buffer).as_mut_ptr() as LPVOID,  // mutable pointer to our buffer as slice
        size,
        &mut number_of_bytes_read
    );

    return buffer[0];
}

fn main() {
    println!("permission: {:?}", PERMISSION);
    unsafe {
        let process = get_process_handle(PID);
        println!("process: {:?}", process);
        let value = read_address(process, VAR_ADDRES);
        println!("value: {:?}", value);
    }
}

