; boot64.asm

BITS 16
ORG 0x7C00

%define KERNEL_LOAD_PHYS   0x00010000
%define KERNEL_SECTORS      9
%define PAGE_TABLE_BASE    0x00008000
%define STACK_16_BASE      0x7C00
%define STACK_32_BASE      0x001FF000
%define STACK_64_BASE      0x001FE000

start:
    cli
    cld

    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, STACK_16_BASE

    mov ax, 0x0003
    int 0x10

    call enable_a20
    call load_kernel
    call enter_protected_mode

enable_a20:
    in   al, 0x92
    or   al, 0000_0010b
    out  0x92, al
    ret

load_kernel:
    mov bx, KERNEL_LOAD_PHYS & 0xFFFF
    mov ax, KERNEL_LOAD_PHYS >> 4
    mov es, ax

    mov ah, 0x02
    mov al, KERNEL_SECTORS
    mov ch, 0x00
    mov dh, 0x00
    mov cl, 0x02
    int 0x13
    jc disk_error
    ret

disk_error:
    mov si, msg_err
.print:
    lodsb
    test al, al
    jz .halt
    mov ah, 0x0E
    mov bx, 0x000C
    int 0x10
    jmp .print
.halt:
    cli
    hlt
    jmp .halt

align 8
gdt_start:
gdt_null:   dq 0x0000000000000000
gdt_code32: dq 0x00CF9A000000FFFF
gdt_data32: dq 0x00CF92000000FFFF
gdt_code64: dq 0x00AF9A000000FFFF
gdt_end:

gdt_desc:
    dw gdt_end - gdt_start - 1
    dd gdt_start

CODE32_SEL  equ gdt_code32 - gdt_start
DATA32_SEL  equ gdt_data32 - gdt_start
CODE64_SEL  equ gdt_code64 - gdt_start

enter_protected_mode:
    lgdt [gdt_desc]
    mov eax, cr0
    or  eax, 1
    mov cr0, eax
    jmp CODE32_SEL:pm_entry

BITS 32
pm_entry:
    mov ax, DATA32_SEL
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov esp, STACK_32_BASE

    mov edi, PAGE_TABLE_BASE
    mov ecx, 4096
    xor eax, eax
    rep stosd

    mov eax, PAGE_TABLE_BASE + 0x1000 | 0x003
    mov [PAGE_TABLE_BASE + 0*8], eax

    mov eax, PAGE_TABLE_BASE + 0x2000 | 0x003
    mov [PAGE_TABLE_BASE + 0x1000 + 0*8], eax

    mov edi, PAGE_TABLE_BASE + 0x2000
    mov eax, 0x00000083
    mov ecx, 512
.map_loop:
    mov [edi], eax
    add eax, 0x00200000
    add edi, 8
    loop .map_loop

    mov eax, PAGE_TABLE_BASE
    mov cr3, eax

    mov eax, cr4
    or  eax, 1 << 5
    mov cr4, eax

    mov ecx, 0xC0000080
    rdmsr
    or  eax, 1 << 8
    wrmsr

    mov eax, cr0
    or  eax, (1 << 31)
    mov cr0, eax

    jmp CODE64_SEL:lm_entry

BITS 64
lm_entry:
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    mov rsp, STACK_64_BASE

    mov rax, KERNEL_LOAD_PHYS
    jmp rax

msg_err  db "Disk error!", 13, 10, 0

times 510-($-$$) db 0
dw 0xAA55
