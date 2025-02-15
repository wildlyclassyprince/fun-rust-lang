
fn main(){
    #[derive(Debug)]
    enum IpAddr {
        V4(Option<String>),
        V6(Option<String>),
    }

    let home = IpAddr::V4(Some(String::from("127.0.0.1")));
    let loopback = IpAddr::V6(Some(String::from("::1")));
    let unknown = IpAddr::V6(None);

    // TODO: fix warning: `0` is never read
    println!("Home IPv4 address: {home:#?}");
    println!("Loopback address: {loopback:#?}");

}