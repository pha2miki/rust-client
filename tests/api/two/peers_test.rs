use *;
use serde_json::{from_str, Value};

use arkecosystem_client::api::two::models::{Meta, Peer};

#[test]
fn test_all() {
    let (_mock, body) = mock_http_request_two("peers");
    {
        let client = mock_client_two();
        let actual = client.peers.all().unwrap();
        let expected: Value = from_str(&body).unwrap();

        let actual_meta = actual.meta.unwrap();
        let expected_meta = expected["meta"].clone();
        assert_peer_meta(actual_meta, expected_meta);

        let actual_data = actual.data[0].clone();
        let expected_data = expected["data"][0].clone();
        assert_peer_data(actual_data, expected_data);
    }
}

#[test]
fn test_all_params() {
    // TODO use a different fixture to check that uses query strings
    let (_mock, body) = mock_http_request_two("peers");
    {
        let client = mock_client_two();
        let params = [("limit", "20")].iter();
        let actual = client.peers.all_params(params).unwrap();
        let expected: Value = from_str(&body).unwrap();

        let actual_meta = actual.meta.unwrap();
        let expected_meta = expected["meta"].clone();
        assert_peer_meta(actual_meta, expected_meta);

        let actual_data = actual.data[0].clone();
        let expected_data = expected["data"][0].clone();
        assert_peer_data(actual_data, expected_data);
    }
}

#[test]
fn test_show() {
    let (_mock, body) = mock_http_request_two("peers/dummy");
    {
        let client = mock_client_two();
        let actual = client.peers.show("dummy").unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_peer_data(actual.data, expected["data"].clone());
    }
}

fn assert_peer_meta(actual: Meta, expected: Value) {
    assert_eq!(
        actual.count,
        expected["count"].as_u64().unwrap() as u32
    );
    assert_eq!(
        actual.page_count,
        expected["pageCount"].as_u64().unwrap() as u32
    );
    assert_eq!(
        actual.total_count,
        expected["totalCount"].as_u64().unwrap() as u32
    );
    assert_eq!(
        actual.next.unwrap(),
        expected["next"].as_str().unwrap()
    );
    assert_eq!(
        actual.previous.unwrap(),
        expected["previous"].as_str().unwrap()
    );
    assert_eq!(
        actual.self_url,
        expected["self"].as_str().unwrap()
    );
    assert_eq!(
        actual.first,
        expected["first"].as_str().unwrap()
    );
    assert_eq!(
        actual.last.unwrap(),
        expected["last"].as_str().unwrap()
    );
}

fn assert_peer_data(actual: Peer, expected: Value) {
    assert_eq!(
        actual.ip,
        expected["ip"].as_str().unwrap()
    );
    assert_eq!(
        actual.port,
        expected["port"].as_u64().unwrap() as u16
    );
    assert_eq!(
        actual.version,
        expected["version"].as_str().unwrap()
    );
    assert_eq!(
        actual.height,
        expected["height"].as_u64().unwrap()
    );
    assert_eq!(
        actual.status,
        expected["status"].as_u64().unwrap() as u16
    );
    assert_eq!(
        actual.os,
        expected["os"].as_str().unwrap()
    );
    assert_eq!(
        actual.latency,
        expected["latency"].as_u64().unwrap() as u32
    );
    assert_eq!(
        actual.hashid,
        expected["hashid"].as_str().unwrap()
    );
}
