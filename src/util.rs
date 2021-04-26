use schema_registry_converter::blocking::schema_registry::SrSettings;
use std::env;

fn schema_registry_address() -> String {
    let address = match env::var_os("SCHEMA_REGISTRY_URI") {
        Some(val) => val.into_string().expect("SCHEMA_REGISTRY_URI not defined"),
        None => "http://localhost:8081".to_owned(),
    };
    address
}

pub fn avro_sr_settings() -> SrSettings {
    let sr_address = schema_registry_address();
    println!(
        "Creating Avro Settings with Schema Registry URI -> {}",
        &sr_address
    );
    return SrSettings::new(sr_address);
}
