enum IpAddressKind {
    // Each variant can have different types and amounts of associated data
    // Attach data to each variant of the enum directly; no need for an extra struct
    V4(u8, u8, u8, u8),
    V6(String),
}

// With the enum set up like this, we can avoid this:
//struct IpAddressV4 {
    //address: (u8, u8, u8, u8)
//}

//struct IpAddressV6 {
    //address: String
//}

fn main() {
    let ip4 = IpAddressKind::V4(127, 0, 0, 1);
    let ip6 = IpAddressKind::V6(String::from("::11"));
}
