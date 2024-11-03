module WNS::Registry {
    use Sui::address::Self;

    struct DomainRegistry has key {
        domain_name: vector<u8>,
        owner: address,
        expiration: u64,
    }

    public fun register(
        wns: &mut WNS,
        domain_name: vector<u8>,
        owner: address,
        duration: u64
    ): bool {
        let expiration = Sui::current_epoch() + duration;
        let registry = DomainRegistry { domain_name, owner, expiration };
        table::add(wns.domains, domain_name, registry);
        true
    }

    public fun renew(
        wns: &mut WNS,
        domain_name: vector<u8>,
        owner: address
    ): bool {
        let domain = table::borrow_mut(wns.domains, domain_name);
        domain.expiration += 365 * 24 * 60 * 60;
        true
    }

    public fun resolve(wns: &WNS, domain_name: vector<u8>): address {
        let domain = table::borrow(wns.domains, domain_name);
        domain.owner
    }
}
