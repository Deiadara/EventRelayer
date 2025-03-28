// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.26;

contract Token {

    event Minted(address indexed sender, string amount);

    function mint(string memory amount) public {
        emit Minted(msg.sender, amount);
    }
}