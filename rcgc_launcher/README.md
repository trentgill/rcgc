rcgc launcher:
- on boot, loads the 'selector' game
- grabs path to config.json from selector
- formats a shell script & executes in shell
	- default overlay for restart / exit


launcher:
	input: path to game's config file
	output: formatted terminal command to run game
		or ERROR (need network / gamepad / whatever! )

	transformation:
		where it is
		which interpretter
		interpretter options
		will current state of system be able to run the game
			> return error if not
		format terminal string



- path to executable / main script
- which engine to run?!
	: can this be automated?
		'find' command
		eg: parse 1st line of the script (for ruby / python)
			'#!/usr/env/ruby'
			need string replacement?

what do you need to make your game compatible w rcgc:
	- if interpreter, which one & which version
		if your required engine not available, request new addition to main repo
	- how to launch your game from the shell


launcher
- move functions in lib.rs into their own modules
- add inline documentation to all functions
- build multiple eg config.json files
	> build requirement logic
		> how to poll the OS for peripheral presence
		> how to poll for network connectivity
	> handling for failing config requirements


browser based (need a browser w javascript)
	skinned chromium?
	map gamepad into keyboard commands (JS lib?)

### wishlist:
- JS games in browser?
- bluetooth support for input devices
