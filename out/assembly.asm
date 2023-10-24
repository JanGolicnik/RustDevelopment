global _start:
_start:
    push rbp
    mov rbp, rsp
    mov rdi, 2
    push rdi
    mov rdi, 1
    push rdi
    mov rdi, 4
    push rdi
    mov rdi, [rbp - 24]
    push rdi
    mov rax, 60
    pop rdi
    syscall
