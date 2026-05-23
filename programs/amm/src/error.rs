use anchor_lang::error_code;
use constant_product_curve::CurveError;

#[error_code]
pub enum AmmError {
    #[msg("Invalid precision")]
    InvalidPrecision,
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("Overflow occurred")]
    Overflow,
    #[msg("Invalid fee amount")]
    InvalidFeeAmount,
    #[msg("Slippage limit exceeded")]
    SlippageLimitExceeded,
    #[msg("Underflow occurred")]
    Underflow,
    #[msg("Zero balance")]
    ZeroBalance,
}

impl From<CurveError> for AmmError {
    fn from(error: CurveError) -> AmmError {
        match error {
            CurveError::InvalidPrecision => AmmError::InvalidPrecision,
            CurveError::InsufficientBalance => AmmError::InsufficientBalance, // You can map this to a different error if needed
            CurveError::Overflow => AmmError::Overflow, // You can map this to a different error if needed
            CurveError::InvalidFeeAmount => AmmError::InvalidFeeAmount, // You can map this to a different error if needed
            CurveError::SlippageLimitExceeded => AmmError::SlippageLimitExceeded, // You can map this to a different error if needed
            CurveError::Underflow => AmmError::Underflow, // You can map this to a different error if needed
            CurveError::ZeroBalance => AmmError::ZeroBalance, // You can map this to a different error if needed
        }
    }
}
