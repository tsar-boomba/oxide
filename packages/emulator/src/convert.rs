#[inline(always)]
pub fn argb8888_to_xrgb8888(pixels: &[u8], result: &mut [u8]) {
    result.copy_from_slice(pixels);
}

#[inline(always)]
pub fn rgb565_to_xrgb8888(pixels: &[u8], result: &mut [u8]) {
    const BYTES_PER_PIXEL: usize = 2;

    assert_eq!(
        pixels.len() % BYTES_PER_PIXEL,
        0,
        "`pixels` length must be a multiple of 2 (16-bits per pixel)"
    );

    let num_pixels = pixels.len() / BYTES_PER_PIXEL;

    for i in 0..num_pixels {
        // This Rust code is decoding a 16-bit color value, represented by two bytes of data, into its corresponding red, green, and blue components.
        let pixel_offset = BYTES_PER_PIXEL * i;
        // these are backwards because of endian stuff
        let first_byte = pixels[pixel_offset + 1];
        let second_byte = pixels[pixel_offset];

        // First extract the red component from the first byte. The first byte contains the most significant 8 bits of the 16-bit color value. The & operator performs a bitwise AND operation on first_byte and 0b1111_1000, which extracts the 5 most significant bits of the byte. The >> operator then shifts the extracted bits to the right by 3 positions, effectively dividing by 8, to get the value of the red component on a scale of 0-31.
        let red5 = (first_byte & 0b1111_1000) >> 3;
        // Next extract the green component from both bytes. The first part of the expression ((first_byte & 0b0000_0111) << 3) extracts the 3 least significant bits of first_byte and shifts them to the left by 3 positions, effectively multiplying by 8. The second part of the expression ((second_byte & 0b1110_0000) >> 5) extracts the 3 most significant bits of second_byte and shifts them to the right by 5 positions, effectively dividing by 32. The two parts are then added together to get the value of the green component on a scale of 0-63.
        let green6 = ((first_byte & 0b0000_0111) << 3) + ((second_byte & 0b1110_0000) >> 5);
        // Next extract the blue component from the second byte. The & operator performs a bitwise AND operation on second_byte and 0b0001_1111, which extracts the 5 least significant bits of the byte. This gives the value of the blue component on a scale of 0-31.
        let blue5 = second_byte & 0b0001_1111;

        // Taken from https://stackoverflow.com/questions/2442576/how-does-one-convert-16-bit-rgb565-to-24-bit-rgb888
        let red8 = (red5 << 3) | (red5 >> 2);
        let green8 = (green6 << 2) | (green6 >> 3);
        let blue8 = (blue5 << 3) | (blue5 >> 2);

        // Finally save the pixel data in the result array as an RGBA32 value
        let output_offset = 4 * i;
        result[output_offset] = blue8;
        result[output_offset + 1] = green8;
        result[output_offset + 2] = red8;
        result[output_offset + 3] = 255;
    }
}

/// Used for storing frames as png
pub fn xrgb8888_to_rgba888(xrgb: &[u32]) -> Vec<u8> {
    let mut result = vec![0u8; xrgb.len() * 4];
    for i in 0..xrgb.len() {
        let pixel = xrgb[i];

        let [_, r, g, b] = pixel.to_be_bytes();
        let pixel_offset = i * 4;

        result[pixel_offset] = r;
        result[pixel_offset + 1] = g;
        result[pixel_offset + 2] = b;
        result[pixel_offset + 3] = 255;
    }

    result
}
