require("@nomicfoundation/hardhat-toolbox");
require("@nomicfoundation/hardhat-verify");
/** @type import('hardhat/config').HardhatUserConfig */

module.exports = {
  solidity: "0.8.0", //replace your own solidity compiler version
  networks: {
    opbnb: {
      url: "https://mainnet.infura.io/v3/<API-KEY>",
      chainId: 1, // Replace with the correct chainId for the "ethereum" network
      accounts: ["{{YOUR-PRIVATE-KEY}}"], // Add private keys or mnemonics of accounts to use
      gasPrice: 20000000000,
    },
  },
  etherscan: {
    apiKey: {
      etherscan: "{{ETHERSCAN-KEY}}", //replace with your etherscan key
    },
  },
};