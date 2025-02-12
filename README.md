# Rabduction!

A simple tile climbing game using bevy.

The name **Rabduction!** comes from (R)ust + a similar old-school Android game called
[Abduction!](https://play.google.com/store/apps/details?id=au.com.phil&hl=en_IN&gl=US)

## Screenshots

<img src="https://i.imgur.com/T4X3vCm.png" width="300" height="500">

## Running

### Nix

If you have flakes enabled, you can try out the game using:
```
$ nix run github:ritiek/rabduction
```

And development environment can be set up through (adds rustc, cargo, etc. to PATH):
```
$ git clone https://github.com/ritiek/rabduction
$ cd rabduction
$ nix develop
$ cargo build --locked
```

You can also build and execute the game using:
```
$ nix build
$ nix run
```

### Others

```
$ git clone https://github.com/ritiek/rabduction
$ cd rabduction
$ cargo run --locked --release
```

Press any key on the keyboard to spawn the player and then use left and right arrow keys to
move the player.

If you have a controller too, connect it and press any action button to spawn the player and
then use the left analog stick to move the player.

## License

MIT
