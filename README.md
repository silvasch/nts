# nts

A quick and simple notes service. It works by hosting two bash
scripts under "/new" and "/get" that can by executed on any
computer with `curl` and `sh` installed.

## Usage

> [!NOTE]
> This section assumes `nts` is hosted under `https://nts.example.com`

To create a note, run `curl -s https://nts.example.com/new | sh`. It will
ask you for the password and the open your default text editor to enter your
note.

To read your notes, run `curl -s https://nts.example.com/get| sh`.

## Setup

### Using Docker (recommended)

1. Download `docker-compose.yml` and `.env`:

```bash
curl -s https://raw.githubusercontent.com/silvasch/nts/refs/heads/main/docker/docker-compose.yml > docker-compose.yml
curl -s https://raw.githubusercontent.com/silvasch/nts/refs/heads/main/docker/.env.example > .env
```

2. Edit your settings in `.env`. Read the comments in the file to
find out what they do. Variables marked with `!!! ATTENTION !!!`
are important to look at.

3. Start the service using `docker compose up -d`.

4. Set your password by executing `docker compose exec nts nts_set_pwd` and
entering it at the prompt.

## Security

This project does not follow high security standards, and probably never will.
You should **never** store private data using this service, and should **always**
use a password that you don't use with any other accounts.

You should also **never** use this service when hosted by someone other than you.
Downloading a script from the internet and immediately executing it without looking
at it is a **really bad** idea.
