extern crate cc;

fn main() {
    let kernels = vec!["add_array"];

    for kernel in kernels.iter() {
        cc::Build::new()
            .cuda(true)
            .flag("-cudart=shared")
            .flag("--expt-extended-lambda")
            .flag("-gencode")
            .flag("arch=compute_86,code=sm_86")
            .file(format!("cuda/{kernel}.cu"))
            .compile(&format!("lib{kernel}.a"));
    }

    /* Link CUDA Runtime (libcudart.so) */

    // Add link directory
    // - This path depends on where you install CUDA (i.e. depends on your Linux distribution)
    // - This should be set by `$LIBRARY_PATH`
    println!("cargo:rustc-link-search=native=/usr/local/cuda/lib64");
    println!("cargo:rustc-link-lib=cudart");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=cuda");

    /* Optional: Link CUDA Driver API (libcuda.so) */

    // println!("cargo:rustc-link-search=native=/usr/local/cuda/lib64/stub");
    // println!("cargo:rustc-link-lib=cuda");
}