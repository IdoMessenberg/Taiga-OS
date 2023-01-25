#![no_std]
#![allow(warnings)]

mod Efi_Data_Types;
mod Efi_Protocols{
    pub mod Efi_System_Table;
    pub mod Efi_Console{
        pub mod EFI_SIMPLE_TEXT_OUTPUT;
    }
}

pub mod macros{
    pub use super::Efi_Data_Types::*;
}

pub mod protocols{
    pub use super::Efi_Data_Types::*;
    pub use super::Efi_Protocols::Efi_System_Table::*;
    pub use super::Efi_Protocols::Efi_Console::*;
}