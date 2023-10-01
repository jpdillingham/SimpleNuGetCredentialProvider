# Simple NuGet Credential Provider

A simple NuGet credential provider that stores credentials in your operating system's keyring.

# Why?

I had a need to retrieve packages from a private NuGet repository and I couldn't figure out how to configure my credentials without installing a bunch of Visual Studio- or repository-specific dependencies, and that seemed kind of ridiculous to me.

Also, some of my repositories contain a NuGet.config, so credentials saved in my personal NuGet.config can't be applied.  The fact that NuGet can't source credentials from multiple NuGet.config files seems a bit ridiculous to me as well.

This is mostly a self-inflicted problem caused by stubbornness.

# How can I install it?

Clone the repo and build it yourself with the included build script (you'll need Rust installed).

## Windows

Unzip the binary to `%localappdata%\NuGet\CredentialProviders`

Note: this method is apparently deprecated, and when using NuGet from the command line you'll get a warning about this.

## Linux or macOS

TBD, need to figure this out still.

# How do I configure credentials?

You'll need to manually store credentials in your operating system's keyring.

## Windows

Open the 'Credential Manager' (Control Panel > User Accounts > Credential Manager, or search 'passwords' and select 'Manage Network Passwords')

Under 'Generic Credentials' click 'Add a generic credential'

In the 'Internet or network address' input, add the following:

```
Simple.NuGet:https://<your_server>/index.json
```

The string must be prefixed with `Simple.NuGet:`, and the URL must be the exact URL that NuGet uses to access your packages.  For example, to access a NuGet feed hosted by GitHub, you'd enter:

```
Simple.NuGet:https://nuget.pkg.github.com/<my_organization>/index.json
```

Enter the username "Simple" and in the password field enter your username and password separated by a colon, e.g `<my username>:<my password>`, then click Ok to save.

## Linux or macOS

Not sure, didn't test.
