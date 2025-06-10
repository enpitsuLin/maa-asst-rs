extern crate embed_resource;

fn main() {
    let rc_file = std::path::Path::new("res/windows/maa-manifest.rc");
    println!("cargo:rerun-if-changed={}", rc_file.display());
    embed_resource::compile(rc_file, embed_resource::NONE)
        .manifest_required()
        .unwrap();
}
