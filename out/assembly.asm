section .text
global _start:
_start:
    push rbp
    mov rbp, rsp
    mov rdi, 1
    mov rax, 60
    syscall
section .data
