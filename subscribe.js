const subscribeAliceBalance = async (api: ApiPromise) => {
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');
    await api.query.system.account(alice.address, aliceAcct => {
        console.log("Subscribed to Alice account.");
        const aliceFreeSub = aliceAcct.data.free;
        console.log('Alice Account (sub): ${aliceFreeSub}');
    });
};
