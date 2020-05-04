use core::fmt::Write;
use embedded_sdmmc::Mode;
use crate::{flash, OS_START_ADDRESS, FIRMWARE_SIZE};
use crate::tf::{SPI1, CS, Time};
use crate::usart::USART2;

/// check and write firmware
pub fn check_and_write() {
    let mut cont = embedded_sdmmc::Controller::new(embedded_sdmmc::SdMmcSpi::new(SPI1, CS), Time);
    match cont.device().init() {
        Ok(_) => {
            match cont.get_volume(embedded_sdmmc::VolumeIdx(0)) {
                Ok(mut v) => {
                    match cont.open_root_dir(&v) {
                        Ok(dir) => {
                            match cont.find_directory_entry(&v, &dir, "firmware.bin") {
                                Ok(_) => {
                                    match cont.open_file_in_dir(&mut v, &dir, "firmware.bin", Mode::ReadOnly) {
                                        Ok(mut file) => {
                                            let mut buf: [u8; FIRMWARE_SIZE] = [0; FIRMWARE_SIZE];
                                            let len = cont.read(&v, &mut file, &mut buf).unwrap();
                                            let address = OS_START_ADDRESS;

                                            writeln!(USART2, "have a firmware, start to write").unwrap();
                                            flash::erase();
                                            flash::write(address, buf, len);
                                            writeln!(USART2, "write done").unwrap();
                                        }
                                        Err(_) => { writeln!(USART2, "open firmware error").unwrap(); }
                                    }
                                }
                                Err(_) => {
                                    writeln!(USART2, "no found firmware").unwrap();
                                }
                            }
                        }
                        Err(_) => { writeln!(USART2, "open root error").unwrap(); }
                    }
                }
                Err(_) => { writeln!(USART2, "no match volume").unwrap(); }
            }
        }
        Err(_) => { writeln!(USART2, "have no tf card").unwrap(); }
    }
}