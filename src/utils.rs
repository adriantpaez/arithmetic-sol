pub fn get_u64(instruction_data: &[u8], start_index: usize) -> u64 {
    u64::from_be_bytes(
        instruction_data[start_index..start_index + 8]
            .try_into()
            .expect("incorrect length"),
    )
}
