use crate::nodes::{
    server_node::ServerNode,
    traits::{ServerNodeTrait, ValidatorTrait},
};

/// Primary Validator
#[derive(Debug)]
pub struct PrimaryValidator {
    /// base server
    base: ServerNode,
}

#[allow(dead_code)]
impl ServerNodeTrait for PrimaryValidator {
    fn get_base(&self) -> &ServerNode {
        &self.base
    }
}

impl ValidatorTrait for PrimaryValidator {}

impl PrimaryValidator {
    /// Create a new primary validator instance
    ///
    /// ```
    ///     use tnb_rs::{nodes::{PrimaryValidator, ValidatorTrait, ServerNodeTrait}};
    ///
    ///     let pv = PrimaryValidator::new("http://52.52.160.149");
    ///
    /// ```
    pub fn new(url: &str) -> Self {
        PrimaryValidator {
            base: ServerNode::new(url),
        }
    }
}

#[test]

fn get_pv_config() {
    let pv = PrimaryValidator::new("http://52.52.160.149");

    println!("config: {:?}", pv.get_config().unwrap());
}
