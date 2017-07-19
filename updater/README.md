updater
- how to build a list of all games & paths
- tech:
	> need a list of all the available games
	> 
- functionality:
	> check the list of games
	> compare current system to list
	> update games w new version
	> pull new games
- how does a new dev use the system?
	> write a config.json (include in root dir)
	> add a link in rcgc_repo back to their game repo
		(every RC member needs r/w access)
		?? create quality issues
		?? security issue (malicious or not)
	> suggest a 'maintainer' who accepts git pulls
		to add
	> automate via a webapp (who has permission!)
- how does a remote system know about updates
	- updater: `git pull origin master` lol


	> dev can point to stable release of their game
		(able to update the official version pointer)
		- is this with a github release?


>
	query a server for a list
	d/l missing games
	update existing games
	< this is essentially a git pull >

have a rcgc repo:
	dev adds a 





### wishlist:
- bittorrent client
	- runs as a bg process & always seeds the base disk image
	- if possible, use a client built at RC

// functionality for future batches to implement:

updater:
-what to update?
	- choosing specific games to pull
	- choosing some subset of games
	- explicitly updating 1 game
	- updating all games, but not pulling new ones
	- pulling new games, but not updating extant
