.section .init,"ax"
_start:
  mov x0, #0
  mov x1, #0
  mov x2, #0
  mov x3, #0
  mov x4, #0
  mov x5, #0
  mov x6, #0
  mov x7, #0
  mov x8, #0
  mov x9, #0
  mov x10, #0
  mov x11, #0
  mov x12, #0
  mov x13, #0
  mov x14, #0
  mov x15, #0
  mov x16, #0
  mov x17, #0
  mov x18, #0
  mov x19, #0
  mov x20, #0
  mov x21, #0
  mov x22, #0
  mov x23, #0
  mov x24, #0
  mov x25, #0
  mov x26, #0
  mov x27, #0
  mov x28, #0
  mov x29, #0
  mov x30, #0

  mrs x0, MPIDR_EL1;
  and x0, x0, #0x3
  cbz x0, cpu0
  b .

cpu0:
  ldr x0, =_STACK_START
  mov sp, x0
  bl main
  b .
