global _start:
_start:
    push rbp
    mov rbp, rsp
    mov rdi, 1
    push rdi
    pop rdi
    cmp rdi, 0
    je LABEL1
    mov rdi, 1
    push rdi
    mov rax, 60
    pop rdi
    syscall
LABEL1:
    mov rdi, 3
    push rdi
    mov rax, 60
    pop rdi
    syscall
