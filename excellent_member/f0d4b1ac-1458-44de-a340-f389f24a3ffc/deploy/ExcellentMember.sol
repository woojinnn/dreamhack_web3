// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./Ownable.sol";

contract ExcellentMember is Ownable {
    bool public solved;

    mapping(address => bool) public join;
    mapping(address => bool) public excellentMember;

    address[] public excellentMembers;

    constructor() Ownable(msg.sender) {
        excellentMember[address(this)] = true;
        excellentMembers.push(address(this));    
    }

    function adminCall(address target, bytes memory data) external payable onlyOwner returns (bytes memory) {
        (bool success, bytes memory returnData) = target.call{value: msg.value}(data);
        require(success, "Fail Low-Level call");
        return returnData;
    }

    function setJoin(bool opinion) external {
        join[msg.sender] = opinion;
    }

    function registryExcellentMember(address member) external {
        require(!excellentMember[member], "Already excellent member");
        require(join[member], "Set join");
        require(excellentMember[msg.sender] == true, "Only excellent member");
        excellentMember[member] = true;
        excellentMembers.push(member);
    }

    function solve() external {
        if (excellentMember[tx.origin] == true) {
            solved = true;
        }
    }
}

