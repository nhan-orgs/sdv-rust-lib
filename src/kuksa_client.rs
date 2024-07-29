use std::collections::HashMap;

use databroker_proto::kuksa::val::v1::SubscribeEntry;
use databroker_proto::kuksa::val::v1::SubscribeResponse;
use databroker_proto::kuksa::val::v1::{
    val_client::ValClient, DataEntry, EntryRequest, Error, GetRequest,
};
use databroker_proto::kuksa::val::v1::{DataType, SubscribeRequest};
use databroker_proto::kuksa::val::v1::{Datapoint, EntryUpdate, Field, SetRequest, View};
use tonic::Request;
use tonic::Streaming;

use crate::common::{str_to_value, ClientError};

pub struct KuksaClient {
    pub server_address: String,
}

impl KuksaClient {
    pub fn new(server_address: &str) -> Self {
        KuksaClient {
            server_address: server_address.to_string(),
        }
    }

    pub async fn get(
        &self,
        path: &str,
        view: i32,
        fields: Vec<i32>,
    ) -> Result<Vec<DataEntry>, ClientError> {
        if let Ok(mut client) = ValClient::connect(self.server_address.clone()).await {
            let request = GetRequest {
                entries: vec![EntryRequest {
                    path: path.to_string(),
                    view: view,
                    fields: fields,
                }],
            };

            match client.get(Request::new(request)).await {
                Ok(response) => {
                    let message = response.into_inner();

                    // collect errors from response
                    let mut errors = vec![];

                    if let Some(err) = message.error {
                        errors.push(err);
                    }

                    for error in message.errors {
                        if let Some(err) = error.error {
                            errors.push(err);
                        }
                    }

                    // check if return error or entries' value
                    if errors.len() > 0 {
                        return Err(ClientError::Function(errors));
                    } else {
                        return Ok(message.entries);
                    }
                }
                Err(error) => {
                    return Err(ClientError::Status(error));
                }
            }
        } else {
            return Err(ClientError::Connection(
                "Can not connect ValClient".to_string(),
            ));
        }
    }

    pub async fn get_datatype(&self, entry_path: &str) -> Result<DataType, ClientError> {
        let mut client = ValClient::connect(self.server_address.clone())
            .await
            .unwrap();

        let request = GetRequest {
            entries: vec![EntryRequest {
                path: entry_path.to_string(),
                view: View::Metadata.into(),
                fields: vec![Field::Metadata.into()],
            }],
        };

        match client.get(Request::new(request)).await {
            Ok(response) => {
                let metadata = response.into_inner();

                println!("Metadata response from server:\n{:?}\n", metadata);

                if metadata.entries.len() != 1 {
                    // no entry found/not a leaf --> get err message from response
                    println!("len = {}\n", metadata.entries.len());
                    return Err(ClientError::Function(vec![]));
                };

                match &metadata.entries[0].metadata {
                    Some(data) => {
                        // exist metadata
                        match DataType::try_from(data.data_type) {
                            Ok(datatype) => {
                                // get datatype sucessfully
                                return Ok(datatype);
                            }
                            Err(_err) => {
                                // can not convert i32 to Datatype
                                return Err(ClientError::Function(vec![]));
                            }
                        }
                    }
                    None => {
                        // no metadata found
                        return Err(ClientError::Function(vec![]));
                    }
                }
            }
            Err(status) => {
                // Err: can not access GET METADATA method
                Err(ClientError::Status(status))
            }
        }
    }

    pub async fn get_entries_data(
        &self,
        entries_path: Vec<&str>,
    ) -> Result<HashMap<String, Option<Datapoint>>, ClientError> {
        // get data of list entries
        // return a hash map (path: value);

        println!("------ get_entries_data:");
        println!("entries_path: {:?}\n", entries_path);

        let mut result = HashMap::new();

        for entry_path in entries_path {
            match self
                .get(
                    entry_path,
                    View::CurrentValue.into(),
                    vec![Field::Value.into()],
                )
                .await
            {
                Ok(entries) => {
                    for entry in entries {
                        result.insert(entry.path, entry.value);
                    }
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }

        Ok(result)
    }

    pub async fn publish_entry_data(&self, entry_path: &str, value: &str) -> Result<(), Error> {
        println!("------ publish_entry_data: ");
        println!("entry_path: {:?}", entry_path);
        println!("value: {:?}", value);
        println!();

        // create a ValClient and connect server
        // create a ValClient and connect server
        let mut client = ValClient::connect(self.server_address.clone())
            .await
            .unwrap();

        // get entry datatype
        match self.get_datatype(entry_path).await {
            Ok(datatype) => {
                // datatype to value
                let entry_value = str_to_value(value, datatype).unwrap();

                // convert entry_path and value from &str into PUBLISH request
                let request = SetRequest {
                    updates: vec![EntryUpdate {
                        fields: vec![Field::Value as i32],
                        entry: Some(DataEntry {
                            path: entry_path.to_string(),
                            value: Some(Datapoint {
                                timestamp: Some(std::time::SystemTime::now().into()),
                                value: Some(entry_value),
                                // Some(Value::Float(101.1)),
                            }),
                            metadata: None,
                            actuator_target: None,
                        }),
                    }],
                };

                println!("{:?}\n", request);

                // call PUBLISH method
                let response = client.set(request).await.unwrap().into_inner();

                // parse response and return () or Error
                println!("Publish response from server:\n{:?}\n", response);
                return Ok(());
            }
            Err(_err) => {
                // return METADATA error
                return Err(Error {
                    code: 1,
                    reason: "reason".to_string(),
                    message: "METADATA error".to_string(),
                });
            }
        }
    }

    pub async fn subscribe_entries(
        &self,
        entries_path: Vec<&str>,
    ) -> Result<Streaming<SubscribeResponse>, ClientError> {
        println!("------ subcribe entries:");
        println!("entries_path: {:?}", entries_path);

        // create ValClient
        let mut client = ValClient::connect(self.server_address.clone())
            .await
            .unwrap();

        // entries_path --> SubscribeRequest
        let mut entries = Vec::new();

        for entry_path in entries_path {
            entries.push(SubscribeEntry {
                path: entry_path.to_string(),
                view: View::CurrentValue.into(),
                fields: vec![Field::Value.into()],
            })
        }

        let request = SubscribeRequest { entries };

        // call subcribes method
        match client.subscribe(request).await {
            Ok(response) => {
                return Ok(response.into_inner()); // return response stream
            }
            Err(err) => {
                return Err(ClientError::Status(err));
            }
        }
    }
}
