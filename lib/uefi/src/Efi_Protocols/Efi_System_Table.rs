use crate::protocols::*;

#[repr(C)]#[derive(Clone, Copy)]
pub struct SYSTEM_TABLE {
    pub Hdr                 :TABLE_HEADER,

    pub FirmwareVendor      :*mut CHAR16,
    pub FirmwareRevision    :UINT32,

    pub ConsoleInHandle     :EFI_HANDLE,
    pub ConIn               :*mut UINT32,//replace with console input protocol

    pub ConsoleOutHandle    :EFI_HANDLE,
    pub ConOut              :*mut EFI_SIMPLE_TEXT_OUTPUT::PROTOCOL,

    pub StandardErrorHandle :EFI_HANDLE,
    pub StdErr              :*mut EFI_SIMPLE_TEXT_OUTPUT::PROTOCOL,
}

#[repr(C)]#[derive(Clone, Copy)]
pub struct TABLE_HEADER{
    Signature  :UINT64,
    Revision   :UINT32,

    HeaderSize :UINT32,
    CRC32      :UINT32,
    Reserved   :UINT32,
}