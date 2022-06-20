use libproxy::ProxyFactory;

// On macOS: brew install libproxy
// On win32: ?
// On Linux: ?

fn main() {
    let url = "https://developer.nordicsemi.com/.pc-tools/nrfutil-test/x64-windows";
    let factory = ProxyFactory::new().unwrap();

    println!("Proxies to try for {:?}:", url);
    for proxy in &factory.get_proxies(url).unwrap() {
        println!(" * {:?}", proxy);
    }
}
