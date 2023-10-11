
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct StatusResponse {
    source_KME_ID: String,
    target_KME_ID: String,
    master_SAE_ID: String,
    slave_SAE_ID: String,
    key_size: u32,
    stored_key_count: u32,
    max_key_count: u32,
    max_key_per_request: u32,
    max_key_size: u32,
    min_key_size: u32,
    max_SAE_ID_count: u32,
}

#[derive(Serialize)]
pub struct KeyRequest {
    pub number: Option<u32>,
    pub size: Option<u32>,
    pub additional_slave_SAE_IDs: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyResponse {
    pub keys: Vec<QKDKey>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QKDKey {
    pub key_ID: String,
    pub key: String,
}

#[derive(Serialize)]
pub struct KeyIdRequest {
    pub key_IDs: Vec<KeyId>
}

#[derive(Serialize)]
pub struct KeyId {
    pub key_ID: String
}

#[derive(Deserialize, Debug)]
pub struct Error {
    pub message: String,
    pub details: Option<Vec<serde_json::Value>>
}
#[derive(Debug, Clone)]
pub struct EndpointETSI {
    pub KME_hostname: String,
    pub slave_SAE_ID: String,
}

const SIZE_UUID: usize = 16;
const SIZE_KEY: usize = 32;

pub struct QKDKeyPair {
    pub id: [u8; SIZE_UUID],
    pub key: [u8; SIZE_KEY]
}


// Just for testing, delete after

impl EndpointETSI {

    // TODO: IMPORTANT -> Change to https

    pub fn status(&self) -> Result<StatusResponse, Error> {
        let resp = reqwest::blocking::get(format!("http://{}/api/v1/keys/{}/status", self.KME_hostname, self.slave_SAE_ID));
        
        match resp {
            Ok(response) => {
                match response.json::<StatusResponse>() {
                    Ok(status) => Ok(status),
                    Err(err) => Err(Error {
                        message: err.to_string(),
                        details: None
                    })
                }
            },
            Err(err) => {
                Err(Error {
                    message: err.to_string(),
                    details: None
                })
            }
        }
    }

    pub fn get_key(&self, key_request: Option<&KeyRequest>) -> Result<KeyResponse, Error> {
        let client: reqwest::blocking::Client = reqwest::blocking::Client::new();

        // Create the json body
        let json_body = match key_request {
            Some(request) => request,
            None => &KeyRequest { number: Some(1), size: Some(256), additional_slave_SAE_IDs: None }
        };

        let resp = client.post(format!("http://{}/api/v1/keys/{}/enc_keys", self.KME_hostname, self.slave_SAE_ID))
                    .json(&json_body).send();
        

        match resp {
            Ok(response) => {
                if response.status().is_client_error() || response.status().is_server_error() {
                    match response.json::<Error>() {
                        Ok(error) => Err(error),
                        Err(err) => Err(Error { message: err.to_string(), details: None })
                    }
                } else {
                    match response.json::<KeyResponse>() {
                        Ok(key) => Ok(key),
                        Err(err) => Err(Error { message: err.to_string(), details: None })
                    }
                }
                
            },
            Err(err) => {
                Err(Error { message: err.to_string(), details: None })
            }
        }
    }

    pub fn get_key_with_id(&self, key_with_id: &KeyIdRequest) -> Result<KeyResponse, Error> {
        let client = reqwest::blocking::Client::new();

        // Create the json body

        // let json_body = serde_json::to_string(key_with_id).expect("Error");
        // log::debug!("Request: {:?}", client.post(format!("http://{}/api/v1/keys/{}/dec_keys", self.KME_hostname, self.slave_SAE_ID))
        // .json(&key_with_id));
        let resp = client.post(format!("http://{}/api/v1/keys/{}/dec_keys", self.KME_hostname, self.slave_SAE_ID))
                    .json(&key_with_id).send();
        // let resp = client.get(format!("http://{}/api/v1/keys/{}/dec_keys?key_ID={}", self.KME_hostname, self.slave_SAE_ID, key_with_id.key_IDs[0].key_ID)).send();


        match resp {
            Ok(response) => {
                if response.status().is_client_error() || response.status().is_server_error() {
                    match response.json::<Error>() {
                        Ok(error) => Err(error),
                        Err(err) => Err(Error { message: err.to_string(), details: None })
                    }
                } else {
                    match response.json::<KeyResponse>() {
                        Ok(key) => Ok(key),
                        Err(err) => Err(Error { message: err.to_string(), details: None })
                    }
                }
                
            },
            Err(err) => {
                Err(Error { message: err.to_string(), details: None })
            }
        }
    }


}



