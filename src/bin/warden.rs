#[cfg(not(target_os = "freebsd"))]
fn main() {
    println!("Only supported on FreeBSD")
}

#[cfg(target_os = "freebsd")]
use {
    nix::sys::utsname::uname,
    jail::RunningJail
};

#[cfg(target_os = "freebsd")]
fn main() {
    let sys = uname();
    
    for jail in RunningJail::all() {
        let rel = jail.param("osrelease").expect("Could not get osrelease")
            .unpack_string().expect("osrelease is not a string");

        println!("jail: {} - {}", jail.name().unwrap(), rel);
        if sys.release() != rel {
            println!(" - OUT OF DATE!");
        }
    }
}