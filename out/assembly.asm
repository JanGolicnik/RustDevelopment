global _start:
_start:
    push rbp
    mov rbp, rsp
    mov rdi, 1
    push rdi
LABEL1:
    mov rdi, [rbp - 8]
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
    mov rdi, [rbp - 8]
    push rdi
    mov rdi, 5
    pop rax
    cmp rax, rdi
    jg LABEL7
LABEL8:
    mov rdi, 0
    jmp LABEL9
LABEL7:
    mov rdi, 1
LABEL9:
    cmp rdi, 0
    je LABEL6
    jmp LABEL2
LABEL6:
    mov rdi, [rbp - 8]
    push rdi
    mov rdi, 1
    pop rax
    add rax, rdi 
    mov rdi, rax
    mov [rbp - 8], rdi
    jmp LABEL1
LABEL2:
    mov rdi, [rbp - 8]
    mov rax, 60
    syscall
