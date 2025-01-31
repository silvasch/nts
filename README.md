# nts

A quick and simple notes service. It works by hosting two bash
scripts under "/read" and "/write" that can by executed on any
computer with `curl` installed.

## Usage

> [!NOTE]
> This section assumes `nts` is hosted under `https://nts.example.com`

To create a note, run `curl -s https://nts.example.com/write | sh`. It will
ask you for the password and the open your default text editor to enter your
note.

To read your notes, run `curl -s https://nts.example.com/read | sh`.

## Setup

### Using Docker (recommended)

1. Clone this repository:

```bash
git clone https://github.com/silvasch/nts
cd nts
```

2. Edit your settings in `.env`. Read the comments in the file to
find out what they do. Variables marked with `!!! Attention !!!`
are important to look at.

3. Start the service using `docker compose up -d`.

## Security

This project does not follow high security standards, and probably never will.
You should **never** store private data using this service, and should **always**
use a password that you don't use with any other accounts.

You should also **never** use this service when hosted by someone other than you.
Downloading a script from the internet and immediately executing it without looking
at it is a **really bad** idea.
