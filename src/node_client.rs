use crate::{Account, Bank, NodeType, PrimaryValidator, ServerNode};

pub struct NodeClient<'a> {
    node: Node,
    nid: &'a Account,
}

pub enum Node {
    Bank(Bank),
    PrimaryValidator(PrimaryValidator),
}

#[allow(dead_code)]
impl Node {
    pub fn node_type(&self) -> NodeType {
        match self {
            Node::Bank(_) => NodeType::BANK,
            Node::PrimaryValidator(_) => NodeType::PRIMARY_VALIDATOR,
        }
    }

    fn is_bank(&self) -> bool {
        match self {
            Node::Bank(_) => true,
            _ => false,
        }
    }

    fn is_primary_validator(&self) -> bool {
        match self {
            Node::PrimaryValidator(_) => true,
            _ => false,
        }
    }
}

impl<'a> NodeClient<'a> {
    fn new(node_url: &'a str, nid: &'a Account) -> Self {
        let server_node = ServerNode::new(node_url, None);
        let node_config = server_node.get_config().unwrap();

        let node: Node = match node_config.node_type {
            NodeType::BANK => Node::Bank(Bank {
                server: server_node,
            }),
            NodeType::PRIMARY_VALIDATOR => Node::PrimaryValidator(PrimaryValidator {
                server: server_node,
            }),
        };

        NodeClient { node, nid }
    }

    fn upgrade_bank(&self) {
        if self.node.is_bank() {
            println!("Upgrade Bank");
        }
    }
}
