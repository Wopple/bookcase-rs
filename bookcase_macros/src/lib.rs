use std::env::var;
use proc_macro::TokenStream;

/// Calling this will result in a compiler error if the release channel for the version of bookcase
/// does not match the respective enabled features.
#[proc_macro]
pub fn assert_release_channel(_: TokenStream) -> TokenStream {
    let version_str = var("CARGO_PKG_VERSION").unwrap();
    let mut parts = version_str.split(".");
    let stable = parts.next().unwrap();
    let beta = parts.next().unwrap();
    let experimental = parts.next().unwrap();

    if experimental != "0" {
        format!(
            "
            #[cfg(feature = \"stable\")]
            compile_error!(\"[bookcase] {0} is an experimental version, but stable feature was enabled.\");
            #[cfg(feature = \"beta\")]
            compile_error!(\"[bookcase] {0} is an experimental version, but beta feature was enabled.\");
            #[cfg(not(feature = \"experimental\"))]
            compile_error!(\"[bookcase] {0} is an experimental version, but experimental feature was disabled.\");
            ",
            version_str,
        )
    } else if beta != "0" {
        format!(
            "
            #[cfg(feature = \"stable\")]
            compile_error!(\"[bookcase] {0} is a beta version, but stable feature was enabled.\");
            #[cfg(not(feature = \"beta\"))]
            compile_error!(\"[bookcase] {0} is a beta version, but beta feature was disabled.\");
            #[cfg(feature = \"experimental\")]
            compile_error!(\"[bookcase] {0} is a beta version, but experimental feature was enabled.\");
            ",
            version_str,
        )
    } else if stable != "0" {
        format!(
            "
            #[cfg(not(feature = \"stable\"))]
            compile_error!(\"[bookcase] {0} is a stable version, but stable feature was disabled.\");
            #[cfg(feature = \"beta\")]
            compile_error!(\"[bookcase] {0} is a stable version, but beta feature was enabled.\");
            #[cfg(feature = \"experimental\")]
            compile_error!(\"[bookcase] {0} is a stable version, but experimental feature was enabled.\");
            ",
            version_str,
        )
    } else {
        format!("compile_error!(\"[bookcase] {} is not a valid version.\");", version_str)
    }.parse().unwrap()
}
