#!/bin/sh
llvm-objcopy -O binary --only-section=.text --only-section=.rodata --only-section=".data*" target/riscv32imac-unknown-none-elf/debug/test_app out/flash_qemu.bin
truncate -s 32M out/flash_qemu.bin
qemu-system-riscv32 -M virt,aclint=on -m 256k -bios none -cpu rv32,s=off,i=on,m=on,a=on,c=on,f=off,d=off,h=off,zba=off,zbb=off,zbc=off,zbs=off,Zicsr=on,Zifencei=off,pmu-num=0,pmp=on,mmu=off -drive file=out/flash_qemu.bin,format=raw,if=pflash,readonly=on -display none -serial mon:stdio -semihosting-config enable=on,target=native,userspace=on
