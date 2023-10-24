global _start:
_start:
    push rbp
    mov rbp, rsp
    mov rdi, 1
    push rdi
    mov rdi, [rbp - 8]
    push rdi
    pop rdi
    cmp rdi, 0
    je LABEL1
    mov rdi, [rbp - 8]
    push rdi
    mov rdi, 123
    push rdi
    pop rdi
    pop rax
    add rdi, rax
    push rdi
    mov rax, 60
    pop rdi
    syscall
LABEL1:
    mov rdi, [rbp - 8]
    push rdi
    mov rax, 60
    pop rdi
    syscall
