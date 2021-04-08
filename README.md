# krak

Avro, kafka, schema_registry binding for kdb+ using Rust.

This rust lib is intended to be loaded inside running q process and used for publishing and consuming avro encoded messages from a kafka broker, it also relies on confluent schema regitry for parsing the messages, more information can be found here https://docs.confluent.io/current/schema-registry/index.html

This library utilies the existing kdb rust binding https://github.com/redsift/rkdb and various other rust libraries for integrating with AVRO.

## Building (use nightly channel)

```
$ rustup default nightly
$ cargo build --release && cp target/release/libkrak.so ${QHOME}/m64/libkrak.so
```

## Usage from q
Set up following envs

* export SCHEMA_REG_HOST=localhost
* export SCHEMA_REG_PORT=8081

## Consume messages from official Kx Kafka consumer

```
q)decode: `libkrak 2:(`decode;2)
q)\l kfk.q
q)client:.kfk.Consumer[`metadata.broker.list`group.id!`localhost:9092`0]
q)data:();
q).kfk.consumecb:{[msg] data,: enlist msg`data}
q).kfk.Sub[client;`trades;enlist .kfk.PARTITION_UA]
q)
q)decode[0b] each data
id sym  price    size
---------------------
77 "ci" 8.388858 12
30 "hk" 19.59907 10
17 "ae" 37.5638  1
23 "bl" 61.37452 90
12 "oo" 52.94808 73
66 "jg" 69.16099 90
36 "cf" 22.96615 43
37 "bp" 69.19531 90
44 "ic" 47.07883 84
28 "in" 63.46716 63
```

## Both encode and decode functions can be used standalone as demonstrated below
```
q)t:([]id:2?100i;sym:string 2?`2;price:2#9999f;size:2?100f)
q)
q)t
id sym  price size    
----------------------
17 "kl" 9999  36.52273
91 "ep" 9999  95.91177
q)
q)encode : `libkrak 2:(`encode;4)
q)decode: `libkrak 2:(`decode;2)
q)
q)x:encode["trade"; t; `int$count t; string cols t]
q)
q)x
0x000000000222046b6c000000008087c3400000b0dfe8424240
0x0000000002b601046570000000008087c34000007b6d5afa5740
q)
q)t ~ decode[1b] each x
1b
```
