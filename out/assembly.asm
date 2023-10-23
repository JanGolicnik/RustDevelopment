global _start:
_start:
    push rbp
    mov rbp, rsp
    mov rdi, 4
    push rdi
    mov rdi, 5
    push rdi
    mov rdi, 1
    push rdi
    pop rdi
    pop rax
    add rdi, rax
    push rdi
    pop rdi
    pop rax
    mul rdi
    push rax
    mov rdi, [rbp - 8]
    push rdi
    mov rax, 60
    pop rdi
    syscall
