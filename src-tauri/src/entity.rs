use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct KCoinfig {
    cburl: String,
    wsurl: String,
    file_dir: String,
}