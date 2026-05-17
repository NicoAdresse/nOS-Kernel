; boot64.asm

BITS 16
ORG 0x7C00

; --- Constants & Memory Layout ---
%define KERNEL_LOAD_PHYS   0x00010000
%define KERNEL_SECTORS     9
%define PAGE_TABLE_BASE    0x00008000
%define STACK_16_BASE      0x7C00
%define STACK_32_BASE      0x001FF000
%define STACK_64_BASE      0x001FE000

start:
    cli                         ; Disable interrupts
    cld                         ; Clear direction flag (Crucial for lodsb/stosd!)

    ; Initialize 16-bit segment registers
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, STACK_16_BASE

    ; Force standard 80x25 Color Text Mode (Clears screen & resets garbage colors)
    mov ax, 0x0003
    int 0x10

    ; Boot message routines completely removed; proceed straight to setup
    call enable_a20
    call load_kernel
    call enter_protected_mode

; --- Helper Routines (16-bit) ---

enable_a20:
    in   al, 0x92
    or   al, 0000_0010b
    out  0x92, al
    ret

load_kernel:
    ; Set ES:BX target address (Segment:Offset notation)
    mov bx, KERNEL_LOAD_PHYS & 0xFFFF
    mov ax, KERNEL_LOAD_PHYS >> 4
    mov es, ax

    mov ah, 0x02                ; BIOS read sectors function
    mov al, KERNEL_SECTORS
    mov ch, 0x00                ; Cylinder 0
    mov dh, 0x00                ; Head 0
    mov cl, 0x02                ; Sector 2 (Sector 1 is this bootloader)
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
    mov bx, 0x000C              ; Light Red foreground text
    int 0x10
    jmp .print
.halt:
    cli
    hlt
    jmp .halt

; --- Global Descriptor Table (GDT) ---

align 8
gdt_start:
gdt_null:    dq 0x0000000000000000
gdt_code32:  dq 0x00CF9A000000FFFF
gdt_data32:  dq 0x00CF92000000FFFF
gdt_code64:  dq 0x00AF9A000000FFFF
gdt_end:

gdt_desc:
    dw gdt_end - gdt_start - 1
    dd gdt_start

CODE32_SEL  equ gdt_code32 - gdt_start
DATA32_SEL  equ gdt_data32 - gdt_start
CODE64_SEL  equ gdt_code64 - gdt_start

; --- Protected Mode Transition ---

enter_protected_mode:
    lgdt [gdt_desc]
    mov eax, cr0
    or  eax, 1
    mov cr0, eax
    jmp CODE32_SEL:pm_entry     ; Far jump to flush CPU pipeline and load CS

; --- 32-Bit Protected Mode ---

BITS 32
pm_entry:
    mov ax, DATA32_SEL
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov esp, STACK_32_BASE

    ; Zero-out 4 pages (16KB) for Paging Tables
    mov edi, PAGE_TABLE_BASE
    mov ecx, 4096
    xor eax, eax
    rep stosd                   ; Relies completely on 'cld' executed earlier!

    ; PML4[0] -> PDPT
    mov eax, PAGE_TABLE_BASE + 0x1000 | 0x003
    mov [PAGE_TABLE_BASE + 0*8], eax

    ; PDPT[0] -> PD
    mov eax, PAGE_TABLE_BASE + 0x2000 | 0x003
    mov [PAGE_TABLE_BASE + 0x1000 + 0*8], eax

    ; PD[0] -> PT
    mov eax, PAGE_TABLE_BASE + 0x3000 | 0x003
    mov [PAGE_TABLE_BASE + 0x2000 + 0*8], eax

    ; Identity map first 2MB via Page Table loop
    mov edi, PAGE_TABLE_BASE + 0x3000
    mov eax, 0x00000003         ; Present + Writable flags
    mov ecx, 512
.map_loop:
    mov [edi], eax
    add eax, 0x1000
    add edi, 8
    loop .map_loop

    ; Load CR3 with PML4 base address
    mov eax, PAGE_TABLE_BASE
    mov cr3, eax

    ; Enable PAE (Physical Address Extension)
    mov eax, cr4
    or  eax, 1 << 5
    mov cr4, eax

    ; Enable Long Mode inside EFER MSR
    mov ecx, 0xC0000080
    rdmsr
    or  eax, 1 << 8
    wrmsr

    ; Enable Paging
    mov eax, cr0
    or  eax, (1 << 31)
    mov cr0, eax

    jmp CODE64_SEL:lm_entry     ; Jump into 64-bit Long Mode

; --- 64-Bit Long Mode ---

BITS 64
lm_entry:
    ; Clear segment registers (long mode ignores data descriptors but expects safety)
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    
    mov rsp, STACK_64_BASE
    
    ; Jump straight into your loaded Rust flat binary
    mov rax, KERNEL_LOAD_PHYS
    jmp rax

; --- Bootloader Data & Metadata ---

msg_err  db "Disk error!", 13, 10, 0

times 510-($-$$) db 0
dw 0xAA55