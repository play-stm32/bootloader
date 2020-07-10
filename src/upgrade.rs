use sdio_sdhc::sdcard::Card;
use fat32::base::Volume;
use core::fmt::Write;
use crate::usb_ttl::USART1;
use crate::{flash, tim, button, OS_START_ADDRESS, UPGRADE_FLAG, SECOND, led};

pub fn check_and_upgrade() {
    // Card from sdio_sdhc
    match Card::init() {
        Ok(card) => {
            // Volume from fat32
            let cont = Volume::new(card);
            // into root dir
            let root = cont.root_dir();
            match root.load_file("firmware.bin") {
                Ok(file) => {
                    let upgrade = || {
                        led::green_dark();
                        led::red_light();

                        let mut addr = OS_START_ADDRESS;

                        writeln!(USART1, "upgrading").unwrap();
                        writeln!(USART1, "start to erase flash, it will take minutes").unwrap();
                        flash::erase(5, 11);
                        writeln!(USART1, "erase flash successfully").unwrap();

                        writeln!(USART1, "start to upgrade firmware").unwrap();
                        for (buf, len) in file.read_per_block() {
                            flash::write(addr, &buf[0..len]);
                            addr += len;
                        }

                        writeln!(USART1, "upgrade successfully").unwrap();
                        writeln!(USART1, "").unwrap();
                    };

                    if let Err(_) = root.exist("install") {
                        writeln!(USART1, "found firmware").unwrap();
                        writeln!(USART1, "if you do nothing, it will boot os in 5 seconds").unwrap();
                        writeln!(USART1, "if you want to upgrade, press the KEY0").unwrap();
                        writeln!(USART1, "").unwrap();

                        tim::enable_count();
                        button::enable_interrupt();

                        while unsafe { SECOND != 5 } {}
                        if unsafe { UPGRADE_FLAG } { upgrade(); }
                    } else {
                        writeln!(USART1, "found install file, auto upgrade").unwrap();
                        upgrade();
                        writeln!(USART1, "delete install file").unwrap();
                        writeln!(USART1, "").unwrap();
                        root.delete("install").unwrap();
                    }
                }
                Err(_) => {
                    writeln!(USART1, "No Found Firmware").unwrap();
                }
            }
        }
        Err(_) => {
            writeln!(USART1, "No Found Card").unwrap();
        }
    };
}