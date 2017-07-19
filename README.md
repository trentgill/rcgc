# RC Game Console

## Consists of 3 main elements:

### chooser
- build it inside of a game engine (love2d?)
- setup & access wifi settings
- getting game-info from local db (global-config.json)
- display list of games w/ screenshots etc
- select game -> return path to that game's root dir

### launcher
- on boot, loads the 'selector' game
- grabs path to config.json from selector
- formats a shell script & executes in shell
	- default overlay for restart / exit

### updater
- querying the git repo (is the db up-to-date?)
- pull games from the repo to local storage
	-> update local db



## Tech Outline

- Raspberry Pi Model 3
- Keyboard / Mouse / Gamepad support
- Sound: HDMI (for starters), then mini-jack output
	- audio input

### Structure

- 2 repos
	- 1: chooser, launcher, updater
	- 2: games repo. (target of updater)
- disk image (tarball): includes OS & game engine
- network configuration (on first load): select network & enter pw
- quality control
	- auto-testing?
	- require a 'stable release' flag -> grab latest 'stable' version
	- talk to git masterminds how best to do this / easily flag commits
- documentation
	- how to get your game into the repo
	- how to design your game to work with predefined inputs
		- require config file to describe input methods / provide alt keybindings
			- focus on ease of use
	- how to mark a 'stable release'
	- how to make your own SDcard (format / image)
- somewhere to host the image?

- piLove: good reference point at least.


# wishlist

- saving games
- online scoreboard
- physical console
	- documentation for how to build one / enable gpio pins etc
- official releases linked to batch cycles
	- pushes updates & displays what the new games are
- volume controls & reset button (GPIO)
