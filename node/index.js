// Goal: Make an API call to the faucet of the Shimmer testnet
// 1) install Axios via: npm i axios
// 2) run application via: node index.js

const axios = require('axios');
async function faucetRequest() {

    // Example Faucet for the shimmer network: const faucetEndpoint = 'https://faucet.testnet.shimmer.network/api/enqueue';
    // Example address: const address='rms1qpk8ecf45f6fvm92geaqgd2dt8a48lszn7x25se47jlnw67zhsgryp9w2zf';
    const faucetEndpoint = 'https://faucet.testnet.shimmer.network/api/enqueue';
    const address = 'your address';
    
    try {
        const response = await axios.post(faucetEndpoint, { address });
        console.log(response.data);
    } catch (error) {
        console.log(error.message.data);
    }
}

// Call the function and catch error if one occurs
faucetRequest().catch((err) => console.log(err));
