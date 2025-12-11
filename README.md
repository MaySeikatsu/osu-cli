# osu-cli

# Design Document

## Feature Overview

### Implementations
- [x] Config file to setup api key and defaults
- [] Command line arguments to compare players and control the output
  - [] subcommands (compare, fetch, stats), flags, dryrun (test without api calls)
- [] Logging / tracing (adding verbose flag)
- [] Proper error handling (review after first features are stable) - custom error types
- [] man pages / documentation
- [] readme file with usage examples
- [] license
- [] support for env variables(?)
- [] writing results to a file to review offline(?)
- [] unit tests / mocking (for api tests)
- [] cicd pipeline
  - [] caching for pipeline
- [] shell auto completion
- [] modular project structure

...?

### Data - User
#### Prio:
- [] gamemode selection
- [] user profile picture
- [] username
- [] country (+image)
- [] best rank (historical)
- [] current rank
- [] country rank
- [] pp
- [] ranked score
- [] accuracy
- [] maximum combo
#### Secondary:
- [] level
- [] playcount
- [] total score
- [] total hits
- [] hits per play
- [] joined year
- [] last seen
- [] medals
- [] total playtime
- [] playstyle (tablet, mouse, touchscreen etc.)
- [] amounts achieved results for each ranking (SS, S, A)
- [] recent plays

### Data - Ranks / Beatmaps
- [] Pinned scores
- [] Overview best 20(?) plays with all stats ( 
  - gamemode
  - mods
  - pp
  - pp gain (relative to player)
  - accuracy
  - ranking
  - global ranking on play
)
- [] Most played beatmaps
- [] favorite beatmaps
- [] created beatmaps


# Future
- [] Create a TUI version
- [] Themes
