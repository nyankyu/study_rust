#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    print_ip_type(&four);
    print_ip_type(&six);
}

fn print_ip_type(ip_type: &IpAddrKind) {
    println!("type of IP is {:?}", ip_type);
}
