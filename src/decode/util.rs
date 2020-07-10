use crate::decode::{
    error::INVALID_ALPHABET_OFFSET,
    model::{RangeSection, RangeSectionType, TCSDecodeError},
};

pub(crate) fn parse_from_bytes(val: &[u8], absolute_start_bit: usize, bit_length: usize) -> u64 {
    let first_byte_start_bit = (absolute_start_bit % 8) as u8;
    let relative_end_bit = bit_length - 1;
    let absolute_end_bit = absolute_start_bit + relative_end_bit;
    let last_byte_end_bit = (absolute_end_bit % 8) as u8;
    let last_byte_index = absolute_end_bit / 8;
    let remaining_bits_in_first_byte =
        (7i64 - (first_byte_start_bit as i64 + (relative_end_bit as i64))).max(0) as u8;
    let mut bit_mask = 0xff << first_byte_start_bit & 0xff;
    let mut current_byte = absolute_start_bit / 8;

    bit_mask >>= first_byte_start_bit + remaining_bits_in_first_byte;
    bit_mask <<= remaining_bits_in_first_byte;

    let mut return_value = val[current_byte] as u64 & (bit_mask) as u64;

    return_value >>= remaining_bits_in_first_byte as u64;

    if current_byte >= last_byte_index {
        return return_value;
    }

    current_byte += 1;

    while current_byte < last_byte_index {
        return_value <<= 8;
        return_value += val[current_byte] as u64;
        current_byte += 1;
    }

    return_value <<= (last_byte_end_bit + 1) as u64;
    let bit_shift = 7 - last_byte_end_bit as u64;
    return_value + ((val[current_byte] as u64 & 0xff << bit_shift) >> bit_shift)
}

pub(crate) fn parse_string_from_bytes(
    val: &[u8],
    bit_start: usize,
    bit_width: usize,
    char_count: usize,
) -> Result<String, TCSDecodeError> {
    byte_list_bit_boundary_check!(val, bit_start + (char_count * bit_width) - 1);

    let mut result = String::with_capacity(char_count);
    let mut offset = 0;

    for _ in 0..char_count {
        let alphabet_offset = parse_from_bytes(val, bit_start + offset, bit_width) as u8;

        if alphabet_offset > 25 {
            return Err(INVALID_ALPHABET_OFFSET);
        }

        result.push((b'A' + alphabet_offset) as char);
        offset += bit_width;
    }

    Ok(result)
}

pub(crate) fn parse_u8_bitfield_from_bytes(
    val: &[u8],
    bit_start: usize,
    bit_length: usize,
) -> Result<Vec<u8>, TCSDecodeError> {
    byte_list_bit_boundary_check!(val, bit_start + bit_length - 1);

    let mut result: Vec<u8> = Vec::with_capacity(bit_length);

    for index in 0..bit_length {
        if parse_from_bytes(val, bit_start + index, 1) as u8 == 1 {
            result.push(index as u8 + 1);
        }
    }

    Ok(result)
}

pub(crate) fn parse_u16_bitfield_from_bytes(
    val: &[u8],
    bit_start: usize,
    bit_length: usize,
) -> Result<Vec<u16>, TCSDecodeError> {
    let bit_end = bit_start + bit_length;

    byte_list_bit_boundary_check!(val, bit_end - 1);

    let mut result: Vec<u16> = Vec::with_capacity(bit_length);
    let mut index = bit_start;

    while index < bit_end {
        if parse_from_bytes(val, index, 1) as u8 == 1 {
            result.push((index - bit_start + 1) as u16);
        }

        index += 1;
    }

    Ok(result)
}

pub(crate) fn parse_vendor_range_from_bytes(
    val: &[u8],
    bit_start: usize,
) -> Result<RangeSection, TCSDecodeError> {
    let mut bit_index = bit_start + 12;

    byte_list_bit_boundary_check!(val, bit_index);

    let num_entries = parse_from_bytes(val, bit_start, 12) as usize;
    let mut entry_list: Vec<u16> = Vec::new();
    let mut count = 0;

    while count < num_entries {
        if parse_from_bytes(val, bit_index, 1) as u8 == 1 {
            byte_list_bit_boundary_check!(val, bit_index + 32);

            let start_vendor_id = parse_from_bytes(val, bit_index + 1, 16) as u16;
            let end_vendor_id = parse_from_bytes(val, bit_index + 17, 16) as u16;

            for vendor_id in start_vendor_id..end_vendor_id + 1 {
                entry_list.push(vendor_id);
            }

            bit_index += 32;
        } else {
            byte_list_bit_boundary_check!(val, bit_index + 16);

            entry_list.push(parse_from_bytes(val, bit_index + 1, 16) as u16);
            bit_index += 16;
        }

        count += 1;
    }

    Ok(RangeSection {
        last_bit: bit_index - 1,
        value: RangeSectionType::Vendor(entry_list),
    })
}