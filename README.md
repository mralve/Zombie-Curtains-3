# Zombie Curtains 3™  ⚠ Work in progress! ⚠
An experimental 2D top-down survival game, originally developed by ZurraGameZ in 2011. This is the third edition of the game, being developed in the Amethyst game engine. This game is procedurally generated, with different types of enemies in the form of zombies.

### How to run
You only need the following setup on your computer to run the game.
- Rust tool chain(rustc, cargo)  https://rustup.rs/
- Git  https://git-scm.com/

```bash
git clone https://github.com/mralve/Zombie-Curtains-3.git
cd Zombie-Curtains-3
cargo run --release
```
- Note, running with --release tag takes more time to compile, but has the best runtime performance. excluding the tag enters debug mode, good for general development.

### Development status.

Current short term goals.
- [x] Refactor and upgrade to Amethyst 0.13.2
- [x] Implement basic tilemap.
- [ ] Re-implement basic player mechanics. 
- [ ] Basic combat.

Long term goals.
- [ ] World Generation (Trees, Structures, ect.)
- [ ] Advanced inventory management (Pickup items, drag and drop, stacking)
- [ ] Multiplayer???
- [ ] Advanced AI, (A-star) or ML(Machine learning)
- [ ] Saving and loading.
- [ ] Proper pause menu and start menu.
- [ ] More to come here :)

### More Experiments under the same "franchise"
Java raycaster experiment.
https://github.com/TheEggHealer/Zombie-Carpet
