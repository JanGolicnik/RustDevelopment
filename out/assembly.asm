global _start:
_start:
    push rbp
    mov rbp, rsp
    mov rdi, 2
    push rdi
    mov rdi, 2
    pop rax
    cmp rax, rdi
    je LABEL1
LABEL2:
    mov rdi, 0
    jmp LABEL3
LABEL1:
    mov rdi, 1
LABEL3:
    push rdi
    mov rdi, [rbp - 8]
    mov rax, 60
    syscall
