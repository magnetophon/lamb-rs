# lamb 🐑

A lookahead compressor/limiter that's soft as a lamb. 

<p align=”center”>
    <img src="images/lamb.png" alt="screenshot">
</p>

Lamb was made with these goals in mind:
- Be as clean as possible
- Give the user full control over the character with the minimum amount of knobs.

The secret sauce is all in the attack/release:
you can change both the length and the shape of their curve.  
The shapes look like [this](https://www.desmos.com/calculator/cog4ujr7cs); _c0_ in Desmos corresponds to the _shape_ parameter in the plugin.  
When it is at value 0, the curve is a slice of pure sine.  

The ``release hold`` parameter prevents the gain reduction from coming back up if it needs to go down again soon.  
You control how soon is soon with ``release hold``.  
This adds latency though.


## Building

After installing [Rust](https://rustup.rs/) and [Faust](https://faust.grame.fr), you can compile lamb as follows:

```shell
git submodule update --init --recursive
./build.sh
./install.sh
```

## user preferences

  **ATTENTION** If you want to use the plugin with a samplerate of more than 48k, make sure you change MaxSampleRate at the start of lamb.dsp.  
  There's a couple of other user preferences as well, documented in the dsp file.

## Rebuilding the Faust dsp
The faust dsp code in ``dsp/lamb.dsp`` is only transpiled to rust if you build with the ``faust-rebuild`` feature activated, like so: 

``` shell
cargo xtask bundle lamb --release  --features faust-rebuild
```

🐑
