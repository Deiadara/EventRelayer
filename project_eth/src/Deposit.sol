// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.26;

contract Deposit {
    // Event declaration
    // Up to 3 parameters can be indexed.
    // Indexed parameters help you filter the logs by the indexed parameter
    event Deposited(address indexed sender, string amount);

    function deposit(string memory amount) public {
        emit Deposited(msg.sender, amount);
    }
}