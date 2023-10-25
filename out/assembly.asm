global _start:
_start:
    push rbp
    mov rbp, rsp
    mov rdi, 1
    push rdi
    mov rdi, [rbp - 8]
    push rdi
    mov rdi, 1
    pop rax
    cmp rax, rdi
    je LABEL2
LABEL3:
    mov rdi, 0
    jmp LABEL4
LABEL2:
    mov rdi, 1
LABEL4:
    cmp rdi, 0
    je LABEL1
    mov rdi, 1
    push rdi
    mov rdi, 2
    pop rax
    add rax, rdi 
    mov rdi, rax
    push rdi
    mov rdi, 3
    pop rax
    add rax, rdi 
    mov rdi, rax
    push rdi
    mov rdi, 4
    pop rax
    add rax, rdi 
    mov rdi, rax
    push rdi
    mov rdi, [rbp - 16]
    push rdi
    mov rdi, [rbp - 8]
    pop rax
    cmp rax, rdi
    jg LABEL6
LABEL7:
    mov rdi, 0
    jmp LABEL8
LABEL6:
    mov rdi, 1
LABEL8:
    cmp rdi, 0
    je LABEL5
    mov rdi, [rbp - 16]
    mov rax, 60
    syscall
LABEL5:
LABEL1:
    mov rdi, [rbp - 8]
    mov rax, 60
    syscall
