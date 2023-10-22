global _start:
_start:
    push rbp
    mov rbp, rsp
    mov rdi, 1
    push rdi
    mov rdi, 1
    push rdi
    pop rax
    pop rdi
    add rdi, rax
    push rdi
    mov rdi, [rbp - 8]
    push rdi
    mov rdi, [rbp - 8]
    push rdi
    pop rax
    pop rdi
    add rdi, rax
    push rdi
    mov rdi, [rbp - 16]
    push rdi
    mov rax, 60
    pop rdi
    syscall
