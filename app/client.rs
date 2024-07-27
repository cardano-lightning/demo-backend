use cldb_tonic::{AddReq, OpenReq, StatusReq};
use cryptoxide::ed25519;

use cldb_tonic::cldb_client::CldbClient;

pub mod cldb_tonic {
    tonic::include_proto!("cldb");
}

// fn mk_witness(: [u8; 64], iou: u64) -> Witness {

//     let nonce = Uuid::new_v4().as_bytes().clone();
//     let message = mk_message(iou, &nonce);
//     let sig = ed25519::signature(&message, &keypair);
//     Witness { iou, nonce, sig }
// }
// 
// fn mk_request(keypair: [u8; 64], id: &str, iou: u64) -> tonic::Request<TimeReq> {
//     let witness = mk_witness(keypair, iou);
//     tonic::Request::new(TimeReq {
//         id: id.to_string(),
//         iou: witness.iou,
//         nonce: witness.nonce.into_iter().collect::<Vec<u8>>(),
//         sig: witness.sig.into_iter().collect::<Vec<u8>>(),
//     })
// }
// 
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = CldbClient::connect("http://0.0.0.0:50051").await?;
    let prv_key = [0u8; 32]; // private key only for example !
    let (keypair, pub_key_arr) = ed25519::keypair(&prv_key);
    let pk = pub_key_arr.into_iter().collect::<Vec<u8>>();

    let name = "waalge".to_string();
    let req = tonic::Request::new(OpenReq {
        pk : pk.clone(), 
        name, 
    });

    let _ = client.open(req).await?;

    let src = "addr1xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string();
    let trg = pk.clone();
    let q = 999_999_999;
    let proof = vec![];
    let add_req = AddReq { src , trg, q , proof };

    let _ = client.add(tonic::Request::new(add_req.clone() )).await?;
    let _ = client.add(tonic::Request::new(add_req.clone() )).await?;
    let _ = client.add(tonic::Request::new(add_req.clone() )).await?;
    let _ = client.add(tonic::Request::new(add_req.clone() )).await?;
    let res = client.status(tonic::Request::new(StatusReq { pk } )).await?;

    println!("{:?}", res);
    // Subscriber id
    // let id = add_res.into_inner().id;

    // // Make a request
    // let request = mk_request(keypair, &id, 1);
    // let response = client.whats_the_time(request).await?;
    // let start = response.into_inner().message;
    // let start_time = DateTime::parse_from_str(&start, "New %+").unwrap();
    // println!("ID={} UTC={:?}", id, start);
    // for ii in 2..20000 {
    //     let request = mk_request(keypair, &id, ii);
    //     let response = client.whats_the_time(request).await?;
    //     let _utc = response.into_inner().message;
    // }
    // let request = mk_request(keypair, &id, 20000);
    // let response = client.whats_the_time(request).await?;
    // let end = response.into_inner().message;
    // println!("ID={} UTC={:?}", id, end);
    // let end_time = DateTime::parse_from_str(&end, "New %+").unwrap();
    // println!(
    //     "ID={} DIFF={:?}",
    //     id,
    //     end_time.signed_duration_since(start_time)
    // );
    Ok(())
}
