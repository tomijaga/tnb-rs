initSidebarItems({
  constant: [
    [
      "MAX_CHILD_INDEX",
      "The max number for the account_index and address_index",
    ],
  ],
  enum: [
    ["BlockType", "Contains the structure of supported block types"],
    ["ChainData", "Enum for Supported Node Requests"],
    ["NodeType", "Enum that specifies a Node’s type"],
  ],
  struct: [
    ["Account", "An Account consists of an account number and a signing key."],
    ["BlockMessage", "Block structure to make a block request on the network"],
    [
      "HDWallet",
      "A Hierarchical Deterministic Wallet (HD Wallet) is a wallet that generates child keys (Accounts) from a mnemonic phrase and stores them in a tree like structure.",
    ],
    ["SignedMessage", "Structure for making Node requests to the network"],
    ["Transaction", "Transaction Data"],
  ],
});
