extern crate grpc;
extern crate futures_cpupool;
extern crate protobuf;
extern crate tls_api;


pub mod underworlds;
pub mod underworlds_grpc;

use underworlds_grpc::*;
use underworlds::*;

use std::env;

struct Scene {
    nodes : Nodes,
}

impl Scene {
    fn read(client: &UnderworldsClient, ctxt: Context) -> Result<Scene, grpc::Error> {

        let resp = client.get_nodes_ids(grpc::RequestOptions::new(), ctxt);

        let res = resp.wait_drop_metadata()?;

        println!("Got the list of nodes: {:?}", res);
        Ok(Scene {nodes: res})

    }
}

fn main() {

    let name = String::from("rust_client");
    let world = env::args().nth(1).map(|s| s.to_owned()).unwrap_or_else(|| "base".to_owned());

    let client = UnderworldsClient::new_plain("localhost", 50051, Default::default()).unwrap();
    println!("Connected");

    let mut client_handle = Client::new();

    let mut req = Name::new();
    req.set_name(name);

    let helo_resp = client.helo(grpc::RequestOptions::new(), req);

    match helo_resp.wait_drop_metadata() {
        Ok(v) => {
            println!("Request succesful: {:?}", v);
            client_handle = v;
        }
        Err(e) => println!("Error during initial helo! {:?}",e),
    }

    let mut context = Context::new();
    context.set_client(String::from(client_handle.get_id()));
    context.set_world(world);

    let uptime_resp = client.uptime(grpc::RequestOptions::new(), client_handle);

    match uptime_resp.wait_drop_metadata() {
        Ok(v) => {
            println!("Server up since {:?}s", v.get_time());
        }
        Err(e) => println!("Error while querying uptime! {:?}",e),
    }


    let scene = Scene::read(&client, context);
    

}
