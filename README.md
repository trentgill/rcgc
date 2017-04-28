# RC Game Console

////////////////////////
////////////////////////////////
//////// SOFTWARE ELEMENTS /////////
////////////////////////////////////////
////////////////////////////////////////

rcgc selector:
- access to wifi settings
- visual display (gui / shell)
- getting game-info from local db (from game config files)
- search & select engine
	<launcher>
- soft-exit (return to loader when quitting game)

rcgc launcher:
- runs shell commands to launch games
- handles loading browser for web games w/o it feeling like web
	<in game> default overlay for restart / exit

rcgc updater:
- querying the git repo (is the db up-to-date?)
- pull games from the repo to local storage
	-> update local db



	

rcgc os-config:
- init files
- dependencies?

///////
wishlist:
- boot into previous game
- saving games
- online scoreboard





////////////////
////////////////////////
////////////////////////////////
//////// GENERAL OUTLINE /////////
////////////////////////////////////////
////////////////////////////////////////

-Raspberry Pi Model 3
-Keyboard / Mouse / Gamepad support
-HDMI output (Full HD+)
-Sound: HDMI (for starters), then mini-jack output

	// audio input?!

### what we need:
- 2 repos
	- 1: the loader / underlying tech
	- 2: games. pulls in 'releases' from RC'ers repos
		// have an 'update' function
- need an image: includes OS & game engine (not games [or current batch games?])
	- 'update' command pulls games! VIA WIFI
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
- somewhere to host the image? (bt as wishlist)

#### software level:
1. interpreted languages (need a specific interpreter)
2. compiled (need to be built for armv8)
3. browser based (need a browser w javascript)
	skinned chromium?
	map gamepad into keyboard commands (JS lib?)

-- game search engine:
	- each game has a config file w info about each game
	- what info?
		- input devices?
		- single/multiplayer/online
		- tags (genre)
		- rc batch?
		optional:
			- cover image
			- screenshot
			- instructions / intro
			- description
	- filter based on the above

-- first screen after loading a game is input device option
	- dual input options?

- which interpreters
	- love2d for lua
	- python
	- ruby


- how to compile a compiled-game to run on the ARM-v8 platform
- how to run a game as a sub-
- piLove: good reference point at least.


### wishlist:
- bittorrent client
	- runs as a bg process & always seeds the base disk image
	- if possible, use a client built at RC
- JS games in browser?
- bluetooth support for input devices
- physical console
	- documentation for how to build one / enable gpio pins etc
- dealing with run-out of storage
- official releases linked to batch cycles
	- pushes updates & displays what the new games are
