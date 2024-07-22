use simple_client::KuksaClient;
use tokio;

#[tokio::main]
async fn main() {
    let vehicle = KuksaClient::new("http://127.0.0.1:55555");

    // >>>> TEST GET ENTRIES DATA
    // match vehicle.get_datatype("Vehicle.ADAS.ABS").await {
    //     Ok(datatype) => println!("Datatype: {:?}", datatype),
    //     Err(err) => println!("Error: {:?}", err),
    // }
    
    // >>>> TEST PUBLISH LEAF ENTRY
    let publish_response = vehicle.publish_entry_data(
        "Vehicle.Speed", 
        "100001"
    ).await;
    println!("{:?}", publish_response);

    let get_response = vehicle.get_entries_data(vec!["Vehicle.ADAS.ABS", "Vehicle.Speed"]).await;
    println!("{:?}", get_response);

    // // >>>> TEST SUBSCRIBE ENTRIES
    // match vehicle.subscribe_entries(vec!["Vehicle.Speed", "Vehicle.ADAS.ABS"]).await {
    //     // how to keep the client (ValClient) alive
    //     Ok(mut response_stream) => {
    //         println!("{:?}", response_stream);

    //         tokio::spawn(async move {
    //             let mut i = 0;
    //             loop {
    //                 i = i + 1;
    //                 print!("\n>>> {}: ", i);

    //                 match response_stream.message().await {
    //                     Ok(response) => {
    //                         match response {
    //                             None => {
    //                                 // The stream was closed by the sender 
    //                                 // and no more messages will be delivered
    //                                 println!("[None] Server gone");
    //                                 break;
    //                             },
    //                             Some(message) => {
    //                                 // The sender streamed a valid response message val
    //                                 println!("[Message] {:?}", message);
    //                             }
    //                         }
    //                     }
    //                     Err(err) => {
    //                         // a gRPC error was sent by the sender instead of a valid response message. 
    //                         // Refer to Status::code and Status::message to examine possible error causes.
    //                         println!("[Error] {:?}", err);
    //                     }
    //                 }
    //             }
    //         });
    //     },
    //     Err(err) => {
    //         println!("{:?}", err);
    //     }
    // }
}
