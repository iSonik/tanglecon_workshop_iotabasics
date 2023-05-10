# TangleCon 2023: IOTA for beginners
## Downloading and installing the Firefly wallet for shimmer:

[Firefly Wallet](https://firefly.iota.org/) - Only download the wallet from the official website!

## Coding the faucet:
[Node.js playground](https://stackblitz.com/edit/node-99aw1w)
```javascripT
const axios = require('axios');
async function faucetRequest() {

    const faucetEndpoint = 'https://faucet.testnet.shimmer.network/api/enqueue';
    const address = 'your address';

    try {
      const response = await axios.post(faucetEndpoint, { address });
      console.log(response.data);
    } catch (error) {
      console.log(error.message);
    }
    
}
faucetRequest().catch((err) => console.log(err));
```
1) Click on the console and install axios: npm i axios
2) To start the application, just run the command node index.js



## Links:
## Image upload
[Pasteboard](https://pasteboard.co/)

### Explorer

[Testnet Explorer](https://explorer.iota.org/testnet)

[Mainnet Explorer](https://explorer.iota.org/mainnet)

[IOTA 2.0 Visualizer](https://v2.iota.org/visualizer)

        
### IOTA Testnet APIs:

[General infos](https://wiki.iota.org/develop/endpoints/devnet/)

[Node API](https://api.lb-0.h.chrysalis-devnet.iota.cafe)

[Faucet app](https://faucet.chrysalis-devnet.iota.cafe/)


### Shimmer Testnet APIs:
[General infos](https://wiki.iota.org/shimmer/develop/endpoints/testnet/)

[Node API](https://api.testnet.shimmer.network)

[Faucet app](https://faucet.testnet.shimmer.network)

[Faucet API](https://faucet.testnet.shimmer.network/api/enqueue)

[Chronicle API](https://chronicle.testnet.shimmer.network)

       
### IOTA Wiki:
[IOTA Wiki](https://wiki.iota.org/)

[Shimmer Wiki](https://wiki.iota.org/shimmer)


### IOTA Libraries: 
[IOTA Client](https://github.com/iotaledger/iota.rs)

[IOTA Wallet](https://github.com/iotaledger/wallet.rs)

[IOTA Identity](https://github.com/iotaledger/identity.rs)

[IOTA Stronghold](https://github.com/iotaledger/stronghold.rs)
