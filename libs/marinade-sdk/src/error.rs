use derive_more::Display;
use solana_program::program_error::ProgramError;

#[repr(u32)]
#[derive(Debug, Clone, Copy, Display)]
pub enum CommonError {
    WrongReserveOwner,
    NonEmptyReserveData,
    InvalidInitialReserveLamports,
    ZeroValidatorChunkSize,
    TooBigValidatorChunkSize,
    ZeroCreditChunkSize,
    TooBigCreditChunkSize,
    TooLowCreditFee,
    InvalidMintAuthority,
    MintHasInitialSupply,
    InvalidOwnerFeeState,
    InvalidProgramId = 6116,
    UnexpectedAccount = 65140,
    CalculationFailure = 51619,
    AccountWithLockup = 45694,
    NumberTooLow = 7892,
    NumberTooHigh = 7893,
    FeeTooHigh = 4052,
    FeesWrongWayRound = 4053,
    LiquidityTargetTooLow = 4054,
    TicketNotDue = 4055,
    TicketNotReady = 4056,
    WrongBeneficiary = 4057,
    StakeAccountNotUpdatedYet = 4058,
    StakeNotDelegated = 4059,
    StakeAccountIsEmergencyUnstaking = 4060,
    InsufficientLiquidity = 4205,
    InvalidValidator = 47525,
}

const ERROR_CODE_OFFSET: u32 = 300;

impl From<CommonError> for ProgramError {
    fn from(e: CommonError) -> Self {
        ProgramError::Custom(e as u32 + ERROR_CODE_OFFSET)
    }
}
