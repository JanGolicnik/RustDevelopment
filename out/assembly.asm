neki:
    push rbp
    mov rbp, rsp
    mov rdi, 1
    mov rax, rdi
    mov rdi, [rbp + 16 + rax * 4]
    pop rsi
    ret
section .text
global _start:
_start:
    push rbp
    mov rbp, rsp
    mov rdi, 0
    push rdi
    push rdi
    mov rdi, 1
    mov rax, rdi
    mov rdi, 10
    mov [rbp - 8 + rax * 4], rdi
    mov rdi, [rbp - 8]
    push rdi
    call neki
    mov rax, 60
    syscall
section .data
