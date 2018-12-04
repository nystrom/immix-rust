extern crate cc;

fn main() {
    cc::Build::new()
      .file("src/heap/gc/clib_x64.c")
      .compile("gc_clib_x64");
}
