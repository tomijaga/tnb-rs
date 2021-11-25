(function () {
  var implementors = {};
  implementors["tnb_rs"] = [
    {
      text: 'impl <a class="trait" href="https://docs.rs/serde/1.0.130/serde/ser/trait.Serialize.html" title="trait serde::ser::Serialize">Serialize</a> for <a class="enum" href="tnb_rs/enum.NodeType.html" title="enum tnb_rs::NodeType">NodeType</a>',
      synthetic: false,
      types: ["tnb_rs::account::NodeType"],
    },
    {
      text: 'impl&lt;\'tx&gt; <a class="trait" href="https://docs.rs/serde/1.0.130/serde/ser/trait.Serialize.html" title="trait serde::ser::Serialize">Serialize</a> for <a class="struct" href="tnb_rs/struct.Transaction.html" title="struct tnb_rs::Transaction">Transaction</a>&lt;\'tx&gt;',
      synthetic: false,
      types: ["tnb_rs::account::Transaction"],
    },
    {
      text: 'impl&lt;\'a&gt; <a class="trait" href="https://docs.rs/serde/1.0.130/serde/ser/trait.Serialize.html" title="trait serde::ser::Serialize">Serialize</a> for <a class="enum" href="tnb_rs/enum.BlockType.html" title="enum tnb_rs::BlockType">BlockType</a>&lt;\'a&gt;',
      synthetic: false,
      types: ["tnb_rs::account::BlockType"],
    },
    {
      text: 'impl&lt;\'a&gt; <a class="trait" href="https://docs.rs/serde/1.0.130/serde/ser/trait.Serialize.html" title="trait serde::ser::Serialize">Serialize</a> for <a class="struct" href="tnb_rs/struct.BlockMessage.html" title="struct tnb_rs::BlockMessage">BlockMessage</a>&lt;\'a&gt;',
      synthetic: false,
      types: ["tnb_rs::account::BlockMessage"],
    },
    {
      text: 'impl <a class="trait" href="https://docs.rs/serde/1.0.130/serde/ser/trait.Serialize.html" title="trait serde::ser::Serialize">Serialize</a> for <a class="enum" href="tnb_rs/enum.ChainData.html" title="enum tnb_rs::ChainData">ChainData</a>',
      synthetic: false,
      types: ["tnb_rs::account::ChainData"],
    },
    {
      text: 'impl&lt;\'a&gt; <a class="trait" href="https://docs.rs/serde/1.0.130/serde/ser/trait.Serialize.html" title="trait serde::ser::Serialize">Serialize</a> for <a class="struct" href="tnb_rs/struct.SignedMessage.html" title="struct tnb_rs::SignedMessage">SignedMessage</a>&lt;\'a&gt;',
      synthetic: false,
      types: ["tnb_rs::account::SignedMessage"],
    },
  ];
  if (window.register_implementors) {
    window.register_implementors(implementors);
  } else {
    window.pending_implementors = implementors;
  }
})();
