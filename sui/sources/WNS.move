module WNS::WNS {
    use Sui::object::{Self, UID};
    use Registry::DomainRegistry;
    use Fees::DomainFees;
    use CrossChain::WormholeMessaging;
    use WNS::Registry;
    use WNS::SuiNSIntegration;
    use sui::clock::Clock;

    struct WNS has key {
        id: UID,
        admin: address,
    }

    public fun initialize(admin: address): WNS {
        WNS {
            id: UID::new(),
            admin,
        }
    }

    public fun register_domain(
        wns: &mut WNS,
        domain_name: vector<u8>,
        owner: address,
        duration: u64,
    ): bool {
        assert!(DomainFees::charge_registration_fee(wns, owner), 0);
        DomainRegistry::register(wns, domain_name, owner, duration)
    }

    public fun renew_domain(
        wns: &mut WNS,
        domain_name: vector<u8>,
        owner: address,
    ): bool {
        assert!(DomainFees::charge_renewal_fee(wns, owner), 0);
        DomainRegistry::renew(wns, domain_name, owner)
    }

    public fun resolve_domain(
        wns: &WNS,
        domain_name: vector<u8>
    ): address {
        DomainRegistry::resolve(wns, domain_name)
    }

     public fun assign_free_wns_if_sui_exists(
        wns: &mut WNS,
        suins: &SuiNS,
        domain_name: vector<u8>,
        clock: &Clock
    ) {
        SuiNSIntegration::assign_wns_if_sui_exists(wns, suins, domain_name, clock)
    }
}
