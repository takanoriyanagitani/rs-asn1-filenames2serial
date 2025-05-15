use std::process::ExitCode;

use std::io;

fn sub() -> Result<(), io::Error> {
    rs_asn1_filenames2serial::stdin2names2pairs2der2stdout()
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
