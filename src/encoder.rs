use rkdb::{kbindings::*, types::*};
use schema_registry_converter::schema_registry::SubjectNameStrategy;
use schema_registry_converter::Encoder;
use avro_rs::types::Value;


#[no_mangle]
pub extern "C" fn encode_table(tbl: *const K, rows: *const K, colnames: *const K) -> Vec<Vec<u8>> {
    let mut result : Vec<Vec<u8>> = Vec::new();
    let mut nr = 0;
    let mut cnames:Vec<String> = Vec::new();
    match KVal::new(rows) {
        KVal::Int(KData::Atom(r)) => nr=*r,
        _ => println!("Invalid rows")
    };
    if let KVal::Mixed(v) = KVal::new(colnames) {
        for i in v.iter(){
            if let KVal::String(s) = i {
                cnames.push(s.parse().unwrap());
            }
        }
    }
    match KVal::new(tbl) {
        KVal::Table(box KVal::Dict(_, box KVal::Mixed(cols))) => {
            let mut records : Vec<Vec<(&'static str, Value)>> = Vec::new();
            for i in 0..nr{
                let mut record : Vec<(&'static str, Value)> = Vec::new();
                for (index, col) in cols.iter().enumerate(){
                    let key = Box::leak(cnames[index].clone().into_boxed_str());
                    match col {
                        KVal::Int(KData::List(ic)) => record.push((key, Value::Int((ic)[i as usize]))),
                        KVal::Float(KData::List(pc)) => record.push((key, Value::Double((pc)[i as usize]))),
                        KVal::Long(KData::List(sc)) => record.push((key, Value::Long((sc)[i as usize]))),
                        KVal::Bool(KData::List(bc)) => record.push((key, Value::Boolean((bc)[i as usize]))),
                        KVal::Mixed(symbols) => {
                            match symbols[i as usize] {
                                KVal::String(syf) =>  record.push((key, Value::String(syf.parse().unwrap()))),
                                _ => println!("No string symbol")
                            }
                        },
                        _ => println!("Unrecognized Col")
                    }
                }
                records.push(record);
            }
            result = encode_trades_with_schema_registry(&records);
        },
        _ => println!("No Table")
    };
    result
}


pub(crate) fn encode_trades_with_schema_registry(records: &Vec<Vec<(&'static str, Value)>>) -> Vec<Vec<u8>>  {
    let value_strategy = SubjectNameStrategy::TopicNameStrategy("trade".into(), false);
    let mut encoder = Encoder::new("localhost:8081".to_string());
    let mut bytes : Vec<Vec<u8>> = Vec::new();
    for record in records.iter(){
        let encoded = encoder.encode(record.to_vec(), &value_strategy);
        bytes.push(encoded.unwrap());
    }
    bytes
}