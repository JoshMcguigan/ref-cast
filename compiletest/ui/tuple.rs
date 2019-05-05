use ref_cast::RefCast;

#[derive(RefCast)]
struct Test {
    s: String,
    t: (u8, u8),
}

fn main() {}
