import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import ConstructorsSteakoin from "../typechain-generated/constructors/steakoin";
import ContractSteakoin from "../typechain-generated/contracts/steakoin";
import ConstructorsVoting from "../typechain-generated/constructors/voting";
import ContractVoting from "../typechain-generated/contracts/voting";
import { Keyring } from "@polkadot/keyring";

async function balance() {
  const api = await ApiPromise.create();
  const keyring = new Keyring({ type: "sr25519" });

  const deployerSigner: KeyringPair = keyring.addFromUri("//Alice");

  const steakoin = new ContractSteakoin("5DiqXZyev26QggMxqacWXWCfSrSi22BKJCWHEJupMuqNpNko", deployerSigner, api);
  const voting = new ContractVoting("5H8SY96wRbPBDN4BHKD8Dewzrp4bB9zQxp4MqemZCsYkM4by", deployerSigner, api);

  const balance = await steakoin.query.balanceOf(deployerSigner.address)
  console.log(balance.value.ok?.toString())

}

balance()
  .then(() => process.exit(0))
  .catch(console.error);
