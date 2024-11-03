module WNS::Fees {
    use Sui::coin::{Self, transfer};

    const REGISTRATION_FEE: u64 = 200_000_000; // 0.2 SUI
    const RENEWAL_FEE: u64 = 200_000_000;      // 0.2 SUI

    public fun charge_registration_fee(owner: address): bool {
        transfer::transfer_sui(owner, REGISTRATION_FEE)
    }

    public fun charge_renewal_fee(owner: address): bool {
        transfer::transfer_sui(owner, RENEWAL_FEE)
    }
}
