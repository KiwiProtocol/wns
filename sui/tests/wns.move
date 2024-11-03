module WNS::tests::WnsTests {
    use WNS::WNS;
    use Registry::DomainRegistry;
    use WNS::Registry;
    use SuiNS::suins::{SuiNS};
    use sui::clock::Clock;

    public fun test_register_domain() {
        let wns = WNS::initialize(0x1);
        assert!(WNS::register_domain(&wns, b"alice.wns", 0x1, 365));
        assert!(WNS::resolve_domain(&wns, b"alice.wns") == 0x1);
    }

    public fun test_renew_domain() {
        let wns = WNS::initialize(0x1);
        WNS::register_domain(&wns, b"alice.wns", 0x1, 365);
        WNS::renew_domain(&wns, b"alice.wns", 0x1);
        let expiration = DomainRegistry::resolve_expiration(&wns, b"alice.wns");
        assert!(expiration > Sui::current_epoch());
    }

      public fun test_assign_wns_if_sui_exists() {
        let wns = WNS::initialize(0x1);
        let suins = SuiNS::initialize();
        let clock = Clock::new();

        // Register a `.sui` domain in SuiNS (mock data)
        let domain_name = b"alice.sui";
        SuiNS::register_domain(suins, domain_name, 0x1, clock);

        // Assign a corresponding `.wns` domain
        WNS::assign_free_wns_if_sui_exists(&mut wns, &suins, domain_name, &clock);

        // Verify the `.wns` domain is registered
        let owner = Registry::resolve(&wns, b"alice.wns");
        assert!(owner == 0x1);
    }
}
