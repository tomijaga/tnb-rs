(function () {
  var implementors = {};
  implementors["tnb_rs"] = [
    {
      text: 'impl <a class="trait" href="https://doc.rust-lang.org/1.56.0/core/marker/trait.Send.html" title="trait core::marker::Send">Send</a> for <a class="enum" href="tnb_rs/enum.NodeType.html" title="enum tnb_rs::NodeType">NodeType</a>',
      synthetic: true,
      types: ["tnb_rs::account::NodeType"],
    },
    {
      text: 'impl&lt;\'tx&gt; <a class="trait" href="https://doc.rust-lang.org/1.56.0/core/marker/trait.Send.html" title="trait core::marker::Send">Send</a> for <a class="struct" href="tnb_rs/struct.Transaction.html" title="struct tnb_rs::Transaction">Transaction</a>&lt;\'tx&gt;',
      synthetic: true,
      types: ["tnb_rs::account::Transaction"],
    },
    {
      text: 'impl&lt;\'a&gt; <a class="trait" href="https://doc.rust-lang.org/1.56.0/core/marker/trait.Send.html" title="trait core::marker::Send">Send</a> for <a class="enum" href="tnb_rs/enum.BlockType.html" title="enum tnb_rs::BlockType">BlockType</a>&lt;\'a&gt;',
      synthetic: true,
      types: ["tnb_rs::account::BlockType"],
    },
    {
      text: 'impl&lt;\'a&gt; <a class="trait" href="https://doc.rust-lang.org/1.56.0/core/marker/trait.Send.html" title="trait core::marker::Send">Send</a> for <a class="struct" href="tnb_rs/struct.BlockMessage.html" title="struct tnb_rs::BlockMessage">BlockMessage</a>&lt;\'a&gt;',
      synthetic: true,
      types: ["tnb_rs::account::BlockMessage"],
    },
    {
      text: 'impl <a class="trait" href="https://doc.rust-lang.org/1.56.0/core/marker/trait.Send.html" title="trait core::marker::Send">Send</a> for <a class="enum" href="tnb_rs/enum.ChainData.html" title="enum tnb_rs::ChainData">ChainData</a>',
      synthetic: true,
      types: ["tnb_rs::account::ChainData"],
    },
    {
      text: 'impl&lt;\'a&gt; <a class="trait" href="https://doc.rust-lang.org/1.56.0/core/marker/trait.Send.html" title="trait core::marker::Send">Send</a> for <a class="struct" href="tnb_rs/struct.SignedMessage.html" title="struct tnb_rs::SignedMessage">SignedMessage</a>&lt;\'a&gt;',
      synthetic: true,
      types: ["tnb_rs::account::SignedMessage"],
    },
    {
      text: 'impl <a class="trait" href="https://doc.rust-lang.org/1.56.0/core/marker/trait.Send.html" title="trait core::marker::Send">Send</a> for <a class="struct" href="tnb_rs/struct.Account.html" title="struct tnb_rs::Account">Account</a>',
      synthetic: true,
      types: ["tnb_rs::account::Account"],
    },
    {
      text: 'impl <a class="trait" href="https://doc.rust-lang.org/1.56.0/core/marker/trait.Send.html" title="trait core::marker::Send">Send</a> for <a class="struct" href="tnb_rs/struct.HDWallet.html" title="struct tnb_rs::HDWallet">HDWallet</a>',
      synthetic: true,
      types: ["tnb_rs::hd_wallet::HDWallet"],
    },
  ];
  if (window.register_implementors) {
    window.register_implementors(implementors);
  } else {
    window.pending_implementors = implementors;
  }
})();
