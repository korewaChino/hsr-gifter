use serde::{Deserialize, Serialize};

// example of a simple message
// {"retcode":0,"message":"OK","data":{"msg":"Redeemed successfully"}}
// 
// example of an error message
// 
// {"data":null,"message":"Redemption code has already been used","retcode":-2017}

pub(crate) type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Return error: {0}")]
    Return(Return),
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Return {
    pub retcode: i32,
    pub message: String,
    pub data: Option<Message>,
}

impl std::fmt::Display for Return {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.retcode, self.message)
    }
}


impl Return {
    pub fn is_ok(&self) -> bool {
        self.retcode == 0
    }
    
    pub fn import(data: &str) -> Result<Self> {
        let ret: Return = serde_json::from_str(data).map_err(Error::Json)?;
        if ret.is_ok() {
            Ok(ret)
        } else {
            Err(Error::Return(ret))
        }
    }
}

impl From<String> for Return {
    fn from(s: String) -> Self {
        if let Ok(ret) = serde_json::from_str(&s) {
            ret
        } else {
            panic!("Failed to parse JSON: {}", s);
        }
    }
}

