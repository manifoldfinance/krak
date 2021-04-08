use std::env;

pub fn get_schema_registry() -> String {
    let schema_reg_host = env::var("SCHEMA_REG_HOST").expect("schema reg host not defined");
    let schema_reg_port = env::var("SCHEMA_REG_PORT").expect("schema reg port not defined");
    let conn = format!("{}:{}", schema_reg_host, schema_reg_port);
    println!("Using Schema Registry : {}", conn);
    conn
}
