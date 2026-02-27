//! Criterion benchmarks for the DNS cache — TASK-46.

use criterion::{Criterion, black_box, criterion_group, criterion_main};

use hickory_proto::op::{Message, MessageType, OpCode, Query, ResponseCode};
use hickory_proto::rr::{DNSClass, Name, RData, Record, RecordType};
use hickory_proto::rr::rdata::A;
use hickory_proto::serialize::binary::BinEncodable;

use agh_dns::cache::DnsCache;

// ── Helpers ───────────────────────────────────────────────────────────────────

fn make_a_response(name: &str, ip: std::net::Ipv4Addr) -> Message {
    let mut msg = Message::new();
    msg.set_id(1);
    msg.set_message_type(MessageType::Response);
    msg.set_response_code(ResponseCode::NoError);
    let parsed_name = Name::from_ascii(name).unwrap();
    let rdata = RData::A(A(ip));
    let mut record = Record::new();
    record.set_name(parsed_name.clone());
    record.set_record_type(RecordType::A);
    record.set_dns_class(DNSClass::IN);
    record.set_ttl(300);
    record.set_data(Some(rdata));
    msg.add_answer(record);
    msg
}

fn make_query(name: &str, qtype: RecordType) -> Message {
    let mut msg = Message::new();
    msg.set_id(42);
    msg.set_op_code(OpCode::Query);
    msg.set_message_type(MessageType::Query);
    let parsed = Name::from_ascii(name).unwrap();
    let mut q = Query::new();
    q.set_name(parsed);
    q.set_query_type(qtype);
    q.set_query_class(DNSClass::IN);
    msg.add_query(q);
    msg
}

// ── Benchmarks ────────────────────────────────────────────────────────────────

fn bench_cache_insert(c: &mut Criterion) {
    c.bench_function("cache_insert_1k", |b| {
        let cache = DnsCache::new(10_000);
        let mut i = 0u32;
        b.iter(|| {
            let ip = std::net::Ipv4Addr::from(i);
            let name = format!("host{i}.bench.test.");
            let query = make_query(&name, RecordType::A);
            let response = make_a_response(&name, ip);
            cache.insert(black_box(&query), black_box(response));
            i = i.wrapping_add(1);
        });
    });
}

fn bench_cache_hit(c: &mut Criterion) {
    let cache = DnsCache::new(10_000);
    // Warm up: insert 1000 entries.
    for i in 0u32..1000 {
        let name = format!("hit{i}.bench.test.");
        let query = make_query(&name, RecordType::A);
        let response = make_a_response(&name, std::net::Ipv4Addr::from(i));
        cache.insert(&query, response);
    }
    let hit_query = make_query("hit500.bench.test.", RecordType::A);

    c.bench_function("cache_lookup_hit", |b| {
        b.iter(|| cache.get(black_box(&hit_query)))
    });
}

fn bench_cache_miss(c: &mut Criterion) {
    let cache = DnsCache::new(10_000);
    let miss_query = make_query("not-in-cache.bench.test.", RecordType::A);

    c.bench_function("cache_lookup_miss", |b| {
        b.iter(|| cache.get(black_box(&miss_query)))
    });
}

criterion_group!(benches, bench_cache_insert, bench_cache_hit, bench_cache_miss);
criterion_main!(benches);
