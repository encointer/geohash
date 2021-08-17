extern crate geohash;
extern crate alloc;

use fixed::types::I64F64;
use geohash::GeoHash;

#[test]
fn test_encode() {
    let lon = I64F64::from_num(112.5584);
    let lat = I64F64::from_num(37.8324f64);
    assert_eq!(GeoHash::try_from_params(lat, lon, 9usize).unwrap(), GeoHash::from("ww8p1r4t8"));

    let lon = I64F64::from_num(117);
    let lat = I64F64::from_num(32);
    assert_eq!(GeoHash::try_from_params(lat, lon, 3usize).unwrap(), GeoHash::from("wte"));

    let lon = I64F64::from_num(190);
    let lat = I64F64::from_num(-100);
    assert!(GeoHash::try_from_params(lat, lon, 3usize).is_err());
}

fn compare_within(a: I64F64, b: I64F64, diff: I64F64) {
    assert!(
        (a - b).abs() < diff,
        format!("{:?} and {:?} should be within {:?}", a, b, diff)
    );
}

fn compare_decode(gh: GeoHash, exp_lon: I64F64, exp_lat: I64F64, exp_lon_err: I64F64, exp_lat_err: I64F64) {
    let (lon, lat, lon_err, lat_err) = gh.try_as_coordinates().unwrap();
    let diff = I64F64::from_num(1e-5);
    compare_within(lon_err, exp_lon_err, diff);
    compare_within(lat_err, exp_lat_err, diff);
    compare_within(lon, exp_lon, diff);
    compare_within(lat, exp_lat, diff);
}

#[test]
fn test_decode() {
    compare_decode(GeoHash::from("ww8p1r4t8"), I64F64::from_num(112.558386), I64F64::from_num(37.832386), I64F64::from_num(0.000021457), I64F64::from_num(0.000021457));
    compare_decode(GeoHash::from("9g3q"), I64F64::from_num(-99.31640625), I64F64::from_num(19.423828125), I64F64::from_num(0.175781250), I64F64::from_num(0.087890625));

    assert!(GeoHash::from("abcd").try_as_coordinates().is_err());
}

#[test]
fn test_neighbor() {
    let ns = &GeoHash::from("ww8p1r4t8").neighbors().unwrap();
    assert_eq!(ns.sw, GeoHash::from("ww8p1r4mr"));
    assert_eq!(ns.s, GeoHash::from("ww8p1r4t2"));
    assert_eq!(ns.se, GeoHash::from("ww8p1r4t3"));
    assert_eq!(ns.w, GeoHash::from("ww8p1r4mx"));
    assert_eq!(ns.e, GeoHash::from("ww8p1r4t9"));
    assert_eq!(ns.nw, GeoHash::from("ww8p1r4mz"));
    assert_eq!(ns.n, GeoHash::from("ww8p1r4tb"));
    assert_eq!(ns.ne, GeoHash::from("ww8p1r4tc"));
}

#[test]
fn test_neighbor_wide() {
    let ns = &GeoHash::from("9g3m").neighbors().unwrap();
    assert_eq!(ns.sw, GeoHash::from("9g3h"));
    assert_eq!(ns.s, GeoHash::from("9g3k"));
    assert_eq!(ns.se, GeoHash::from("9g3s"));
    assert_eq!(ns.w, GeoHash::from("9g3j"));
    assert_eq!(ns.e, GeoHash::from("9g3t"));
    assert_eq!(ns.nw, GeoHash::from("9g3n"));
    assert_eq!(ns.n, GeoHash::from("9g3q"));
    assert_eq!(ns.ne, GeoHash::from("9g3w"));
}
