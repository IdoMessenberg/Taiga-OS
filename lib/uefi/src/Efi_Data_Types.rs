//--[normal_types]
pub type BOOLEAN = bool;
pub type INTN    = isize;
pub type UINTN   = usize;
pub type INT8    = i8;
pub type UINT8   = u8;
pub type INT16   = i16;
pub type UINT16  = u16;
pub type INT32   = i32;
pub type UINT32  = u32;
pub type INT64   = i64;
pub type UINT64  = u64;
pub type INT128  = i128;
pub type UINT128 = u128;
pub type CHAR8   = *const u8;
pub type CHAR16  = *const u16;


//--[uefi_types]
pub type VOID = core::ffi::c_void;
pub type EFI_HANDLE = *mut VOID;
pub type EFI_EVENT = *mut VOID;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct EFI_STATUS(UINTN);
impl EFI_STATUS{
    const WIDTH :UINTN = 8usize*core::mem::size_of::<EFI_STATUS>();
    const MASK  :UINTN = 0xc0 <<(EFI_STATUS::WIDTH - 8);
    const ERROR_MASK   :UINTN = 0x80 <<(EFI_STATUS::WIDTH -8);
    const WARNING_MASK :UINTN = 0x00 <<(EFI_STATUS::WIDTH -8);

    pub const SUCCESS :EFI_STATUS = EFI_STATUS::from_UINTN(0);

    const fn from_UINTN(value :UINTN)->EFI_STATUS{ EFI_STATUS(value)}
}