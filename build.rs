fn main() {
    slint_build::compile("ui/confirm-dialog.slint").expect("Slint build failed");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=ui/confirm-dialog.slint");
}
