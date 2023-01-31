import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import ConstructorsSteakoin from "../typechain-generated/constructors/steakoin";
import ContractSteakoin from "../typechain-generated/contracts/steakoin";
import ConstructorsVoting from "../typechain-generated/constructors/voting";
import ContractVoting from "../typechain-generated/contracts/voting";
import { Keyring } from "@polkadot/keyring";

async function deploy() {
  const api = await ApiPromise.create();
  const keyring = new Keyring({ type: "sr25519" });

  const deployerSigner: KeyringPair = keyring.addFromUri("//Alice");

  const { address: steakoinAddress } = await new ConstructorsSteakoin(
    api,
    deployerSigner
  ).new();

  console.log("Steakoin deployed at: ", steakoinAddress);

  const steakoin = new ContractSteakoin(steakoinAddress, deployerSigner, api);

  const { address: votingAddress } = await new ConstructorsVoting(
    api,
    deployerSigner
  ).new(steakoinAddress);

  console.log("Voting deployed at: ", votingAddress);

  const voting = new ContractVoting(votingAddress, deployerSigner, api);
}

deploy()
  .then(() => process.exit(0))
  .catch(console.error);
