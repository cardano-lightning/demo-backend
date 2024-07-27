pub type PubKey = [u8; 32];

// Mainly for the record
// ub type PrvKey = [u8; 32];
// ub type Nonce = [u8; 16];
// ub type Sig = [u8; 64];
// 
// [derive(Debug, )]
// ub struct Act {
//    pub at: usize,
//    pub channel_id : ChannelId,
//    pub act_type : ActType,
// 
// 
// [derive(Debug, )]
// ub enum ActType { 
//    Open(OpenP),
//    Add(AddP),
// 
// 
// [derive(Debug, )]
// ub struct OpenP {
//    pub pka : PubKey, 
//    pub pkb : PubKey, 
//    pub q: usize
// 
// 
// [derive(Debug, )]
// ub struct AddP {
//    pub is_a: bool,
//    pub q: usize,
// 
// 
// ub struct Witness {
//    pub iou: u64,
//    pub nonce: Nonce,
//    pub sig: Sig,
// 
// 
// mpl Witness {
//    pub fn from_proto(iou: &u64, nonce: &Vec<u8>, sig: &Vec<u8>) -> Self {
//        let iou = iou.clone();
//        let nonce = nonce.clone().try_into().unwrap();
//        let sig = sig.clone().try_into().unwrap();
//        Self { iou, nonce, sig }
//    }
// 
// 
// ub fn mk_message(iou: u64, nonce: &Nonce) -> Vec<u8> {
//    nonce
//        .clone()
//        .into_iter()
//        .chain(iou.to_be_bytes().into_iter())
//        .collect::<Vec<u8>>()
// 
