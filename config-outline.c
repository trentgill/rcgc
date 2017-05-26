// rcgc config file template

- path of executable file (relative to parent dir)
- author
	- batch // needs correct formatting (eg SP2'17)
	- website / github
- game description // text with character / word limit?
- paths of splash / gameplay image(s) // for the launcher




// the below could be automated with a web app
	// someone else should write it!
	// it could live at: rcgc.recurse.com!!!

{
	// for the launcher
	path       = /main.game;
	engine     = Python;
	keyboard   = (not supported / optional / required);
	mouse      = (not supported / optional / required);
	gamepad    = (not supported / optional / required);
	network    = (not supported / optional / required);
	multiplayer = (local / network / both / none);
	singleplayer = (t / f);


	// for the selector
	author     = "Mr Big Shot";
	batch      = "SP2'17";
	website    = "http://github.com/mrbigshot";
	splash     = "rcgc/splash.png";
	screenshot = "rcgc/screenshot.png";
}


/*
on boot:
	backend loads w/ default config file
	check for system peripheral availability
	which launches the fancy frontend! (it's just a game)
*/	


/*
	can we bitbang the gpios via echo?!?!?!
*/