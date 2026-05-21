nasm -f bin boot64.asm -o boot64.bin

rustc \
  --target x86_64-unknown-none \
  -C relocation-model=static \
  -C panic=abort \
  -C opt-level=z \
  -C no-redzone=yes \
  -C link-arg=-Tkernel.ld \
  src/main.rs -o kernel.elf

objcopy -O binary kernel.elf kernel.bin

dd if=/dev/zero of=nos.img bs=512 count=2880
dd if=boot64.bin of=nos.img conv=notrunc bs=512 count=1
dd if=kernel.bin of=nos.img conv=notrunc bs=512 seek=1

qemu-system-x86_64 -drive format=raw,file=nos.img
