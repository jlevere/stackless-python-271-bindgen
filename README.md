# stackless-python-271-bindgen



To generate rust bindings for stackless python 2.7.1.

Needs some work. A nix flake is what pulls down the stackless repo and extracts the header files and stuff. But unfortunently this doesnt exactly work on windows. And we can only build the windows headrs on a windows system because of crazy dependancy hell.

To get around this, I have just manually copied over the results of the nix run into a windows machine and built it, but like, thats horrible. I would rather have something more safe than just a random shell script, so idk.

I'll get around to this one day. It would be nice to build the bindings for all versions of stackless python and all arch types that stackless supports. That is a future problem tho.