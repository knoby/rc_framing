#![no_std]

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc() {
        let data = [0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39];

        let crc_value = calc_crc8(&data);

        assert_eq!(crc_value, 0xF4);
    }
}
