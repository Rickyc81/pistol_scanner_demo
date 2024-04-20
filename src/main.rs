use subnetwork::Ipv4Pool;
use::pistol::arp_scan_subnet;

fn main() {
    let ipv4_pool = Ipv4Pool::from("192.168.1.1/24").unwrap();
    let result_scan = arp_scan_subnet(ipv4_pool, None, None, 0, true, Some(2) ).unwrap();

    println!("{:?}", result_scan);
}
