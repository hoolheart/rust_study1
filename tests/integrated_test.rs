use rust_study1::bootes::def;

#[test]
fn test_ipaddr() {
    let addr0 = def::IpAddr::V4(127, 0, 0, 1);
    assert_eq!(addr0.version(), 4);

    let addr1 = def::IpAddr::V6(0xaabb, 0x7788, 0, 0, 0, 0, 0, 1);
    assert_eq!(addr1.to_string(), String::from("AABB:7788:0000:0000:0000:0000:0000:0001"));
}