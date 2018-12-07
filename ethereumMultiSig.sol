pragma solidity ^0.4.24;
contract TesseractMultisig {
    
    //public fields
    uint epochSize;
    uint claimSize;
    uint withdrawSize;
    uint blockStartNum;
    uint globalCoinCounter;
    State currentState;
    mapping(address => TwoWayChannel) public balances;
    address[] public activeUsers;
    address public tesseract = 0x18F27271966963AA375f35950407f5074B16fEF7;  //hard code tesseract address
    
    //enum to manage state for each acct
    enum State {Epoch, Claim, Withdraw}
    
    //redeem request definition
    struct ClaimRequest{
        uint globalCoinID;
        uint nonce;
        uint coinVal;
    }
    
    //two-way channel definition (value in contract balance mapping)
    struct TwoWayChannel {
        uint value;
        uint numSplits;
        ClaimRequest[] coins;
    }
    
    //three possible events
    event Deposit(address _sender, uint _value);
    event Claim(address _owner, address _sender, uint _coinID);
    event Withdraw(address invoker, uint nextEpochBlock);
    
    //modifier for initiating party -> HARD CODE IN TESSERACT ADDRESS SO CAN'T BE MODIFIED
    modifier onlyInitial() {
        require(
            msg.sender == tesseract,
            "Only Tesseract can call this."
        );
        _;
    }
    
    //contract constructor
    constructor() public onlyInitial(){
        epochSize = 5;      //these can be hardcoded or determined at deploy time based on ETH block rate
        claimSize = 5;
        withdrawSize = 3;
        blockStartNum = block.number;
        currentState = State.Epoch;
        globalCoinCounter = 0;
    }
    
    //deposit function
    function deposit(uint _numSplits) payable public {
        require(determineState() == State.Epoch, "Can only deposit during Epoch phase.");
        activeUsers.push(msg.sender);
        if(msg.value > 0.0 && balances[msg.sender].value==0){
            balances[msg.sender].value = msg.value;
            balances[msg.sender].numSplits = _numSplits;
            for(uint a=0;a<_numSplits;a++){
                ClaimRequest memory initialCR = ClaimRequest(globalCoinCounter++,0,msg.value/_numSplits);
                balances[msg.sender].coins.push(initialCR);
            }
            emit Deposit(msg.sender, msg.value);
        }
    }
    
    //claim function
    function claim(address _owner, bytes32 hash, uint _coinIndex, uint _globalCoinID, uint _nonce, uint8 ownerFirst, bytes32 ownerSecond, bytes32 ownerThird, uint8 tesseractFirst, bytes32 tesseractSecond, bytes32 tesseractThird) public{
        require(determineState() == State.Claim, "Can only claim during Claim phase.");
        require(verifySigner(_owner, hash, ownerFirst, ownerSecond, ownerThird) == true, "Failed owner signature check.");     //verify owner signature
        require(verifySigner(tesseract, hash, tesseractFirst, tesseractSecond, tesseractThird) == true, "Failed tesseract signature check.");        //verify tesseract signature
        require(balances[_owner].coins[_coinIndex].globalCoinID == _globalCoinID, "Accessing incorrect coin, check the blockchain as it may have been traded or a user is making malicious claims.");     //verify global coin id matches 
        if(_nonce > balances[_owner].coins[_coinIndex].nonce){
            uint _coinVal = balances[_owner].coins[_coinIndex].coinVal;
            balances[_owner].coins[_coinIndex].coinVal = 0;
            ClaimRequest memory request = ClaimRequest(_globalCoinID, _nonce, _coinVal);
            balances[msg.sender].coins.push(request); 
        }
        emit Claim(_owner, msg.sender, _globalCoinID);
    }
    
    //withdraw function ->callable by anyone and will envoke all withdraws
    function withdraw() public{
        require(determineState() == State.Withdraw, "Can only withdraw during Withdraw phase.");
        for(uint k=0;k<activeUsers.length;k++){     //does the withdrawls for all users
            uint transferAmount = 0;
            for(uint l=0;l<balances[activeUsers[k]].coins.length;l++){
                if(balances[activeUsers[k]].coins[l].coinVal > 0){
                    transferAmount += balances[activeUsers[k]].coins[l].coinVal;
                } 
            }
            balances[activeUsers[k]].value = 0;
            balances[activeUsers[k]].numSplits = 0;
            if(transferAmount > 0) activeUsers[k].transfer(transferAmount); 
            delete balances[activeUsers[k]].coins;
        }
        delete activeUsers;
        currentState = State.Epoch;
        blockStartNum = block.number;
        globalCoinCounter = 0;
        emit Withdraw(msg.sender, blockStartNum);
    }
    
    
    //function to determine current global Tesseract state
    function determineState() public view returns(State theState){
        uint currentBlock = block.number;
        uint cycleSize = epochSize + claimSize + withdrawSize;
        uint cycles = (currentBlock - blockStartNum)/cycleSize;
        if ((currentBlock - (cycles*cycleSize+blockStartNum)) < epochSize){
            return State.Epoch;
        }
        else if ((currentBlock - (cycles*cycleSize+blockStartNum)) < (epochSize+claimSize)){
            return State.Claim;
        }
        else{
            return State.Withdraw;
        }
    }
      

    //function to verify signer
    function verifySigner(address p, bytes32 hash, uint8 v, bytes32 r, bytes32 s) internal pure returns(bool) {
        return ecrecover(hash, v, r, s) == p;   //checks that address recovered from signed data matches owner's
    }
    

}