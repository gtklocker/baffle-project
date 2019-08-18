extern crate baffle;
extern crate web3;

#[cfg(test)]
mod tests {
    use web3::contract::Options;
    use web3::futures::Future;
    use web3::types::*;

    #[test]
    fn it_works() {
        let (_eloop, web3) = baffle::make_web3_ganache();
        let first_account = web3.eth().accounts().wait().unwrap()[0];
        let simple_storage_artifact = baffle::get_artifact("SimpleStorage");
        let simple_storage = baffle::deploy_contract(&simple_storage_artifact, &web3, first_account);
        simple_storage.call("set", (42,), first_account, Options::default());

        let result: U256 = simple_storage.query("get", (), None, Options::default(), None).wait().unwrap();
        assert_eq!(result.as_u32(), 42);
    }
}
