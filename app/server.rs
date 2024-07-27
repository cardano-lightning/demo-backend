use std::sync::{Mutex};

use tonic::{transport::Server, Code, Request, Response, Status};

use cldb_tonic::cldb_server::{Cldb,  CldbServer};
use cldb_tonic::{
    OpenReq,
        AddReq, Empty, StatusReq, StatusRes,
};

pub mod cldb_tonic {
    tonic::include_proto!("cldb"); // The string specified here must match the proto package name
}

use cldb::db::{Db, DbError};
use cldb::types::{PubKey, };

pub struct Ctx {
    pub db: Mutex<Db>,
}

type MyResult<T> = Result<Response<T>, Status>;

fn store_error_status(err: sqlite::Error) -> Status {
    Status::new(Code::Unknown, format!("{:?}", err))
}

#[tonic::async_trait]
impl Cldb for Ctx {
    async fn open(&self, req: Request<OpenReq>) -> MyResult<Empty> {
        let OpenReq { pk, name } = req.into_inner();
        let pk = PubKey::try_from(pk)
            .map_err(|_| Status::new(Code::InvalidArgument, "Cannot parse pub key"))?;
        let db = self.db.lock().unwrap();
        let _err = db.add_account(&pk, name.as_str()).unwrap();
        Ok(Response::new( Empty {} ) )
    }
    
    async fn add(&self, req: Request<AddReq>) -> MyResult<Empty> {
        let AddReq { src, trg, q, proof } = req.into_inner();
        let trg = PubKey::try_from(trg)
            .map_err(|_| Status::new(Code::InvalidArgument, "Cannot parse pub key"))?;
        let db = self.db.lock().unwrap();
        let _ = db.add_add(&src,&trg, q, proof).unwrap();
        Ok(Response::new( Empty { }))
    }
    
    async fn status(&self, req: Request<StatusReq>) -> MyResult<StatusRes> {
        let StatusReq { pk } = req.into_inner();
        let pk = PubKey::try_from(pk).unwrap();
        let db = self.db.lock().unwrap();
        let acc = db.get_balance(&pk).unwrap();
        Ok(Response::new( StatusRes { q: acc }))

    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse().unwrap();

    println!("cldb listening on {}", addr);

    let ctx = Ctx { db : Mutex::new(Db::new_mem()) };

    Server::builder()
        .add_service(CldbServer::new(ctx))
        .serve(addr)
        .await?;

    Ok(())
}
