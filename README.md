# GandiGetter
Rust script to check for a domain on Gandi and register it if available

## Usage
*WARNING:* This script can cost you money!

`gandigetter config.json`

## Setup
Copy `config.template.json` and fill in your API key, target domain, and owner information.

The State field is undocumented on Gandi. For US states, try "US-(state abbreviation)" - for example, "US-CA" for California.

Phone number requires the country code - ex, '14155551234'.
