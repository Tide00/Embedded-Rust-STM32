MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* TODO Adjust these memory regions to match your device memory layout */
  /* These values correspond to the b-l475e-iot01a, one of the few devices QEMU can emulate */
  FLASH : ORIGIN = 0x00000000, LENGTH = 1024K
  RAM : ORIGIN = 0x20000000, LENGTH = 96K
  RAM2 : ORIGIN = 0x10000000, LENGTH = 32K
}

