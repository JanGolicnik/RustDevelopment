section .bss
    char_buffer resb 4
section .text
global _start:
_start:
    push rbp
    mov rbp, rsp
    mov rax, 1
    mov rdi, 1
    mov rsi, STRING1
    mov rdx, 11
    syscall
    mov rdi, 1
    mov rax, 60
    syscall
section .data
STRING1:
    db "Hello World", 10
