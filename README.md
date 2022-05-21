# mapgen
A small collection of procedural dungeon generation algorithms.  
Built with Rust & [macroquad](https://github.com/not-fl3/macroquad).  
WebAssembly build deployed [here](https://optimistic-nihilist.github.io/mapgen).  

Animated version (separate repo so I could deploy it too using github pages): [mapgen-animated](https://github.com/optimistic-nihilist/mapgen-animated)  
deployed [here](https://optimistic-nihilist.github.io/mapgen-animated/)

### Run
Clone the repository, then `cargo run --release`.

### Build WASM
`cargo build --release --target wasm32-unknown-unknown` produces `mapgen.wasm` under `target/wasm32-unknown-unknown/release`.
Read [this](https://github.com/not-fl3/macroquad#wasm) for a detailed example on what to do with it.  

### Credits
- Heavily inspired (and on occasion, downright stolen from) https://github.com/AtTheMatinee/dungeon-generation  
- Tiles are from 0x72's amazing 16x16 dungeon tileset ([v1](https://0x72.itch.io/16x16-dungeon-tileset) & [v2](https://0x72.itch.io/dungeontileset-ii))
- Algorithms (sometimes loosely, oftentimes lousily) based on:
  - Tunneling algorithm: [roguebasin](http://www.roguebasin.com/index.php/Complete_Roguelike_Tutorial,_using_python%2Blibtcod,_part_3)
  - BSP tree: [roguebasin](http://www.roguebasin.com/index.php/Basic_BSP_Dungeon_generation)
  - Random walk: [roguebasin](http://www.roguebasin.com/index.php/Random_Walk_Cave_Generation)
  - Cellular automata: [roguebasin](http://www.roguebasin.com/index.php/Cellular_Automata_Method_for_Generating_Random_Cave-Like_Levels)
  - Room placement: [rockpapershotgun](https://www.rockpapershotgun.com/how-do-roguelikes-generate-levels)
  - Maze with rooms: [journal.stuffwithstuff.com](http://journal.stuffwithstuff.com/2014/12/21/rooms-and-mazes/)
