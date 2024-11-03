module WNS::CrossChain {
    use Wormhole::core::Self;
    use Sui::address;

    public fun post_cross_chain_message(
        domain_name: vector<u8>,
        recipient_chain: u8,
        recipient_address: address
    ): bool {
        Wormhole::core::post_message(domain_name, recipient_chain, recipient_address);
        true
    }
}
