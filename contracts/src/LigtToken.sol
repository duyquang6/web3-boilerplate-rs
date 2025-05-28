// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract LigtToken is ERC20 {
    constructor() ERC20("LigtToken", "LIGT") {}

    function mint(uint256 amount) public {
        _mint(msg.sender, amount);
    }
}
