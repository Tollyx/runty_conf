# runty_conf

A tiny rust library for using simple ini-like config files.

## Usage

### Loading/creating a config file

```rust
let conf = runty_conf::load("app.cfg");
```

### Getting values
```rust
let name = conf.get_string(&key, &default);
```
It returns and inserts the default value if the key doesn't exist.

There are methods for numbers (f32) and bools as well.

### Setting values
```rust
conf.set_string(&key, &value);
```

### Example output
```
width = 800
height = 600
fullscreen = false
mode = foobar
```

## Possible improvements
 * Do not allow identical keys for different types of values. (The config values are stored in different BTreeMaps depending on type)
 * Store and return values in enums?
