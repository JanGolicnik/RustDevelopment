section .text
global _start:
_start:
    push rbp
    mov rbp, rsp
    mov rdi, 65
    push rdi
    mov rdi, rbp
    sub rdi, 8
    push rdi
    mov rdi, [rbp - 16]
mov rdi, [rdi]
    mov rax, 60
    syscall
section .data
