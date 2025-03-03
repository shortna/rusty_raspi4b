ENTRY(_start)

MEMORY
{
  RAM (rwx) : ORIGIN = 0x0, LENGTH = 2048M
}

SECTIONS
{
  .init 0x0 : AT(0x0) { .init } > RAM
  _STACK_START = ORIGIN(RAM) + LENGTH(RAM);
  _STACK_SIZE = 16M;
}
