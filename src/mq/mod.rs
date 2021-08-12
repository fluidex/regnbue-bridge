pub mod messages {
    use serde::{Deserialize, Serialize};

    #[derive(Debug)]
    pub enum WrappedMessage {
        User(UserMessage),
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct UserMessage {
        pub user_id: i32,
        pub l1_address: String,
        pub l2_pubkey: String,
    }
}
