section .text
global _start:
_start:
    push rbp
    mov rbp, rsp
    mov rdi, 65
    push rdi
    push rdi
    mov rdi, 1
    mov rax, rdi
    mov rdi, 66
    mov [rbp - 8 + rax * 4], rdi
    mov rdi, 2
    mov rax, rdi
    mov rdi, 67
    mov [rbp - 8 + rax * 4], rdi
    mov rdi, rbp
    sub rdi, 8
    push rdi
    mov rdi, 8
    pop rax
    add rax, rdi 
    mov rdi, rax
    mov rax, 1
    mov rsi, rdi
    mov rdi, 1
    mov rdx, 1
    syscall
    mov rdi, rbp
    sub rdi, 8
    mov rax, 60
    syscall
section .data
