use neo4rs_macros::BoltStruct;

pub const MARKER: u8 = 0xB0;
pub const SIGNATURE: u8 = 0x0F;

#[derive(Debug, PartialEq, Eq, Clone, BoltStruct)]
pub struct Reset;

impl Reset {
    fn marker() -> (u8, Option<u8>) {
        (MARKER, Some(SIGNATURE))
    }
}

impl Reset {
    pub fn new() -> Reset {
        Reset
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::*;
    use std::convert::TryInto;

    #[test]
    fn should_serialize_reset() {
        let reset = Reset::new();

        let bytes: Bytes = reset.try_into().unwrap();

        assert_eq!(bytes, Bytes::from_static(&[MARKER, SIGNATURE,]));
    }
}
