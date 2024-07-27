use std::time::{SystemTime, UNIX_EPOCH};
use sqlite;

use crate::types::PubKey;  

static SCHEMA_000: &'static str = include_str!("../db/000.sql");

pub fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

pub fn mk_mem_conn() -> sqlite::Connection {
    mk_conn(":memory:")
}

pub fn mk_conn(location : &str) -> sqlite::Connection {
    let conn = sqlite::open(location).unwrap();
    conn.execute(SCHEMA_000).unwrap();
    conn
}

pub struct Db {
    conn : sqlite::Connection,
}

// pub fn concat_index(a: &[u8; 32], b: &[u8; 32]) -> [u8; 64] {
//     std::array::from_fn(|i| {
//         if let Some(i) = i.checked_sub(a.len()) {
//             b[i]
//         } else {
//             a[i]
//         }
//     })
// }

// fn mk_channel_id( pka : &PubKey, pkb : &PubKey) -> ChannelId {
//     let mut digest = [0u8; 28];
//     let mut b = blake2b::Blake2b::new(28);
//     b.input(&concat_index(pka, pkb));
//     b.result(&mut digest);
//     digest
// } 
// 

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DbError {
    ReadError
}

#[derive(Debug, PartialEq, Eq)]
pub struct Account {
    pub at : u64,
    pub pk : PubKey,
    pub name: String,
}

impl Account {
    pub fn new(pk : &PubKey, name : &str) -> Self {
        Account { pk : pk.clone(), name : name.to_string(), at : now() }
    }

    pub fn insert(self, conn : &sqlite::Connection) -> Result<(), sqlite::Error> {
        let mut prep = conn.prepare("INSERT INTO accounts VALUES (:pk, :name, :at)")? ;
        prep.reset()?;
        prep.bind((":pk", &self.pk[..]))?;
        prep.bind((":name", &self.name[..]))?;
        prep.bind((":at", self.at.to_string().as_str()))?;
        prep.next()?;
        Ok(())
    }

    pub fn from_row(row : sqlite::Row ) -> Result<Self, sqlite::Error> {
        let pk = PubKey::try_from(row.read::<&[u8], _>("pk")).unwrap();
        let name = row.read::<&str, _>("name").to_string();
        let at =  row.read::<&str, _>("at").parse().unwrap();
        Ok(Self { pk, name, at } )
    }

    pub fn from_prep(prep : &mut sqlite::Statement ) -> Result<Self, sqlite::Error> {
        let pk = PubKey::try_from(prep.read::<Vec<u8>, _>("pk")?).unwrap();
        let name = prep.read::<String, _>("name")?;
        let at =  prep.read::<String, _>("at")?.parse().unwrap();
        Ok(Self { pk, name, at } )
    }

    pub fn get_by_pk( conn : &sqlite::Connection, pk : &PubKey) -> Result<Self, sqlite::Error> {
        let mut prep = conn.prepare("SELECT * FROM accounts where pk = :pk ")?;
        prep.reset()?;
        prep.bind((":pk", &pk[..])).unwrap();
        prep.next()?;
        Self::from_prep(&mut prep)
    }

