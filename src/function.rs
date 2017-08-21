// wip, this is a design to get things started

use uuid::Uuid;
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Function<IL> {
    pub name: Cow<'static,str>,
    // flanfly: these come in real handy at times, for example linking caller & callee in the IL
    uuid: Uuid,
    entry_point: u64,
    instructions: Vec<IL>, // in reverse postorder
    basic_blocks: Vec<BasicBlock>,
    mnemonics: Vec<Mnemonic>,
}

// impl Function {
//     fn entry(&self) -> u64;
//     // size in bytes
//     fn size(&self) -> usize;
//     fn instruction() -> Iterator<Item=&IL>;
//     fn mnenonics(&self) -> Iterator<Item=&Mnemonic>;
//     fn basic_blocks(&self) -> Iterator<Item=&BasicBlock>;
//     // we will probably use petgraph for this?
//     // Yes, not much alternatives anyway 
//     fn cflow_graph(&self) -> Graph<Node,Guard>
// }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Mnemonic {
    opcode: Cow<'static,str>, //'
    start: usize, // index into instructions
    end: usize,   // ditto
    first_byte: u64, // address
    last_byte: u64,  // ditto 
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BasicBlock {
    start: usize, // index into instructions^. u64, is this an address?
    end: usize,   // u64
    first_byte: u64,
    last_byte: u64,   
}
