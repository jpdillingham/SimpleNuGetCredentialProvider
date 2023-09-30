# Simple NuGet Credential Provider

A simple NuGet credential provider that stores credentials in your operating system's keyring.

# Why?

I had a need to retrieve packages from a private NuGet repository and I couldn't figure out how to configure my credentials without installing a bunch of Visual Studio- or repository-specific dependencies, and that seemed kind of ridiculous to me.

# How can I install it?

Download the binary for your operating system from the [Releases](https://github.com/jpdillingham/SimpleNuGetCredentialProvider/releases).

## Windows

Unzip the binary to `%localappdata%\NuGet\CredentialProviders`

## Linux or macOS

TBD, need to figure this out still.

# How do I configure credentials?

You'll need to manually store credentials in your operating system's keyring.

## Windows

Open the 'Credential Manager' (Control Panel > User Accounts > Credential Manager, or search 'passwords' and select 'Manage Network Passwords')

Under 'Generic Credentials' click 'Add a generic credential'

In the 'Internet or network address' input, add the following:

```
NuGet:https://<your_server>/index.json
```

The string must be prefixed with `NuGet:`, and the URL must be the exact URL that NuGet uses to access your packages.  For example, to access a NuGet feed hosted by GitHub, you'd enter:

```
NuGet:https://nuget.pkg.github.com/<my_organization>/index.json
```

Enter your username and password, then click Ok to save.
