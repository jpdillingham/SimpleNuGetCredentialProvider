use std::env;
use std::process::ExitCode;
use keyring::Entry;

const SUCCESS: u8 = 0;
const PROVIDER_NOT_APPLICABLE: u8 = 1;
const FAILURE: u8 = 2;

fn main() -> ExitCode {
    // collect the command line arguments; NuGet passes the URL it is seeking credentials
    // for, along with some miscellaneous options that we'll most likely ignore.
    let args: Vec<String> = env::args().collect();

    // find the value of the '-uri' argument
    // this is probably always going to be the 3rd element, but we'll be cautious.
    let mut uri: &str = "";
    let mut capture: bool = false;

    for arg in &args {
        if capture {
            uri = arg;
            break;
        }

        if arg == "-uri" {
            capture = true;
        }
    };

    // build the 'service name' for the keyring credential
    let entry_service: String = format!("NuGet:{}", uri);
    
    // the keyring lib requires a username, and includes it in the entry.
    // seems limiting but there must be a reason. always fixed to 'Simple' here
    let entry: Entry = Entry::new(&entry_service, "Simple").unwrap();

    // fetch the stored creds, if they exist. if they don't, return an empty string.
    let credentials_result = entry.get_password();
    let credentials = match credentials_result {
        Ok(creds) => creds,
        Err(_) => return ExitCode::from(PROVIDER_NOT_APPLICABLE),
    };

    let parts: Vec<&str> = credentials.split(':').collect();
    
    if parts.len() != 2 {
        return ExitCode::from(FAILURE);
    }

    let username = parts[0];
    let password = parts[1];

    println!("{{ \"Username\": \"{}\", \"Password\": \"{}\" }}", username, password);

    ExitCode::from(SUCCESS)
}