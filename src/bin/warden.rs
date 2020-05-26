use nix::sys::utsname::uname;

fn main() {
    println!("{}", uname().release());
}