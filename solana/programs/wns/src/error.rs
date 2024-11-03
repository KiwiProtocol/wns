use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("SNS domain not found.")]
    SnsDomainNotFound,
    #[msg("WNS domain already registered for this SNS domain.")]
    DomainAlreadyRegistered,
    #[msg("Name not found in the registry.")]
    NameNotFound,
    #[msg("Name registration has expired.")]
    NameExpired,
}
