## TangleCon 2023: IOTA for beginners

## Coding the faucet:
Node.js playground: https://stackblitz.com/edit/node-99aw1w

    const axios = require('axios');
    async function faucetRequest() {
      const faucetEndpoint = 'https://faucet.testnet.shimmer.network/api/enqueue';
      const address = 'your address';

      try {
          const response = await axios.post(faucetEndpoint, { address });
          console.log(response.data);
      } catch (error) {
          console.log(error.message.data);
      }
    }
    faucetRequest().catch((err) => console.log(err));


## Links:
        
Explorer

        Testnet Explorer: https://explorer.iota.org/testnet 
        Mainnet Explorer: https://explorer.iota.org/mainnet
        IOTA 2.0 Explorer: https://v2.iota.org/visualizer 
        
IOTA Testnet APIs:

        General Infos: https://wiki.iota.org/develop/endpoints/devnet/
 
        Node API: https://api.lb-0.h.chrysalis-devnet.iota.cafe
        Faucet App: https://faucet.chrysalis-devnet.iota.cafe/

Shimmer Testnet APIs:
      
        General Infos: https://wiki.iota.org/shimmer/develop/endpoints/testnet/
      
        Node API: https://api.testnet.shimmer.network
        Faucet App:  https://faucet.testnet.shimmer.network
        Faucet API: https://faucet.testnet.shimmer.network/api/enqueue
        Chronicle API: https://chronicle.testnet.shimmer.network
        
IOTA Wiki:

        https://wiki.iota.org/
        https://wiki.iota.org/shimmer

IOTA Libraries: 
   
        IOTA Client: https://github.com/iotaledger/iota.rs
        IOTA Wallet: https://github.com/iotaledger/wallet.rs
        IOTA Identity: https://github.com/iotaledger/identity.rs
        IOTA Stronghold: https://github.com/iotaledger/stronghold.rs