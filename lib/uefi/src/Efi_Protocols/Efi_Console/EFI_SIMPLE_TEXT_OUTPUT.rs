use crate::protocols::*;

#[repr(C)] #[derive(Clone, Copy)]
pub struct PROTOCOL{
    pub Reset        : extern "C" fn(this :&mut PROTOCOL, extended_verification :BOOLEAN) -> EFI_STATUS,
    pub OutputString : extern "C" fn(this :&mut PROTOCOL,string :CHAR16)                  -> EFI_STATUS,
}
pub struct MODE{
    MaxMod          :INT32,

    //--[Current_Setting]------
    Mode            :INT32,
    Attribute       :INT32,
    CursorColumn    :INT32,
    CursorRow       :INT32,
    CursorVisible   :INT32,
}