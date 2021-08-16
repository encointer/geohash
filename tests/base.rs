extern crate geohash;
extern crate alloc;
use fixed::types::I64F64;
use geohash::{decode, encode, neighbors, GeoHash};

use alloc::string::String;

#[test]
fn test_encode() {
    let lon = I64F64::from_num(112.5584);
    let lat = I64F64::from_num(37.8324f64);
    assert_eq!(encode(lat, lon, 9usize).unwrap(), GeoHash("ww8p1r4t8".as_bytes().to_vec()));

    let lon = I64F64::from_num(117);
    let lat = I64F64::from_num(32);
    assert_eq!(encode(lat, lon, 3usize).unwrap(), GeoHash("wte".as_bytes().to_vec()));

    let lon = I64F64::from_num(190);
    let lat = I64F64::from_num(-100);
    assert!(encode(lat, lon, 3usize).is_err());
}

fn compare_within(a: I64F64, b: I64F64, diff: I64F64) {
    assert!(
        (a - b).abs() < diff,
        format!("{:?} and {:?} should be within {:?}", a, b, diff)
    );
}

fn compare_decode(gh: GeoHash, exp_lon: I64F64, exp_lat: I64F64, exp_lon_err: I64F64, exp_lat_err: I64F64) {
    let (lon, lat, lon_err, lat_err) = decode(&gh).unwrap();
    let diff = I64F64::from_num(1e-5);
    compare_within(lon_err, exp_lon_err, diff);
    compare_within(lat_err, exp_lat_err, diff);
    compare_within(lon, exp_lon, diff);
    compare_within(lat, exp_lat, diff);
}

#[test]
fn test_decode() {
    compare_decode(GeoHash("ww8p1r4t8".as_bytes().to_vec()), I64F64::from_num(112.558386), I64F64::from_num(37.832386), I64F64::from_num(0.000021457), I64F64::from_num(0.000021457));
    compare_decode(GeoHash("9g3q".as_bytes().to_vec()), I64F64::from_num(-99.31640625), I64F64::from_num(19.423828125), I64F64::from_num(0.175781250), I64F64::from_num(0.087890625));

    assert!(decode(&GeoHash("abcd".as_bytes().to_vec())).is_err());
}

#[test]
fn test_neighbor() {
    let ns = neighbors(&GeoHash("ww8p1r4t8".as_bytes().to_vec())).unwrap();
    assert_eq!(ns.sw, GeoHash("ww8p1r4mr".as_bytes().to_vec()));
    assert_eq!(ns.s, GeoHash("ww8p1r4t2".as_bytes().to_vec()));
    assert_eq!(ns.se, GeoHash("ww8p1r4t3".as_bytes().to_vec()));
    assert_eq!(ns.w, GeoHash("ww8p1r4mx".as_bytes().to_vec()));
    assert_eq!(ns.e, GeoHash("ww8p1r4t9".as_bytes().to_vec()));
    assert_eq!(ns.nw, GeoHash("ww8p1r4mz".as_bytes().to_vec()));
    assert_eq!(ns.n, GeoHash("ww8p1r4tb".as_bytes().to_vec()));
    assert_eq!(ns.ne, GeoHash("ww8p1r4tc".as_bytes().to_vec()));
}

#[test]
fn test_neighbor_wide() {
    let ns = neighbors(&GeoHash("9g3m".as_bytes().to_vec())).unwrap();
    assert_eq!(ns.sw, GeoHash("9g3h".as_bytes().to_vec()));
    assert_eq!(ns.s, GeoHash("9g3k".as_bytes().to_vec()));
    assert_eq!(ns.se, GeoHash("9g3s".as_bytes().to_vec()));
    assert_eq!(ns.w, GeoHash("9g3j".as_bytes().to_vec()));
    assert_eq!(ns.e, GeoHash("9g3t".as_bytes().to_vec()));
    assert_eq!(ns.nw, GeoHash("9g3n".as_bytes().to_vec()));
    assert_eq!(ns.n, GeoHash("9g3q".as_bytes().to_vec()));
    assert_eq!(ns.ne, GeoHash("9g3w".as_bytes().to_vec()));
}
