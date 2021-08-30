use crate::{
  debug,
  log,
};
use bytecursor::ByteCursor;
use reqwest::{
  self,
  multipart,
  Client,
};
use serde_json;
use sp_ipld::{
  dag_cbor::{
    cid,
    DagCborCodec,
  },
  Codec,
  Ipld,
};
use std::sync::{
  Arc,
  Mutex,
};

/// Settings for how to connect to an IPFS Http API
#[derive(Debug, Clone)]
pub struct IpfsApi {
  host: String,
}

fn log_err<T, E: std::fmt::Debug>(e: E) -> Result<T, ()> {
  log!("{:?}", e);
  Err(())
}

impl IpfsApi {
  pub fn ipfs_yatima_io() -> Self { IpfsApi { host: "http://ipfs.yatima.io:5001".to_owned() } }

  pub fn local_daemon() -> Self { IpfsApi { host: "http://localhost:5001".to_owned() } }

  /// Pin an Ipld using the IPFS API
  pub fn dag_put_with_callback(&self, dag: Ipld) -> Result<String, String> {
    let url = format!(
      "{}{}?{}",
      self.host, "/api/v0/dag/put", "format=cbor&pin=true&input-enc=cbor&hash=blake2b-256"
    );
    let cbor =
      DagCborCodec.encode(&dag).map_err(|e| format!("encoding error: {:?}", e))?.into_inner();
    let client = Client::new();
    let form = multipart::Form::new().part("file", multipart::Part::bytes(cbor));
    let ptr: Arc<Mutex<serde_json::Value>> = Arc::new(Mutex::new(serde_json::Value::Null));
    let ptr2 = ptr.clone();
    wasm_bindgen_futures::spawn_local(async move {
      let r: serde_json::Value = client
        .post(&url)
        .multipart(form)
        .send()
        .await
        .or_else(log_err)
        .unwrap()
        .json()
        .await
        .unwrap();
      *ptr2.lock().unwrap() = r;
    });

    let response = ptr.lock().unwrap();
    let ipfs_cid: String = response["Cid"]["/"].as_str().unwrap().to_string();
    let local_cid: String = cid(&dag).to_string();

    if ipfs_cid == local_cid {
      Ok(ipfs_cid)
    }
    else {
      Err(format!("CIDs are different {} != {}", ipfs_cid, local_cid))
    }
  }

  /// Load Ipld from the IPFS API
  pub fn dag_get_with_callback(
    &self,
    cid: String,
    callback: Box<dyn FnOnce(Result<Ipld, String>)>,
  ) {
    let url = format!("{}{}?arg={}", self.host, "/api/v0/block/get", cid);
    // let ptr: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(vec![]));
    // let ptr2 = ptr.clone();
    wasm_bindgen_futures::spawn_local(async move {
      let client = Client::new();
      log!("Trying to call IPFS api at {}", url);
      let response = client.post(&url).send().await.or_else(log_err);
      if response.is_err() {
        return;
      }
      let response = response.unwrap().bytes().await.or_else(log_err).unwrap().to_vec();
      debug!("response: {:?}", &response);
      // *ptr2.lock().unwrap() = r.into();

      let ipld_res = DagCborCodec
        .decode(ByteCursor::new(response.to_vec()))
        .map_err(|e| format!("Invalid ipld cbor: {}", e));

      callback(ipld_res);
    });
    // let response = ptr.lock().unwrap();
    // log!("response: {:?}", response);
  }

  pub async fn dag_put(&self, dag: Ipld) -> Result<String, reqwest::Error> {
    let url = format!(
      "{}{}?{}",
      self.host, "/api/v0/dag/put", "format=cbor&pin=true&input-enc=cbor&hash=blake2b-256"
    );
    let cbor = DagCborCodec.encode(&dag).unwrap().into_inner();
    let client = Client::new();
    let form = multipart::Form::new().part("file", multipart::Part::bytes(cbor));
    let response: serde_json::Value =
      client.post(url).multipart(form).send().await?.json().await?;

    let ipfs_cid: String = response["Cid"]["/"].as_str().unwrap().to_string();
    let local_cid: String = cid(&dag).to_string();

    if ipfs_cid == local_cid {
      Ok(ipfs_cid)
    }
    else {
      panic!("CIDs are different {} != {}", ipfs_cid, local_cid);
    }
  }

  pub async fn dag_get(&self, cid: String) -> Result<Ipld, reqwest::Error> {
    let url = format!("{}{}?arg={}", self.host, "/api/v0/block/get", cid);
    let client = Client::new();
    let response = client.post(url).send().await?.bytes().await?;
    let response = response.to_vec();
    debug!("response: {:?}", response);
    let ipld = DagCborCodec.decode(ByteCursor::new(response)).expect("invalid ipld cbor.");

    Ok(ipld)
  }
}

#[cfg(test)]
mod tests {
  use crate::file::store::{
    FileStore,
    FileStoreOpts,
  };
  use std::{
    path::PathBuf,
    rc::Rc,
  };
  use yatima_utils::file::parse::{
    parse_text,
    PackageEnv,
  };
  #[ignore]
  #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
  async fn test_get_boolya() {
    let src = "
    package bool where

    def Bool: Type = #Bool

    def Bool.True: Bool = #Bool.true
    def Bool.False: Bool = #Bool.false

    def Bool.eql: ∀ (x y: Bool) -> Bool = #Bool.eql
    def Bool.lte: ∀ (x y: Bool) -> Bool = #Bool.lte
    def Bool.lth: ∀ (x y: Bool) -> Bool = #Bool.lth
    def Bool.gte: ∀ (x y: Bool) -> Bool = #Bool.gte
    def Bool.gth: ∀ (x y: Bool) -> Bool = #Bool.gth


    def Bool.and: ∀ (x y: Bool) -> Bool = #Bool.and
    def Bool.or:  ∀ (x y: Bool) -> Bool = #Bool.or
    def Bool.xor: ∀ (x y: Bool) -> Bool = #Bool.xor

    def Bool.not: ∀ (x: Bool) -> Bool = #Bool.not

    def Bool.neq (x y: Bool): Bool = Bool.not (Bool.eql x y)

    def Bool.if (A: Type) (bool : Bool) (t f: A): A = (case bool) (λ _ => A) t f
    ";
    let root = std::env::current_dir().unwrap();
    let path = PathBuf::from("bool.ya");
    let store = Rc::new(FileStore::new(FileStoreOpts {
      use_ipfs_daemon: true,
      use_file_store: true,
      root: root.clone(),
    }));
    let env = PackageEnv::new(root, path, store);
    let (cid, p, _defs) = parse_text(src, env).unwrap();

    let ipld = super::dag_get(cid.to_string()).await.unwrap();
    assert_eq!(ipld, p.to_ipld());
  }
}