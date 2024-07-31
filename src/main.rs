use simple_client::KuksaClient;
use simple_client::common;
use tokio;

#[tokio::main]
async fn main() {
    let vehicle = KuksaClient::new("http://127.0.0.1:55555");

    // // >>>> [DONE] TEST GET METADATA
    // let path = "Vehicle.ADAS.ABS";
    // match vehicle.get_metadata(path).await {
    //     Ok(metadatas) => {
    //         println!(">>> Metadata in path '{}'", path);
    //         for metadata in metadatas {
    //             println!("{}: {:?}\n", metadata.0, metadata.1);
    //         }
    //     },
    //     Err(err) => println!("Error: {:?}", err),
    // }

    // // >>>> [DONE] TEST GET DATATYPE
    // let path = "Vehicle.ADAS";
    // match vehicle.get_datatype(path).await {
    //     Ok(datatypes) => {
    //         println!(">>> Datatype in path '{}'", path);
    //         for datatype in datatypes {
    //             println!("{}: {:?}", datatype.0, datatype.1);
    //         }
    //     },
    //     Err(err) => println!("Error: {:?}", err),
    // }

    // // >>>> [DONE] TEST GET ENTRIES DATA
    // only leaf entry
    let paths = vec!["Vehicle.ADAS.ABS.IsEnabled", "Vehicle.Sped"];
    match vehicle.get_entries_data(paths.clone()).await {
        Ok(response) => {
            println!(">>> Get entries' value in paths '{:?}'\n", paths);
            for data_value in response {
                let value = common::value_from_option_datapoint(data_value.1);
                println!("{}: {:?}", data_value.0, value);
            }
        },
        Err(error) => {
            println!("Get entries value failed: {:?}", error);
        }
    }

    // // >>>> [DONE] TEST PUBLISH LEAF ENTRY
    // match vehicle.publish_entry_data(
    //     "Vehicle.ADAS.ABS.IsEnabled",
    //     "false"
    // ).await {
    //     Ok(_) => {
    //         println!("Publish done!");
    //     },
    //     Err(error) => {
    //         println!("Error while publishing entry data: {:?}", error);
    //     }
    // }

    // // >>>> TEST SUBSCRIBE ENTRIES
    // TODO: try to return the client ???
    match vehicle.subscribe_entries(vec!["Vehicle.Speed", "Vehicle.ADAS.ABS"]).await {
        // how to keep the client (ValClient) alive
        Ok(mut response_stream) => {
            tokio::spawn(async move {
                loop {
                    match response_stream.message().await {
                        Ok(response) => {
                            match response {
                                None => {
                                    // The stream was closed by the sender
                                    // and no more messages will be delivered
                                    println!("[None] Server gone");
                                    break;
                                },
                                Some(message) => {
                                    // The sender streamed a valid response message val

                                    println!("[Message]");

                                    for entry in message.updates {
                                        println!("\n{:?}\n", entry);
                                    }
                                }
                            }
                        }
                        Err(err) => {
                            // a gRPC error was sent by the sender instead of a valid response message.
                            // Refer to Status::code and Status::message to examine possible error causes.
                            println!("[Error] {:?}", err);
                        }
                    }
                }
            });
        },
        Err(err) => {
            println!("{:?}", err);
        }
    }
}
