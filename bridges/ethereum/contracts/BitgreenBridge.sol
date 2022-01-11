// SPDX-License-Identifier: MIT
// contract to manage the Inherit Actions
pragma solidity ^0.8.11;
contract BitgreenBridge {
    // settings storage
    address [20] keepers;
    address [5] watchdogs;
    address [5] watchcats;
    uint8 threshold;
    // address of the owner of the contract (the one allowed to change the settings)
    address payable public owner;
    // lockdown 
    bool lockdown;
    // transaction queue structure
    struct transactionqueue {
        address payable recipient;      // recipient of the transaction
        uint amount;                    // amount of the transaction
        uint8 cnt;                      // counter of votes received from keepers
        address erc20;                  // address of the ERC20 contract (optional)
    }
    mapping( bytes32 => transactionqueue ) public txqueue;


    // set the owner to the creator of the contract, ownership can be changed calling transferOwnership()
    constructor() payable {
          owner = payable(msg.sender);
          lockdown=false;
    }
     
    /**
     * @dev store configuration  for Keepers
     * @param Keepers is an array of address of the allowed keepers of the bridge transactions
     */
    function configurationKeepers(address [] memory Keepers) public {
        require(msg.sender == owner,"Function accessible only to owner");
        uint i=0;
        // store state
        for(i=0;i<20;i++){
            keepers[i]=Keepers[i];
        }
    }
    /**
     * @dev store configuration for Watchdogs
     * @param Watchdogs is an array of address of the accounts allowed to lockdown the bridge when a transaction arrives
     */
    function configurationWatchdogs(address [] memory Watchdogs) public {
        require(msg.sender == owner,"Function accessible only to owner");
        uint i=0;
        // store state
        for(i=0;i<5;i++){
            watchdogs[i]=Watchdogs[i];
        }
    }
    /**
     * @dev store configuration for Watchcats
     * @param Watchcats is an array of address of the accounts allowed to lockdown the bridge when a transaction is in the pool mem
     */
    function configurationWatchcats(address [] memory Watchcats) public {
        require(msg.sender == owner,"Function accessible only to owner");
        uint i=0;
        // store state
        for(i=0;i<5;i++){
            watchcats[i]=Watchcats[i];
        }
    }
    /**
     * @dev store configuration of the minimum threshold to reach a consensus on the transaction
     * @param Threshold is the minimum number of "votes" from Keepers to execute a transaction
     */
    function configurationThreshold(uint8 Threshold) public {
        require(msg.sender == owner,"Function accessible only to owner");
        threshold=Threshold;
    }
    /**
     * @dev transfer ownership
     * @param newOwner is the address wished as new owner
     */
    function transferOwnership(address payable newOwner) public {
        require(msg.sender == owner);
        owner = newOwner;
    }
    // functiont to receive deposit of native token
    function deposit() public payable {}

    //function to send back the balance
    function getBalance() public view returns (uint) {
        return address(this).balance;
    }
    /**
     * @dev transfer native tokens to a recipient
      * @param txid is the transaction id, it should be unique
     * @param recipient is a payable address
     * @param amount is a payable address
     * @param erc20 is the address of the erc20 contract (optional)
     */
    function transfer(bytes32 txid,address payable recipient, uint amount,address payable erc20) public {
        bool execute=false;
        uint8 i;
        // check for keepers
        for(i=0;i<20;i++) {
            if(keepers[i]==msg.sender){
                execute=true;
                break;
            }
        }
        require(execute==true,"Only Keepers account can access this function");
        // check for matching data of the transaction
        require(txid.length>0,"tx id is required");
        require(txqueue[txid].recipient==address(0) || txqueue[txid].recipient==recipient,"Recipient is wrong");
        require(recipient!=address(0),"Recipient cannot be empty");
        require(amount>0,"Amount cannot be zero");
        // update the queue
        if(txqueue[txid].recipient==address(0)){
            txqueue[txid].recipient=recipient;
            txqueue[txid].amount=amount;
            txqueue[txid].erc20=erc20;
        }
        txqueue[txid].cnt++;
        // make the transaction
        if(txqueue[txid].cnt==threshold) {
            // native token
            if(erc20==address(0)){
                (bool success, ) =recipient.call{value: amount}("");
                require(success, "Failed to send native tokens");
            }else {
                  // erc20 token
                  IERC20(erc20).transferFrom(owner, recipient, amount);
            }
        }
    }
    /**
     * @dev set lockdown of the operation, enabled for watchdogs, watchcats and owner
     */
    function setLockdown() public {
        bool execute=false;
        // check for owner
        if (msg.sender == owner){
            execute=true;
        }
        uint8 i;
        // check for watchdogs
        for(i=0;i<5;i++) {
            if(watchdogs[i]==msg.sender){
                execute=true;
            }
        }
        // check for watchcats
        for(i=0;i<5;i++) {
            if(watchcats[i]==msg.sender){
                execute=true;
            }
        }
        require(execute==true,"Function accessible only to owner, watchdogs and watchcats");
        lockdown=true;
    }
    /**
     * @dev unset lockdown of the operation, enabled for owner only
     */
    function unsetLockdown() public {
        // check for owner
        require (msg.sender == owner);
        // unset the lockdown 
        lockdown=false;
    }

}
/**
 * @dev Interface of the ERC20 standard as defined in the EIP.
 */
interface IERC20 {
    /**
     * @dev Returns the amount of tokens in existence.
     */
    function totalSupply() external view returns (uint256);

    /**
     * @dev Returns the amount of tokens owned by `account`.
     */
    function balanceOf(address account) external view returns (uint256);

    /**
     * @dev Moves `amount` tokens from the caller's account to `recipient`.
     *
     * Returns a boolean value indicating whether the operation succeeded.
     *
     * Emits a {Transfer} event.
     */
    function transfer(address recipient, uint256 amount) external returns (bool);

    /**
     * @dev Returns the remaining number of tokens that `spender` will be
     * allowed to spend on behalf of `owner` through {transferFrom}. This is
     * zero by default.
     *
     * This value changes when {approve} or {transferFrom} are called.
     */
    function allowance(address owner, address spender) external view returns (uint256);

    /**
     * @dev Sets `amount` as the allowance of `spender` over the caller's tokens.
     *
     * Returns a boolean value indicating whether the operation succeeded.
     *
     * IMPORTANT: Beware that changing an allowance with this method brings the risk
     * that someone may use both the old and the new allowance by unfortunate
     * transaction ordering. One possible solution to mitigate this race
     * condition is to first reduce the spender's allowance to 0 and set the
     * desired value afterwards:
     * https://github.com/ethereum/EIPs/issues/20#issuecomment-263524729
     *
     * Emits an {Approval} event.
     */
    function approve(address spender, uint256 amount) external returns (bool);

    /**
     * @dev Moves `amount` tokens from `sender` to `recipient` using the
     * allowance mechanism. `amount` is then deducted from the caller's
     * allowance.
     *
     * Returns a boolean value indicating whether the operation succeeded.
     *
     * Emits a {Transfer} event.
     */
    function transferFrom(
        address sender,
        address recipient,
        uint256 amount
    ) external returns (bool);

    /**
     * @dev Emitted when `value` tokens are moved from one account (`from`) to
     * another (`to`).
     *
     * Note that `value` may be zero.
     */
    event Transfer(address indexed from, address indexed to, uint256 value);

    /**
     * @dev Emitted when the allowance of a `spender` for an `owner` is set by
     * a call to {approve}. `value` is the new allowance.
     */
    event Approval(address indexed owner, address indexed spender, uint256 value);
}