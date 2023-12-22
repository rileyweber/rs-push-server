// use axum::prelude::*;
// use hyper;
use std::time::Duration;
use axum::{
    // prelude::*,
    extract::Path,
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use std::sync::Arc;
use std::net::SocketAddr;
use serde::{Serialize, Deserialize};
use rs_consul::{
    Consul, Config,
    types::{ ReadKeyRequest, ReadKeyResponse }, LockWatchRequest
};

#[derive(Serialize, Debug)]
struct Response {
    test: String,
    req: ReqSt
}

#[derive(Serialize, Deserialize, Debug)]
struct ConsulHello {
    this: String
}
#[derive(Serialize, Deserialize, Debug)]
struct ConsulConfigStruct {
    hello: ConsulHello
}

impl ConsulConfigStruct {
    pub fn get_hello(&self) -> &ConsulHello {
        &self.hello
    }
    // pub fn new(): Self {
    //
    // }
}

#[derive(Serialize, Deserialize, Debug)]
struct ConsulConfigStructt {
    hello: String,
    this: String,
}

#[tokio::main]
async fn main() {
    let consul = start_consul().await;

    // let aconsul = Arc::new(consul);

    // let 

    // let request = ReadKeyRequest {
    //     key: "/config/test-server.yaml",
    //     consistency: 
    // };
    let mut request = ReadKeyRequest::default();
    // let mut request = LockWatchRequest::default();
    request.key = "mykey.yml";
    // request.wait = Duration::from_secs(30);
    // request.index = 0.into();
    // let consul_config = consul.watch_lock(request).await; // .unwrap();
    let consul_config = consul.read_key(request).await; // .unwrap();
    println!("{:?}", consul_config);
    let value = consul_config.unwrap().get(0).unwrap().value.clone().unwrap();
    println!("{}", value);

    // let map = serde_yaml::Mapping::new();
    // let value = serde_yaml::Value::Mapping(map);

    let res: ConsulConfigStruct = serde_yaml::from_str(&value).unwrap();
    test_the_thing(&res);
    // let res: serde_yaml::Mapping = serde_yaml::from_str(&value).unwrap();
    // let res: serde_yaml::Value = serde_yaml::from_str(&value).unwrap();
    println!("{:?}", res);


    // let value = match consul_config {
    //     Ok(v) => {
    //         let a = v.get(0).unwrap().value.clone().unwrap();
    //         let res: ConsulConfigStruct = serde_yaml::from_str(a.as_str())
    //         println!("res is {:?}", res);
    //
    //         // println!("{:?}", a);
    //         1
    //     },
    //     Err(e) => {
    //         println!("error {:?}", e);
    //         0
    //     }
    // };
    // println!("{:?}", value);
    
    let app = Router::new()
        // .route("/push", post(|req| async { send_push(req).await}))
        .route("/push", post(send_push))
        .route("/hello-world", post(handle_get));
        // .route("/get/:key", get(|path| get_consul_key(path, aconsul)));
        // .route("/get/:key", get(|path| async { get_consul_key(path, consul).await }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8181").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn test_the_thing(config: &ConsulConfigStruct) {
    println!("config is {:?}", config);
    println!("the hello value is {:?}", config.get_hello())
}

// #[derive(Serialize)]
// struct GetConsulKeyResponse {
//     value: Vec<ReadKeyResponse>
// }

// impl GetConsulKeyResponse {
//     fn new(value: Vec<ReadKeyResponse>) -> Self {
//         Self {
//             value
//         }
//     }
// }

// async fn get_consul_key(Path(key): Path<String>, consul: Arc<Consul>) -> (StatusCode, Json<GetConsulKeyResponse>) {
//     println!("got a key of {:?}", key);
//     // let req = ReadKeyRequest {
//     //     key: "mykey.yml",
//     //     datacenter: "",
//     //     recurse: false,
//     //     namespace: 
//     // }
//     let mut req = ReadKeyRequest::default();
//     req.key = "/config/test-server.yaml";

//     // let consul_config = consul.read_key(req).await;

//     // let value = match consul_config {
//     //     Ok(v) => v,
//     //     Err(e) => println!("error uhoh uhoh {:?}", e)
//     // }
//     //
//     let value = match consul.read_key(req).await {
//         Ok(v) => v,
//         Err(err) => panic!("{}", err),//println!("error, {:?}", err),
//     };

//     // let res = GetConsulKeyResponse {
//     //     value: consul.read_key(req).await.unwrap()
//     // };

//     (StatusCode::OK, Json(res))
// }

#[derive(Deserialize, Debug, Serialize)]
struct ReqSt {
    hello: String
}

async fn handle_get(Json(payload): Json<ReqSt>) -> (StatusCode, Json<Response>) {
    println!("the request was {:?}", payload);
    let r = Response {
        test: "hello world".to_string(),
        req: payload
    };
    (StatusCode::OK, Json(r))

}


#[derive(Deserialize, Debug, Serialize, Clone)]
struct User {
    id: String,
    locale: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
struct Device {
    r#type: String,
    id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Target {
    user: User,
    token: String,
    device: Device,
}

#[derive(Deserialize, Debug, Clone)]
struct SendPushRequest {
    targets: Vec<Target>
}


#[derive(Serialize, Debug, Clone)]
struct FailedTarget {
    target: Target,
    error: String,
}

impl FailedTarget {
    fn new(error: String, target: Target) -> Self {
        Self {
            error,
            target
        }
    }
}

#[derive(Serialize, Debug, Clone)]
struct SendPushResponse {
    success: Vec<Target>,
    failed: Vec<FailedTarget>,
}
//
// impl SendPushResponse {
//     fn new(success: Vec<Target>, failed: Vec<FailedTarget>) -> Self {
//         Self {
//             Some(success),
//             Some(failed),
//         }
//     }
// }


async fn send_push(Json(payload): Json<SendPushRequest>) -> (StatusCode, Json<SendPushResponse>) {
    let response = SendPushResponse {
        success: payload.clone().targets.into(),
        failed: payload.clone().targets.iter().map(|t| FailedTarget::new("fake error".to_string(), t.clone())).collect(), };
    (StatusCode::OK, Json(response))
    // let response = SendPushResponse {
    //     success: payload.targets.into(),
    //     failed: payload.targets.iter().map(|t| FailedTarget::new("fake error".to_string(), t)).collect(),
    //     // failed: payload.targets.iter().map(|target| FailedTarget::new("fake error".to_string(), target)).collect()
    // }
    // (StatusCode::OK, Json(response))
}


async fn start_consul() -> Consul {
    // // let hyper_builer = hyper::client::Builder()
    // //     .pool_idle_timeout(Duration::from_secs(30))
    // //     .http2_only(true);
    // // let config = Config {
    // //     address: "consul.rweber.dev.use1.amx.mtmetest.com".to_string(),
    // //     hyper_builder: hyper::client::Builer()
    // //         .pool_idle_timeout(Duration::from_secs(30))
    // //         .http2_only(true),
    // //     token: None,
    // // };
    //
    let mut config = Config::default();
    config.address = "http://127.0.0.1:32770".into();
    Consul::new(config)
}
