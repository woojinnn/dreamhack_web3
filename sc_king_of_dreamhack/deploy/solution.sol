// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Solution {
    constructor(address level_addr) payable {
        (bool ret, bytes memory data) = level_addr.call{value: 1 gwei}("");
        require(ret, "This should be true");
    }
}
