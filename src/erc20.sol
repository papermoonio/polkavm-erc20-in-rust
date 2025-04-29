
interface IERC20 {
    constructor(
        string memory _name,
        string memory _symbol,
        uint8 _decimals,
        uint256 _initialSupply
    )
    // Get the total token supply
    function name() external view returns (string memory);

    // Get the total token supply
    function symbol() external view returns (string memory);

    // Get the total token supply
    function decimals() external view returns (uint256);

    // Get the total token supply
    function totalSupply() external view returns (uint256);

    // Get the token balance of an account
    function balanceOf(address account) external view returns (uint256);

    // Get the amount of tokens approved for spending by another address
    function allowance(
        address owner,
        address spender
    ) external view returns (uint256);

    // Transfer tokens to a specified address
    function transfer(
        address recipient,
        uint256 amount
    ) external returns (bool);
    
    // Approve a third party to spend tokens from your account
    function approve(address spender, uint256 amount) external returns (bool);

    // Transfer tokens from one address to another within the approved allowance
    function transferFrom(
        address sender,
        address recipient,
        uint256 amount
    ) external returns (bool);
}

// ERC20 just for generate the abi
// we can't use interface since the constructor is not allowed in interface
contract ERC20 {
    string public name;
    string public symbol;
    uint8 public decimals;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    constructor(
        string memory _name,
        string memory _symbol,
        uint8 _decimals,
        uint256 _initialSupply
    ) {
    }

    function transfer(
        address _to,
        uint256 _value
    ) public returns (bool success) {
        
        return true;
    }

    function approve(
        address _spender,
        uint256 _value
    ) public returns (bool success) {
        
        return true;
    }

    function transferFrom(
        address _from,
        address _to,
        uint256 _value
    ) public returns (bool success) {
        
        return true;
    }
}