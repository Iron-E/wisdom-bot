# wisdom-bot

## Introduction

`wisdom-bot` is a [Discord][discord] bot for retrieving random videos from a [YouTube][youtube] channel and playing them.

## Installation

1. [Click this link](https://discord.com/api/oauth2/authorize?client_id=830536265823682591&permissions=67584&scope=bot)

### Prerequisites

1. Admin access to a Discord server
2. An audio bot, such as [Rhythm][rhythm].

## Usage

This bot works through the `!wisdom` command:

```ungram
'!wisdom ' Username ('/' CustomUrl)? PlayCommand?
```

* `Username` corresponds to the user name of a [YouTube][youtube]r (e.g. [Markiplier][markiplier])
* `CustomUrl` corresponds to the custom URL of a [YouTube][youtube]r (e.g. "markiplierGAME" for [Markiplier][markiplier])
* `PlayCommand` is the name of the command which plays a video as audio (defaults to `play`).

The following command will play a random [Markiplier][markiplier] video using [Rhythm][rhythm]:

```
!wisdom Markiplier
```

[discord]: https://discord.com/
[markiplier]: https://www.youtube.com/user/markiplierGAME
[rhythm]: https://rythm.fm/
[youtube]: https://youtube.com
