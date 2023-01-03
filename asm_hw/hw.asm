# x86 Assembly Hello World

.global _start
.intel_syntax
.section .text

_start:

    mov %eax, 0x04          # write syscal
    mov %ebx, 0x01          # stdout address
    lea %ecx, [message]     # load effective address 
    mov %edx, 13            # size of message
    int 0x80

    mov %eax, 0x01          # exit syscall
    mov %ebx, 0x0           # error code 
    int 0x80                # invoke system call

# write Hello World to output

.section data
    message:
        .ascii "Hello, World\n"