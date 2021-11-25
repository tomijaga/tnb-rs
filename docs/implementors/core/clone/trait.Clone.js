(function () {
  var implementors = {};
  implementors["tnb_rs"] = [
    {
      text: 'impl <a class="trait" href="https://doc.rust-lang.org/1.56.0/core/clone/trait.Clone.html" title="trait core::clone::Clone">Clone</a> for <a class="enum" href="tnb_rs/enum.NodeType.html" title="enum tnb_rs::NodeType">NodeType</a>',
      synthetic: false,
      types: ["tnb_rs::account::NodeType"],
    },
    {
      text: 'impl <a class="trait" href="https://doc.rust-lang.org/1.56.0/core/clone/trait.Clone.html" title="trait core::clone::Clone">Clone</a> for <a class="struct" href="tnb_rs/struct.Transaction.html" title="struct tnb_rs::Transaction">Transaction</a>',
      synthetic: false,
      types: ["tnb_rs::account::Transaction"],
    },
    {
      text: 'impl <a class="trait" href="https://doc.rust-lang.org/1.56.0/core/clone/trait.Clone.html" title="trait core::clone::Clone">Clone</a> for <a class="enum" href="tnb_rs/enum.BlockType.html" title="enum tnb_rs::BlockType">BlockType</a>',
      synthetic: false,
      types: ["tnb_rs::account::BlockType"],
    },
    {
      text: 'impl <a class="trait" href="https://doc.rust-lang.org/1.56.0/core/clone/trait.Clone.html" title="trait core::clone::Clone">Clone</a> for <a class="struct" href="tnb_rs/struct.BlockMessage.html" title="struct tnb_rs::BlockMessage">BlockMessage</a>',
      synthetic: false,
      types: ["tnb_rs::account::BlockMessage"],
    },
  ];
  if (window.register_implementors) {
    window.register_implementors(implementors);
  } else {
    window.pending_implementors = implementors;
  }
})();
