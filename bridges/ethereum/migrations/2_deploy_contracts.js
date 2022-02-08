const BitgreenBridge = artifacts.require("BitgreenBridge");

module.exports = function(deployer) {
  deployer.deploy(BitgreenBridge);
};
