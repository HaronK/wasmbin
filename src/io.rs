use thiserror::Error;
pub use wasmbin_derive::Wasmbin;

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("{0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Leb128(#[from] leb128::read::Error),

    #[error("{0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("Could not recognise discriminant 0x{discriminant:X}")]
    UnsupportedDiscriminant { discriminant: i128 },

    #[error("Invalid module magic signature")]
    InvalidMagic,

    #[error("Unrecognized data")]
    UnrecognizedData,

    #[error("Section out of order: {current:?} after {prev:?}")]
    SectionOutOfOrder {
        current: crate::sections::Kind,
        prev: crate::sections::Kind,
    },
}

pub trait Encode {
    fn encode(&self, w: &mut impl std::io::Write) -> std::io::Result<()>;
}

pub trait Decode: Sized {
    fn decode(r: &mut impl std::io::Read) -> Result<Self, DecodeError>;
}

pub trait DecodeWithDiscriminant: Decode {
    type Discriminant: Decode + Copy + Into<i128>;

    fn maybe_decode_with_discriminant(
        discriminant: Self::Discriminant,
        r: &mut impl std::io::Read,
    ) -> Result<Option<Self>, DecodeError>;

    fn decode_with_discriminant(
        discriminant: Self::Discriminant,
        r: &mut impl std::io::Read,
    ) -> Result<Self, DecodeError> {
        Self::maybe_decode_with_discriminant(discriminant, r)?.ok_or_else(|| {
            DecodeError::UnsupportedDiscriminant {
                discriminant: discriminant.into(),
            }
        })
    }

    fn decode_without_discriminant(r: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        Self::decode_with_discriminant(Self::Discriminant::decode(r)?, r)
    }
}