    pub fn get_by_name( conn : &sqlite::Connection, name : &str) -> Result<Self, sqlite::Error> {
        let mut prep = conn.prepare("SELECT * FROM accounts where name = :name ")?;
        prep.reset()?;
        prep.bind((":name", name)).unwrap();
        prep.next()?;
        Self::from_prep(&mut prep)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Add {
    pub id : uuid::Uuid,
    pub src : String,
    pub trg : PubKey,
    pub q : u64,
    pub proof: Vec<u8>,
    pub at: u64,
}

impl Add {
    pub fn new(src: &str, trg : &PubKey, q : u64, proof : Vec<u8> ,) -> Self {
        Add { 
            id : uuid::Uuid::new_v4(), 
            src : src.to_string(),
            trg : trg.clone(), 
            q,
            proof , 
            at : now() 
        }
    }

    pub fn insert(self, conn : &sqlite::Connection) -> Result<(), sqlite::Error> {
        let mut prep = conn.prepare("INSERT INTO adds VALUES (:id, :src, :trg, :q, :proof, :at)")? ;
        prep.reset()?;
        prep.bind((":id", self.id.to_string().as_str()))?;
        prep.bind((":src", self.src.as_str()))?;
        prep.bind((":trg", &self.trg[..]))?;
        prep.bind((":q", self.q.to_string().as_str()))?;
        prep.bind((":proof", &self.proof[..]))?;
        prep.bind((":at", self.at.to_string().as_str()))?;
        prep.next()?;
        Ok(())
    }

    pub fn from_row(row : sqlite::Row ) -> Result<Self, sqlite::Error> {
        let id = uuid::Uuid::parse_str( &row.read::<&str, _>("id").to_string() ).unwrap();
        let src = row.read::<&str, _>("src").to_string();
        let trg = PubKey::try_from(row.read::<&[u8], _>("trg")).unwrap();
        let q =  row.read::<&str, _>("q").parse().unwrap();
        let proof = row.read::<&[u8], _>("proof").to_vec();
        let at =  row.read::<&str, _>("at").parse().unwrap();
        Ok(Self { id, src, trg, q, proof, at } )
    }

    pub fn from_prep(prep : &mut sqlite::Statement ) -> Result<Self, sqlite::Error> {
        let id = uuid::Uuid::parse_str( &prep.read::<String, _>("id")?).unwrap();
        let src = prep.read::<String, _>("src")?;
        let trg = PubKey::try_from(prep.read::<Vec<u8>, _>("trg")?).unwrap();
        let q =  prep.read::<String, _>("q")?.parse().unwrap();
        let proof = prep.read::<Vec<u8>, _>("proof")?;
        let at =  prep.read::<String, _>("at")?.parse().unwrap();
        Ok(Self { id, src, trg, q, proof, at } )
    }

}



impl Db {

    // NEW 
    pub fn new(path : &str) -> Self {
        Self { conn : mk_conn(path) }
    }

    pub fn new_mem() -> Self {
        Self { conn : mk_mem_conn() }
    }

    // ACCOUNT 
    pub fn add_account(&self, pk : &PubKey, name : &str)-> Result<(), sqlite::Error>{
        Account::new(pk, name).insert(&self.conn)
    } 

    pub fn get_accounts(&self) -> Result<Vec<Account>, sqlite::Error> {
        let mut prep = self.conn.prepare("SELECT * FROM accounts")?;
        prep.reset()?;
        prep.into_iter().map(move |row| Account::from_row(row?)).collect()
    }

    pub fn get_account_by_pk(&self, pk : &PubKey) -> Result<Account, sqlite::Error> {
        Account::get_by_pk(&self.conn, pk)
    }

    pub fn get_account_by_name(&self, name : &str) -> Result<Account, sqlite::Error> {
        Account::get_by_name(&self.conn, name)
    }

    // ADDS 
    pub fn add_add(&self, src : &str, trg : &PubKey, q : u64, proof : Vec<u8> )-> Result<(), sqlite::Error>{
        Add::new(src, trg, q, proof).insert(&self.conn)
    } 

    pub fn get_adds_by_trg(&self, trg : &PubKey) -> Result<Vec<Add>, sqlite::Error> {
        let mut prep = self.conn.prepare("SELECT * FROM adds where trg = :trg")?;
        prep.reset()?;
        prep.bind((":trg", &trg[..]))?;
        prep.into_iter().map(move |row| Add::from_row(row?)).collect()
    }

    pub fn get_balance(&self, pk : &PubKey) -> Result<u64, sqlite::Error> {
        let adds = self.get_adds_by_trg(pk)?.into_iter().fold(0, |acc, c| {c.q + acc});
        Ok(adds)
    }


}

#[test] 
fn test_account() {
    let db = Db::new_mem();
    let pk : PubKey = [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1];
    let name = "waalge";
    let _ = db.add_account(&pk, &name);
    let acc_pk = db.get_account_by_pk(&pk).unwrap();
    let acc_name = db.get_account_by_pk(&pk).unwrap();
    assert_eq!(acc_pk.pk, pk);
    assert_eq!(acc_pk.name, name) ;
    assert_eq!(acc_pk, acc_name) ;
}

#[test] 
fn test_unique_pk() {
    let db = Db::new_mem();
    let pk : PubKey = [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1];
    let name_0 = "waalge";
    let name_1 = "Waslje";
    let _ = db.add_account(&pk, &name_0);
    let oops = db.add_account(&pk, &name_1);
    match oops {
        Ok(_) => panic!("Bad"),
        Err(_e) => {}
    }
}

#[test] 
fn test_unique_name() {
    let db = Db::new_mem();
    let pk_0 : PubKey = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    let pk_1 : PubKey = [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1];
    let name = "waalge";
    let _ = db.add_account(&pk_0, &name);
    let oops = db.add_account(&pk_1, &name);
    match oops {
        Ok(_) => panic!("Bad"),
        Err(_e) => {}
    }
}

#[test] 
fn test_accounts() {
    let db = Db::new_mem();
    let pk_0 : PubKey = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    let name_0 = "waalge";
    let pk_1 : PubKey = [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1];
    let name_1 = "Waslje";
    let pk_2 : PubKey = [2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2];
    let name_2 = "a";
    let pk_3 : PubKey = [3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3];
    let name_3 = "a3";
    let _ = db.add_account(&pk_0, &name_0).unwrap();
    let _ = db.add_account(&pk_1, &name_1).unwrap();
    let _ = db.add_account(&pk_2, &name_2).unwrap();
    let _ = db.add_account(&pk_3, &name_3).unwrap();
    let accounts = db.get_accounts().unwrap();
    eprintln!("{:?}", accounts);
    assert_eq!(accounts.len(), 4)  
}

#[test] 
fn test_adds() {
    let db = Db::new_mem();
    let pk_0 : PubKey = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    let name_0 = "waalge";
    let pk_1 : PubKey = [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1];
    let name_1 = "Waslje";
    let pk_2 : PubKey = [2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2];
    let name_2 = "a";
    let pk_3 : PubKey = [3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3];
    let name_3 = "a3";
    let _ = db.add_account(&pk_0, &name_0).unwrap();
    let _ = db.add_account(&pk_1, &name_1).unwrap();
    let _ = db.add_account(&pk_2, &name_2).unwrap();
    let _ = db.add_account(&pk_3, &name_3).unwrap();
    let src = "addr1xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";
    let trg = pk_3;
    let q = 999_999_999;
    let proof = vec![];
    let _ = db.add_add(src, &trg, q, proof.clone()).unwrap();
    let _ = db.add_add(src, &trg, q, proof.clone()).unwrap();
    let _ = db.add_add(src, &trg, q, proof.clone()).unwrap();
    let _ = db.add_add(src, &trg, q, proof.clone()).unwrap();
    let adds = db.get_adds_by_trg(&trg).unwrap();
    assert_eq!(adds.len(), 4);
    let balance = db.get_balance(&trg).unwrap();
    assert_eq!(4 * q, balance);

}

