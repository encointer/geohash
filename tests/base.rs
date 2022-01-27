extern crate alloc;
extern crate geohash;

use fixed::types::I64F64;
use geohash::GeoHash;
use std::convert::TryFrom;

#[test]
fn test_encode() {
	let lon = I64F64::from_num(112.5584);
	let lat = I64F64::from_num(37.8324f64);
	assert_eq!(
		GeoHash::<9>::try_from_params(lat, lon).unwrap(),
		GeoHash::<9>::try_from("ww8p1r4t8").unwrap()
	);

	let lon = I64F64::from_num(117);
	let lat = I64F64::from_num(32);
	assert_eq!(
		GeoHash::<3>::try_from_params(lat, lon).unwrap(),
		GeoHash::<3>::try_from("wte").unwrap()
	);

	let lon = I64F64::from_num(190);
	let lat = I64F64::from_num(-100);
	assert!(GeoHash::<9>::try_from_params(lat, lon).is_err());
}

fn compare_within(a: I64F64, b: I64F64, diff: I64F64) {
	if !((a - b).abs() < diff) {
		panic!("{:?} and {:?} should be within {:?}", a, b, diff);
	}
}

fn compare_decode<const LEN: usize>(
	gh: GeoHash<LEN>,
	exp_lon: I64F64,
	exp_lat: I64F64,
	exp_lon_err: I64F64,
	exp_lat_err: I64F64,
) {
	let (lon, lat, lon_err, lat_err) = gh.try_as_coordinates().unwrap();
	let diff = I64F64::from_num(1e-5);
	compare_within(lon_err, exp_lon_err, diff);
	compare_within(lat_err, exp_lat_err, diff);
	compare_within(lon, exp_lon, diff);
	compare_within(lat, exp_lat, diff);
}

#[test]
fn test_decode() {
	compare_decode(
		GeoHash::<9>::try_from("ww8p1r4t8").unwrap(),
		I64F64::from_num(112.558386),
		I64F64::from_num(37.832386),
		I64F64::from_num(0.000021457),
		I64F64::from_num(0.000021457),
	);
	compare_decode(
		GeoHash::<4>::try_from("9g3q").unwrap(),
		I64F64::from_num(-99.31640625),
		I64F64::from_num(19.423828125),
		I64F64::from_num(0.175781250),
		I64F64::from_num(0.087890625),
	);

	assert!(GeoHash::<4>::try_from("abcd").is_err());
}

#[test]
fn test_neighbor() {
	type Geo9 = GeoHash<9>;
	let ns = Geo9::try_from("ww8p1r4t8").unwrap().neighbors().unwrap();
	assert_eq!(ns.sw, Geo9::try_from("ww8p1r4mr").unwrap());
	assert_eq!(ns.s, Geo9::try_from("ww8p1r4t2").unwrap());
	assert_eq!(ns.se, Geo9::try_from("ww8p1r4t3").unwrap());
	assert_eq!(ns.w, Geo9::try_from("ww8p1r4mx").unwrap());
	assert_eq!(ns.e, Geo9::try_from("ww8p1r4t9").unwrap());
	assert_eq!(ns.nw, Geo9::try_from("ww8p1r4mz").unwrap());
	assert_eq!(ns.n, Geo9::try_from("ww8p1r4tb").unwrap());
	assert_eq!(ns.ne, Geo9::try_from("ww8p1r4tc").unwrap());
}

#[test]
fn test_neighbor_wide() {
	type Geo4 = GeoHash<4>;
	let ns = Geo4::try_from("9g3m").unwrap().neighbors().unwrap();
	assert_eq!(ns.sw, Geo4::try_from("9g3h").unwrap());
	assert_eq!(ns.s, Geo4::try_from("9g3k").unwrap());
	assert_eq!(ns.se, Geo4::try_from("9g3s").unwrap());
	assert_eq!(ns.w, Geo4::try_from("9g3j").unwrap());
	assert_eq!(ns.e, Geo4::try_from("9g3t").unwrap());
	assert_eq!(ns.nw, Geo4::try_from("9g3n").unwrap());
	assert_eq!(ns.n, Geo4::try_from("9g3q").unwrap());
	assert_eq!(ns.ne, Geo4::try_from("9g3w").unwrap());
}
