#![no_std] #![no_main]
#![allow(warnings)]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]

use uefi::protocols::*;

#[no_mangle]
pub extern "C" fn efi_main(image :EFI_HANDLE, main_system_table :SYSTEM_TABLE) -> EFI_STATUS{
    let string_as_bytes :&[u8] = "hello world".as_bytes();
    let ConsoleOutput = unsafe{ &mut *(main_system_table.ConOut)};
    let mut buf = [0u16; 32];

    for i in 0..string_as_bytes.len()  
    {
        buf[i] = string_as_bytes[i] as u16;   
    }

    (ConsoleOutput.Reset)(ConsoleOutput,true);
    (ConsoleOutput.OutputString)(ConsoleOutput,buf.as_ptr());

    loop{}

    EFI_STATUS::SUCCESS
}


use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop {} }


#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    for test in tests {
        test();
    }
}