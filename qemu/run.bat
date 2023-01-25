@ECHO OFF
qemu-system-x86_64 -cpu qemu64 -m 128M -drive  if=pflash,format=raw,unit=0,file=Qemu/OVMF_CODE.fd,readonly=on -drive if=pflash,format=raw,unit=1,file=Qemu/OVMF_VARS.fd -drive format=raw,file=fat:rw:target\x86_64-unknown-uefi\debug
PAUSE