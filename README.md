# nts

A quick and simple notes service. It works by hosting two bash
scripts under "/read" and "/write" that can by executed on any
computer with `curl` installed.

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
