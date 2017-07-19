rcgc chooser:
- build it inside of a game engine (love2d?)
- setup & access wifi settings
- getting game-info from local db (global-config.json)
- display list of games w/ screenshots etc
- select game -> return path to that game's root dir


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

# Wishlist

-whether / when to update?
	- modifies the startup script (which runs updater)
- access to list of games & paths (from updater)
- return a path to the game-dir
- handles the update
- changing network connection details
