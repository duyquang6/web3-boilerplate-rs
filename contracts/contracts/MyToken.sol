// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

// Import OpenZeppelin's ERC20 implementation for standard token functionality
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

/// @title MyToken
/// @notice A simple ERC20 token implementation with minting capability
/// @dev Inherits from OpenZeppelin's ERC20 implementation
contract MyToken is ERC20 {
    /// @notice Constructor initializes the token with name and symbol
    /// @dev Sets the token name to "MyToken" and symbol to "MYTK"
    constructor() ERC20("MyToken", "MYTK") {}

    /// @notice Mints new tokens to the message sender
    /// @dev Uses OpenZeppelin's internal _mint function
    /// @param amount The number of tokens to mint
    function mint(uint256 amount) public {
        _mint(msg.sender, amount);
    }
}
