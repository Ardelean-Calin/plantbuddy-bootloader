MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  MBR                               : ORIGIN = 0x00000000, LENGTH = 4K
  SOFTDEVICE                        : ORIGIN = 0x00001000, LENGTH = 96K
  ACTIVE                            : ORIGIN = 0x00019000, LENGTH = 188K
  DFU                               : ORIGIN = 0x00048000, LENGTH = 192K /* Needs to be 1 page (4k) bigger than ACTIVE */
  FLASH                             : ORIGIN = 0x00078000, LENGTH = 24K
  BOOTLOADER_STATE                  : ORIGIN = 0x0007E000, LENGTH = 4K
  RAM                         (rwx) : ORIGIN = 0x200024b8, LENGTH = 32K
  uicr_bootloader_start_address (r) : ORIGIN = 0x10001014, LENGTH = 0x4
}

__bootloader_state_start = ORIGIN(BOOTLOADER_STATE);
__bootloader_state_end = ORIGIN(BOOTLOADER_STATE) + LENGTH(BOOTLOADER_STATE);

__bootloader_active_start = ORIGIN(ACTIVE);
__bootloader_active_end = ORIGIN(ACTIVE) + LENGTH(ACTIVE);

__bootloader_dfu_start = ORIGIN(DFU);
__bootloader_dfu_end = ORIGIN(DFU) + LENGTH(DFU);

__bootloader_start = ORIGIN(FLASH);

SECTIONS
{
  .uicr_bootloader_start_address :
  {
    LONG(__bootloader_start)
  } > uicr_bootloader_start_address
}
