section .text
global _start:
_start:
    push rbp
    mov rbp, rsp
    mov rdi, STRING1
    push rdi
    mov rdi, STRING2
    push rdi
    mov rdi, STRING3
    push rdi
    mov rdi, 10
    push rdi
    mov rdi, 0
    push rdi
    mov rdi, [rbp - 8]
    mov rsi, rdi
    mov rdi, 39
    mov rdx, rdi
    mov rax, 1
    mov rdi, 1
    syscall
    mov rdi, rbp
    sub rdi, 32
    mov rsi, rdi
    mov rdi, 1
    mov rdx, rdi
    mov rax, 1
    mov rdi, 1
    syscall
LABEL1:
    mov rdi, 1
    cmp rdi, 0
    je LABEL2
    mov rdi, [rbp - 16]
    mov rsi, rdi
    mov rdi, 14
    mov rdx, rdi
    mov rax, 1
    mov rdi, 1
    syscall
    mov rdi, rbp
    sub rdi, 40
    mov rsi, rdi
    mov rdi, 1
    mov rdx, rdi
    mov rax, 0
    mov rdi, 1
    syscall
    mov rdi, [rbp - 40]
    push rdi
    mov rdi, rbp
    sub rdi, 40
    mov rsi, rdi
    mov rdi, 1
    mov rdx, rdi
    mov rax, 0
    mov rdi, 1
    syscall
    mov rdi, [rbp - 24]
    mov rsi, rdi
    mov rdi, 15
    mov rdx, rdi
    mov rax, 1
    mov rdi, 1
    syscall
    mov rdi, rbp
    sub rdi, 40
    mov rsi, rdi
    mov rdi, 1
    mov rdx, rdi
    mov rax, 0
    mov rdi, 1
    syscall
    mov rdi, [rbp - 40]
    push rdi
    mov rdi, rbp
    sub rdi, 40
    mov rsi, rdi
    mov rdi, 1
    mov rdx, rdi
    mov rax, 0
    mov rdi, 1
    syscall
    mov rdi, [rbp - 48]
    mov rax, rdi
    mov rdi, 48
    sub rax, rdi
    mov rdi, rax
    mov [rbp - 48], rdi
    mov rdi, [rbp - 56]
    mov rax, rdi
    mov rdi, 48
    sub rax, rdi
    mov rdi, rax
    mov [rbp - 56], rdi
    mov rdi, [rbp - 48]
    mov rax, rdi
    mov rdi, 4
    cmp rax, rdi
    jb LABEL4
LABEL5:
    mov rdi, 0
    jmp LABEL6
LABEL4:
    mov rdi, 1
LABEL6:
    cmp rdi, 0
    je LABEL3
    mov rdi, [rbp - 56]
    mov rax, rdi
    mov rdi, 4
    cmp rax, rdi
    jb LABEL8
LABEL9:
    mov rdi, 0
    jmp LABEL10
LABEL8:
    mov rdi, 1
LABEL10:
    cmp rdi, 0
    je LABEL7
    mov rdi, [rbp - 48]
    mov rax, rdi
    mov rdi, [rbp - 56]
    add rax, rdi 
    mov rdi, rax
    mov rax, rdi
    mov rdi, 48
    add rax, rdi 
    mov rdi, rax
    push rdi
    mov rdi, rbp
    sub rdi, 64
    mov rsi, rdi
    mov rdi, 1
    mov rdx, rdi
    mov rax, 1
    mov rdi, 1
    syscall
    mov rdi, rbp
    sub rdi, 32
    mov rsi, rdi
    mov rdi, 1
    mov rdx, rdi
    mov rax, 1
    mov rdi, 1
    syscall
    mov rdi, [rbp - 48]
    mov rax, rdi
    mov rdi, [rbp - 56]
    add rax, rdi 
    mov rdi, rax
    mov rax, rdi
    mov rdi, 48
    add rax, rdi 
    mov rdi, rax
    mov rax, 60
    syscall
    pop rsi
LABEL7:
LABEL3:
    pop rsi
    pop rsi
    jmp LABEL1
LABEL2:
section .data
STRING1:
    db "Enter two numbers please      ", 10
STRING2:
    db "First number: ", 10
STRING3:
    db "Second number: ", 10
