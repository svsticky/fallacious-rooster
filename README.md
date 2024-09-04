# fallacious-rooster
Anonymous reporting point for social safety for study association Sticky.
This reporting point allows you to anonymously send a message to either the board or to the confidential advisors, or both.

The application does require a login via Koala to avoid spam messages. The board has quite good spam filters, but the confidential advisors might not have.
Even though login is required, the application takes upmost care to ensure user anonymity.

The original idea for this application was put forward in [the policy of the 10th KKB on the kbALV on 2024-05-21, section 2.2](https://files.svsticky.nl/alv_stukken/2024_05_21_kbALV/Beleidsplan%2010e%20KKB.pdf). 

## Development 
The guide below will tell you roughly how to get started with working on fallacious-rooster.

## Koala
Fallacious-rooster uses Koala for user authorization, you need to have this running when working on the application.

## Network setup
Add `fallacious-rooster.localhost` to your hosts file. On Ubuntu this is `/etc/hosts`
This is rqeuired for cookies.

## Server setup
The server's written in Rust and thus requires a Rust compiler to be installed.

1. Copy the configuration file
```bash
mv sample_config.json config.json
```
2. Create an OAuth application on your [Koala instance](http://koala.rails.local:3000/api/oauth/applications)
    1. Set the redirect URI to match your `config.json` file
    2. Set the scopes to `member-read openid email profile`
3. Copy the client ID and client secret from Koala to `config.json` (`koala.client_id` and `koala.client_secret`)
4. Start the application, either via Cargo directly:
```bash
cargo run -- --config ./config.json
```
or via:
```bash
cargo build 
./target/debug/fallacious-rooster --config config.json --dry-run
```
Replace `config.json` with the path to your config file.

### Dry run
The `--dry-run` argument is useful during development. This argument tells the application to not send emails when a report is sent in,
instead it will print the body of the email to stdout.

### Settings
The server preserves user-configurable settings in a `storage.json` file. Where this file is placed can be configured via the `local_storage` key of `config.json`.
This file contains no sensitive data.

## Frontend setup
The frontend is written in Vue and TypeScript. Use Node version 21. Using [NVM](https://github.com/nvm-sh/nvm) will make your life easier.

1. Install yarn
```bash
npm install --global yarn
```
2. Install packages
```bash
cd frontend/
yarn
```
3. Start the frontend in development mode
```bash
yarn run dev
```

# License
MIT or Apache-2.0, at your option.