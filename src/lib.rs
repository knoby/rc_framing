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

pub mod framing {

    /// Framing Constants
    pub const END: u8 = 0x00;
    const ESC: u8 = 0x33;
    const ESC_END: u8 = 0x34;
    const ESC_ESC: u8 = 0x35;

    /// Decote the given data using SLIP. Bevor encoding a crc value is appended to the data.the
    /// Returns the length of the data in the output buffer
    pub fn encode(data: &[u8], output: &mut [u8]) -> Result<usize, ()> {
        //Create a peekable iterator over the output buffer for safe access to the data
        let mut iter_out = output.iter_mut();

        let mut len = 0;

        // Add Start byte
        try_add_byte(END, iter_out.next())?;
        len += 1;

        // Iterate over the data and apply the slip encoding
        for byte in data.iter() {
            match *byte {
                END => {
                    try_add_byte(ESC, iter_out.next())?;
                    len += 1;
                    try_add_byte(ESC_END, iter_out.next())?;
                    len += 1;
                }
                ESC => {
                    try_add_byte(ESC, iter_out.next())?;
                    len += 1;
                    try_add_byte(ESC_ESC, iter_out.next())?;
                    len += 1;
                }
                _ => {
                    try_add_byte(*byte, iter_out.next())?;
                    len += 1;
                }
            };
        }

        //Try to add the CRC
        try_add_byte(super::crc::calc_crc8(data), iter_out.next())?;
        len += 1;

        // Add End byte
        try_add_byte(END, iter_out.next())?;
        len += 1;

        Ok(len)
    }

    /// Decode the recived Data. Expects the Data between two END byte of the message.
    /// Returns Err if the buffer is not big enough or the crc failed;
    pub fn decode(input: &[u8], msg: &mut [u8]) -> Result<usize, ()> {
        // Iterator over the Input
        let mut input_iter = input.iter().peekable();

        // length of the message
        let mut len = 0;

        {
            // Iterator for storing the msg
            let mut msg_iter = msg.iter_mut();

            // Iterate over the input
            while let Some(&byte) = input_iter.next() {
                match byte {
                    // Unescape Character
                    ESC => {
                        if let Some(&esc_char) = input_iter.next() {
                            match esc_char {
                                ESC_END => {
                                    try_add_byte(END, msg_iter.next())?;
                                    len += 1;
                                }
                                ESC_ESC => {
                                    try_add_byte(ESC, msg_iter.next())?;
                                    len += 1;
                                }
                                _ => return Err(()),
                            }
                        } else {
                            return Err(());
                        }
                    }
                    _ => {
                        try_add_byte(byte, msg_iter.next())?;
                        len += 1;
                    }
                }
            }
        }

        // Check the crc
        super::crc::check_crc(&msg[..len])?;

        if len >= 1 {
            // Remove the crc Value
            msg[len - 1] = 0;

            Ok(len - 1)
        } else {
            Err(())
        }
    }

    // Try to write to the iterator and return the result of the operation
    fn try_add_byte(byte: u8, target: Option<&mut u8>) -> Result<(), ()> {
        if let Some(dest) = target {
            *dest = byte;
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

    #[test]
    fn slip_test() {
        // Testdata
        let data_in = [0xde, 0xad, 0x00, 0xbe, 0xef];

        let mut buffer = [0_u8; 32];

        // Encode the data
        let enc_len = framing::encode(&data_in, &mut buffer).unwrap();

        // Check length of encoded frame
        assert_eq!(enc_len, 9);
        // Check encoded frame
        assert_eq!(
            buffer[..enc_len],
            [0x00, 0xde, 0xad, 0x33, 0x34, 0xbe, 0xef, 36, 0x00]
        );

        let mut data_out = [0_u8; 32];

        // Decode the data
        let dec_len = framing::decode(&buffer[1..enc_len - 1], &mut data_out).unwrap();

        // Check length of decoded frame
        assert_eq!(dec_len, 5);

        // Check decoded frame
        assert_eq!(data_in, data_out[..dec_len]);
    }
}
