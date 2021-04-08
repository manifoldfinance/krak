use crate::util::*;
use avro_rs::types::Value;
use lazy_static::lazy_static;
use rkdb::kbindings::{kdict, kvoid, KData, KVal};
use rkdb::types::K;
use schema_registry_converter::Decoder;
use std::sync::Mutex;

lazy_static! {
    static ref DECODER: Mutex<Decoder> =
        Mutex::new(Decoder::new(get_schema_registry().to_string()));
}

#[no_mangle]
pub extern "C" fn decode(enumasint: *const K, msg: *const K) -> *const K {
    let mut result = kvoid();
    let mut easint = false;
    if let KVal::Bool(KData::Atom(b)) = KVal::new(enumasint) {
        easint = b.to_owned();
    }
    if let KVal::Byte(KData::List(m)) = KVal::new(msg) {
        result = parse_msg(m, easint);
    } else {
        println!("MSG not a byte array")
    }
    result
}

fn parse_msg(data: &[u8], enumasint: bool) -> *const K {
    let mut result = kvoid();
    let payload = DECODER.lock().unwrap().decode(Some(data)).unwrap();
    match payload {
        Value::Record(mut v) => {
            let mut keys: Vec<String> = Vec::new();
            let mut values: Vec<KVal> = Vec::new();
            for (k, v) in v.iter_mut() {
                println!("Key    = {:?}", k);
                println!("Value  = {:?}", v);
                keys.push(k.parse().unwrap());
                match v {
                    Value::Array(arr) => {
                        let mut rows: Vec<KVal> = Vec::new();
                        for a in arr.into_iter() {
                            rows.push(parse_msgtype(a, enumasint));
                        }
                        values.push(KVal::Mixed(rows));
                    }
                    _ => values.push(parse_msgtype(v, enumasint)),
                }
            }
            let kkeys = KVal::Symbol(KData::List(&mut keys));
            let kvals = KVal::Mixed(values);
            let kret = kdict(&kkeys, &kvals);
            result = kret;
        }
        _ => println!("Did not receive a record"),
    }
    result
}

fn parse_msgtype(val: &mut Value, enumasint: bool) -> KVal {
    match val {
        Value::Int(i) => KVal::Int(KData::Atom(i)),
        Value::Long(l) => KVal::Long(KData::Atom(l)),
        Value::Float(f) => KVal::Real(KData::Atom(f)),
        Value::Double(d) => KVal::Float(KData::Atom(d)),
        Value::Boolean(b) => KVal::Bool(KData::Atom(b)),
        Value::String(s) => KVal::String(&s[0..]),
        Value::Null => KVal::Unknown,
        Value::Enum(i, s) => {
            if enumasint {
                KVal::Int(KData::Atom(i))
            } else {
                KVal::Symbol(KData::Atom(s))
            }
        }
        Value::Union(box u) => parse_msgtype(u, enumasint),
        Value::Record(records) => {
            let mut keys: Vec<KVal> = Vec::new();
            let mut values: Vec<KVal> = Vec::new();
            for (key, val) in records.into_iter() {
                keys.push(KVal::Symbol(KData::Atom(key)));
                values.push(parse_msgtype(val, enumasint));
            }
            KVal::Dict(Box::new(KVal::Mixed(keys)), Box::new(KVal::Mixed(values)))
        }
        Value::Map(map) => {
            let mut keys: Vec<KVal> = Vec::new();
            let mut values: Vec<KVal> = Vec::new();
            for (key, val) in map.into_iter() {
                keys.push(KVal::String(key));
                values.push(parse_msgtype(val, enumasint));
            }
            KVal::Dict(Box::new(KVal::Mixed(keys)), Box::new(KVal::Mixed(values)))
        }
        _ => {
            println!("Unrecognized msg field received");
            KVal::Unknown
        }
    }
}
