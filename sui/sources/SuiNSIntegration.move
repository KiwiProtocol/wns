module WNS::SuiNSIntegration {
    use std::string::{String, Self};
    use sui::clock::Clock;
    use SuiNS::suins::{SuiNS};
    use SuiNS::registry::{Registry};
    use SuiNS::domain::{Domain};
    use Registry::{Self as WnsRegistry};
    use WNS::WNS;

    const ENameNotFound: u64 = 0;
    const ENameExpired: u64 = 1;
    const ENameAlreadyExists: u64 = 2;

    /// Check if a `.sui` domain exists, and if so, automatically assign a corresponding `.wns` domain.
    public fun assign_wns_if_sui_exists(wns: &mut WNS, suins: &SuiNS, domain_name: String, clock: &Clock) {
        // Look up the name in SuiNS registry
        let optional_sui_record = suins.registry<Registry>().lookup(Domain::new(domain_name.clone()));
        // Ensure that the .sui name exists in SuiNS
        assert!(optional_sui_record.is_some(), ENameNotFound);

        let sui_record = optional_sui_record.extract();
        // Verify that the SuiNS domain has not expired
        assert!(!sui_record.has_expired(clock), ENameExpired);

        let wns_name = concatenate(domain_name, ".wns");

        // Check if a .wns domain already exists for this .sui name
        let existing_wns_record = WnsRegistry::lookup(wns, wns_name.clone());
        assert!(existing_wns_record.is_none(), ENameAlreadyExists);

        // Register the new .wns domain
        WnsRegistry::register(wns, wns_name, sui_record.owner);
    }

    fun concatenate(s1: String, s2: String): String {
        std::string::concat(s1, s2)
    }
}
