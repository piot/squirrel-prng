use squirrel_prng::squirrel_noise5;

#[test]
fn known_values() {
    // (position_x, seed, expected_output)
    let test_cases: Vec<(i32, u32, u32)> = vec![
        (0, 0, 0x16791E00),
        (1, 0, 0xC895CB1D),
        (-1, 0, 0xFAF16D54),
        (123, 456, 0x0771723F),
        (-123, 456, 0x09B50E33),
        (2147483647, 4294967295, 0x1697A56A),
        (-2147483648, 0, 0x679CCD13),
        (42, 1337, 0x968DE4C9),
        (9999, 9999, 0x173A5069),
        (314159, 271828, 0xEFA3B8DC),
    ];

    for (position_x, seed, expected) in test_cases {
        let result = squirrel_noise5(position_x as u32, seed);
        assert_eq!(
            result, expected,
            "Failed for position_x: {position_x}, seed: {seed}. Expected: {expected:#010X}, Got: {result:#010X}",
        );
    }
}

#[test]
fn determinism() {
    let seed = 12345;
    let positions: Vec<i32> = vec![0, 1, -1, 123, -123, 42, 9999, 314159];

    for pos in positions {
        let noise1 = squirrel_noise5(pos as u32, seed);
        let noise2 = squirrel_noise5(pos as u32, seed);
        assert_eq!(
            noise1, noise2,
            "Non-deterministic for position_x: {} with seed: {}",
            pos, seed
        );
    }
}
