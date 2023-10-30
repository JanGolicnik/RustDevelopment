neki:
    push rbp
    mov rbp, rsp
    mov rdi, 0
    push rdi
LABEL1:
    mov rdi, [rbp - 16]
    push rdi
    mov rdi, 10
    pop rax
    cmp rax, rdi
    jl LABEL3
LABEL4:
    mov rdi, 0
    jmp LABEL5
LABEL3:
    mov rdi, 1
LABEL5:
    cmp rdi, 0
    je LABEL2
    mov rdi, [rbp - 16]
    push rdi
    mov rdi, 1
    pop rax
    add rax, rdi 
    mov rdi, rax
    mov [rbp - 16], rdi
    jmp LABEL1
LABEL2:
    mov rdi, [rbp - 16]
    pop rsi
    pop rsi
    ret
neki:
    push rbp
    mov rbp, rsp
    mov rdi, 1
    pop rsi
    ret
section .text
global _start:
_start:
    push rbp
    mov rbp, rsp
    call neki
    mov rax, 60
    syscall
section .data
