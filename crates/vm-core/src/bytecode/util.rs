use super::BytecodeError;

pub fn read_i64(bytes: &[u8], cursor: &mut usize) -> Result<i64, BytecodeError> {
    let end = *cursor + 8;
    let slice = bytes.get(*cursor..end).ok_or(BytecodeError::Truncated)?;
    *cursor = end;
    let mut arr = [0u8; 8];
    arr.copy_from_slice(slice);
    Ok(i64::from_le_bytes(arr))
}

pub fn read_u64_as_usize(bytes: &[u8], cursor: &mut usize) -> Result<usize, BytecodeError> {
    let end = *cursor + 8;
    let slice = bytes.get(*cursor..end).ok_or(BytecodeError::Truncated)?;
    *cursor = end;
    let mut arr = [0u8; 8];
    arr.copy_from_slice(slice);
    Ok(u64::from_le_bytes(arr) as usize)
}

pub fn read_u16(bytes: &[u8], cursor: &mut usize) -> Result<u16, BytecodeError> {
    let end = *cursor + 2;
    let slice = bytes.get(*cursor..end).ok_or(BytecodeError::Truncated)?;
    *cursor = end;
    let mut arr = [0u8; 2];
    arr.copy_from_slice(slice);
    Ok(u16::from_le_bytes(arr))
}
