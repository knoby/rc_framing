#![no_std]
pub mod crc {
    /// Calculate the CRC value for a given array of bytes
    /// The function uses CRC-8-CIIT
    pub fn calc_crc8(data: &[u8]) -> u8 {
        // The CRC Mask
        let mask = 0x07;

        let mut crc = 0x00;

        for byte in data {
            crc ^= byte;

            for _ in 0..8 {
                if (crc & 0x80) >= 1 {
                    crc = (crc << 1) ^ mask;
                } else {
                    crc <<= 1;
                }
            }
        }

        crc
    }

    /// Check the given data with the crc8 function. Returns Ok(()) if the crc of the data is 0, otherwise Err(())
    pub fn check_crc(data: &[u8]) -> Result<(), ()> {
        if calc_crc8(data) == 0 {
            Ok(())
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc() {
        // Testdata
        let data = [0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0xF4];

        // Calculated CRC Value
        let crc_value = crc::calc_crc8(&data[..9]);

        // Check the calculate Value
        assert_eq!(crc_value, 0xF4);

        // Check the CRC check function
        assert_eq!(Ok(()), crc::check_crc(&data[..=9]));
    }
}
