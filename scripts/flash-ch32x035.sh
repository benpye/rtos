#!/bin/sh
llvm-objcopy -O binary --only-section=.text --only-section=.rodata --only-section=".data*" target/riscv32imac-unknown-none-elf/debug/ch32x035_demo out/flash_ch32x035.bin
wch-openocd -f device/ch32x035/openocd.cfg -c init -c halt -c "flash erase_sector wch_riscv 0 last" -c "flash write_image out/flash_ch32x035.bin" -c wlink_reset_resume -c exit
