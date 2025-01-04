/// SquirrelNoise5 - Squirrel's Raw Noise utilities (version 5)
///
/// This implementation is inspired by the C++ SquirrelNoise5 function provided
/// by [Squirrel Eiserloh](http://eiserloh.net/noise/SquirrelNoise5.hpp).
///
/// It generates a pseudo-random 32-bit unsigned integer based on an input value and an optional seed.
///
/// # Example
///
/// ```
/// use squirrel_prng::squirrel_noise5;
/// let value = 123;
/// let seed = 456;
/// let noise = squirrel_noise5(value, seed);
/// println!("Noise value: {}", noise);
/// ```
#[inline]
#[must_use]
pub fn squirrel_noise5(value: u32, seed: u32) -> u32 {
    const SQ5_BIT_NOISE1: u32 = 0xD2A80A3f; // 11010010101010000000101000111111
    const SQ5_BIT_NOISE2: u32 = 0xA884F197; // 10101000100001001111000110010111
    const SQ5_BIT_NOISE3: u32 = 0x6C736F4B; // 01101100011100110110111101001011
    const SQ5_BIT_NOISE4: u32 = 0xB79F3ABB; // 10110111100111110011101010111011
    const SQ5_BIT_NOISE5: u32 = 0x1B56C4f5; // 00011011010101101100010011110101

    let mut mangled_bits = value;

    mangled_bits = mangled_bits.wrapping_mul(SQ5_BIT_NOISE1);
    mangled_bits = mangled_bits.wrapping_add(seed);
    mangled_bits ^= mangled_bits >> 9;
    mangled_bits = mangled_bits.wrapping_add(SQ5_BIT_NOISE2);
    mangled_bits ^= mangled_bits >> 11;
    mangled_bits = mangled_bits.wrapping_mul(SQ5_BIT_NOISE3);
    mangled_bits ^= mangled_bits >> 13;
    mangled_bits = mangled_bits.wrapping_add(SQ5_BIT_NOISE4);
    mangled_bits ^= mangled_bits >> 15;
    mangled_bits = mangled_bits.wrapping_mul(SQ5_BIT_NOISE5);
    mangled_bits ^= mangled_bits >> 17;

    mangled_bits
}
