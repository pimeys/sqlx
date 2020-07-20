use crate::{
    decode::Decode,
    encode::{Encode, IsNull},
    error::BoxDynError,
    mysql::{protocol::text::ColumnType, MySql, MySqlTypeInfo, MySqlValueRef},
    types::Type,
};
use bit_vec::BitVec;

impl Type<MySql> for BitVec {
    fn type_info() -> MySqlTypeInfo {
        MySqlTypeInfo::binary(ColumnType::Bit)
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        matches!(
            ty.r#type,
            ColumnType::VarChar
                | ColumnType::Blob
                | ColumnType::TinyBlob
                | ColumnType::MediumBlob
                | ColumnType::LongBlob
                | ColumnType::String
                | ColumnType::VarString
                | ColumnType::Bit
        )
    }
}

impl Encode<'_, MySql> for BitVec {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> IsNull {
        assert!(self.len() <= 64); // `bit(64)` is the maximum size
        <Vec<u8> as Encode<'_, MySql>>::encode(self.to_bytes(), buf)
    }

    fn produces(&self) -> Option<MySqlTypeInfo> {
        Some(MySqlTypeInfo::binary(ColumnType::Blob))
    }
}

impl<'r> Decode<'r, MySql> for BitVec {
    fn decode(value: MySqlValueRef<'r>) -> Result<Self, BoxDynError> {
        let bits = BitVec::from_bytes(value.as_bytes()?);
        Ok(bits)
    }
}
