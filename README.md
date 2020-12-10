# Xipe Oito
## a.k.a. Emerson, Lake & Palmer

This was my first experiment with both Rust and emulation, I hope you like it.
It's divided in 3 projects: a native front-end, a WASM web front-end and a library implementing Chip 8's engine.

## Emerson, the pretentious native front-end
It's made with the [iced](https://github.com/hecrj/iced/) library and runs pretty smoothly on anything but Windows native (you can run it on WSL with X11 
and it will run perfectly).
To play with it, run `make emerson-dev`. A file picker will open an you just have to select your rom and start playing.
To build a release binary, run `make emerson-build`.

## Lake, the delightful web front-end
This one was made with the [yew](https://github.com/yewstack/yew) framework. I chose it since it looks a lot like React, which is my favorite JS library.
To run it on dev mode, simply run `make lake-dev`. It will be listening on `localhost:8080`.
To build the production bundle, run `make lake-build` and it will be on the `lake/dist/`.
A [live version](chip8.cel.so) is available.

## Palmer, the core of it all
This is the library responsible for the Chip 8 engine. This is the only library that has tests because I couldn't find documentation on testing both `yew` or `iced`. If you have it, 
I would love to add more tests.
There is no program here, but you can run the tests with `make test`.


![Keybinding map, showing how to play the games on your computer's keyboard](https://raw.githubusercontent.com/celsobonutti/xipe-oito/master/map.png)
This is a simple image showing how your computer's keyboard is mapped to the Chip8's.

If you read it all through here, thank you a lot! This is one of the projects I'm the proudest about. If you have any doubt or tip, feel free to reach me - you can find my information 
on my GitHub profile.
