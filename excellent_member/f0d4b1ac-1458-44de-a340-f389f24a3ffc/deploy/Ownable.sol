// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

abstract contract Ownable {
    address public owner;
    constructor(address initialOwner) {
        owner = initialOwner;
    }

    modifier onlyOwner() {
        _;
        require(owner == msg.sender, "Only owner");
    }

    function renounceOwnership() public virtual onlyOwner {
        owner = address(0);
    }

    function transferOwnership(address newOwner) public virtual onlyOwner {
        owner = newOwner;
    }
}

