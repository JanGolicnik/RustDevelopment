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
    push rdi
    mov rdi, 0
    push rdi
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
    mov rdi, 2
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
    sub rdi, 56
    mov rsi, rdi
    mov rdi, 2
    mov rdx, rdi
    mov rax, 0
    mov rdi, 1
    syscall
    mov rdi, 0
    mov rax, rdi
    mov rdi, [rbp - 40 + rax * 4]
    push rdi
    mov rdi, 0
    mov rax, rdi
    mov rdi, [rbp - 56 + rax * 4]
    push rdi
    mov rdi, [rbp - 72]
    mov rax, rdi
    mov rdi, 48
    sub rax, rdi
    mov rdi, rax
    mov [rbp - 72], rdi
    mov rdi, [rbp - 80]
    mov rax, rdi
    mov rdi, 48
    sub rax, rdi
    mov rdi, rax
    mov [rbp - 80], rdi
    mov rdi, 0
    push rdi
    mov rdi, 0
    push rdi
    mov rdi, [rbp - 72]
    mov rax, rdi
    mov rdi, 4
    cmp rax, rdi
    ja LABEL2
LABEL3:
    mov rdi, 0
    jmp LABEL4
LABEL2:
    mov rdi, 1
LABEL4:
    cmp rdi, 0
    je LABEL1
    mov rdi, 1
    mov [rbp - 88], rdi
LABEL1:
    mov rdi, [rbp - 80]
    mov rax, rdi
    mov rdi, 4
    cmp rax, rdi
    ja LABEL6
LABEL7:
    mov rdi, 0
    jmp LABEL8
LABEL6:
    mov rdi, 1
LABEL8:
    cmp rdi, 0
    je LABEL5
    mov rdi, 1
    mov [rbp - 96], rdi
LABEL5:
    mov rdi, [rbp - 88]
    mov rax, rdi
    mov rdi, [rbp - 96]
    add rax, rdi 
    mov rdi, rax
    mov rax, rdi
    mov rdi, 0
    cmp rax, rdi
    je LABEL10
LABEL11:
    mov rdi, 0
    jmp LABEL12
LABEL10:
    mov rdi, 1
LABEL12:
    cmp rdi, 0
    je LABEL9
    mov rdi, 1
    mov rax, 60
    syscall
LABEL9:
    mov rdi, [rbp - 72]
    mov rax, rdi
    mov rdi, [rbp - 80]
    add rax, rdi 
    mov rdi, rax
    mov rax, 60
    syscall
section .data
STRING1:
    db "Tell me two numbers between 0-4 please      ", 10
STRING2:
    db "First number: ", 10
STRING3:
    db "Second number: ", 10
