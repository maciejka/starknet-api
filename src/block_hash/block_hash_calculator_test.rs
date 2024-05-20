use starknet_types_core::felt::Felt;
use super::concat_counts;
use crate::data_availability::L1DataAvailabilityMode;

#[test]
fn concat_counts_test() {
    let concated = concat_counts(4, 3, 2, L1DataAvailabilityMode::Blob);
    let expected_felt =
        Felt::from_hex_unchecked("0x0000000000000004000000000000000300000000000000028000000000000000");
    assert_eq!(concated, expected_felt)
}
