#[cfg(not(target_os = "freebsd"))]
fn main() {
    println!("Only supported on FreeBSD")
}

#[cfg(target_os = "freebsd")]
fn main() {
    use nix::sys::utsname::uname;
    use jail::RunningJail;

    let version = uname().release();
    
    for jail in RunningJail::all() {
        let rel = jail.param("osrelease").expect("Could not get osrelease")
            .unpack_string().expect("osrelease is not a string");

        println!("jail: {} - {}", jail.name().unwrap(), rel);
        if version != rel {
            println!(" - OUT OF DATE!");
        }
    }
}