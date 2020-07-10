# This a in-application programming (IAP)  bootloader

## Supported feature

* Upgrade firmware from sd card
* Use usb-ttl to show log

## Step

### Ready

* A fat32 card (only sdhc card)
* Rename your firmware to firmware.bin (the format is bin not elf)
* Copy your firmware to the card's root
* Insert card

### Power Up Board

* Check the bootloader works well (green LED light)
* Auto upgrade if card has `install` file in the root 
* OR press KEY0 to upgrade (green LED dark and red LED light)
* Wait

## Attentions

### Partition information

* Bootloader: 0x08000000 to 0x0801FFFF (128KB)
* Firmware: 0x08020000 to 0x080FFFFF (896KB; Sector from 5 to 11)

###  USB-TTL (CH340)

* Baudrare: 115200
* Data Bits: 8
* Parity: none
* Stop Bits: 1

### How to turn elf to bin

```arm-none-eabi-objcopy -O binary target/thumbv7m-none-eabi/debug/xxxx firmware.bin``` 

### Console log

``` 
This is a IAP bootloader
start to check for upgrade from sd card

found firmware
if you do nothing, it will boot os in 5 seconds
if you want to upgrade, press the KEY0

upgrading
start to erase flash, it will take minutes
erase flash successfully
start to upgrade firmware
upgrade successfully

boot os
```

## Download

* Download artifacts from [Actions](https://github.com/play-stm32/bootloader/actions)

## Relevant Project 

* [sdio_sdhc](https://github.com/play-stm32/sdio_sdhc)
* [fat32](https://github.com/play-stm32/fat32)

