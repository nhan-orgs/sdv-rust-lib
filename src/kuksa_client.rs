use std::collections::HashMap;

use databroker_proto::kuksa::val::v1::Error;
use databroker_proto::kuksa::val::v1::Metadata;
use databroker_proto::kuksa::val::v1::SubscribeEntry;
use databroker_proto::kuksa::val::v1::SubscribeResponse;
use databroker_proto::kuksa::val::v1::{
    val_client::ValClient, DataEntry, EntryRequest, GetRequest,
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

    pub async fn set(&self, entries: Vec<EntryUpdate>) -> Result<(), ClientError> {
        if let Ok(mut client) = ValClient::connect(self.server_address.clone()).await {
            let request = SetRequest { updates: entries };

            match client.set(request).await {
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

                    if errors.len() > 0 {
                        return Err(ClientError::Function(errors));
                    } else {
                        return Ok(());
                    }
                }
                Err(err) => return Err(ClientError::Status(err)),
            }
        } else {
            Err(ClientError::Connection(
                "Can not connect ValClient".to_string(),
            ))
        }
    }

    pub async fn get_metadata(
        &self,
        entry_path: &str,
    ) -> Result<HashMap<String, Metadata>, ClientError> {
        match self
            .get(
                entry_path,
                View::Metadata.into(),
                vec![Field::Metadata.into()],
            )
            .await
        {
            Ok(response) => {
                let mut result = HashMap::new();
                let data_entries = response.into_iter();

                for data_entry in data_entries {
                    if let Some(metadata) = data_entry.metadata {
                        result.insert(data_entry.path, metadata);
                    }
                }

                Ok(result)
            }
            Err(error) => {
                // Err: can not access GET METADATA method
                return Err(error);
            }
        }
    }

    pub async fn get_datatype(
        &self,
        entry_path: &str,
    ) -> Result<HashMap<String, DataType>, ClientError> {
        match self.get_metadata(entry_path).await {
            Ok(metadatas) => {
                let mut result = HashMap::new();

                for metadata in metadatas {
                    match DataType::try_from(metadata.1.data_type) {
                        Ok(datatype) => {
                            result.insert(metadata.0, datatype);
                        },
                        Err(_error) => {
                            println!("Decode error:  DataType::try_from() failed");
                            return Err(ClientError::Function(vec![]));
                        }
                    }
                }

                Ok(result)
            }
            Err(error) => {
                return Err(error);
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

    pub async fn publish_entry_data(
        &self,
        entry_path: &str,
        value: &str,
    ) -> Result<(), ClientError> {
        println!("------ publish_entry_data: ");
        println!("entry_path: {:?}", entry_path);
        println!("value: {:?}", value);
        println!();

        match self.get_datatype(entry_path).await {
            Ok(datatype) => {
                // datatype to value
                // check if entry_path exist in datatype hashmap
                if !datatype.contains_key(entry_path) {
                    return Err(ClientError::Function(vec![
                        Error{
                            code: 401,
                            reason: "Error retrieve metadata".to_string(),
                            message: "Can not found metadata for path, path maybe not a leaf entry".to_string(),
                        }
                    ]));
                }

                match str_to_value(value, datatype[entry_path]) {
                    Ok(entry_value) => {
                        let entry = EntryUpdate {
                            fields: vec![Field::Value as i32],
                            entry: Some(DataEntry {
                                path: entry_path.to_string(),
                                value: Some(Datapoint {
                                    timestamp: Some(std::time::SystemTime::now().into()),
                                    value: Some(entry_value),
                                }),
                                metadata: None,
                                actuator_target: None,
                            }),
                        };

                        if let Err(error) = self.set(vec![entry]).await {
                            return Err(error);
                        }

                        Ok(())
                    }

                    Err(_) => {
                        return Err(ClientError::Function(vec![Error {
                            code: 400,
                            reason: "Convert data value error".to_string(),
                            message: "Can not convert string to {$datatype}".to_string(),
                        }]));
                    }
                }
            }

            Err(err) => {
                // return METADATA error
                return Err(err);
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
