import { buildModule } from "@nomicfoundation/ignition-core";
import { ethers } from "ethers";

export default buildModule("MyTokenModule", (m) => {
    const myToken = m.contract("MyToken", [/* constructor arguments here */]);

    return { myToken };
});
