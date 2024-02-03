# lamb


A lookahead compressor/limiter that's soft as a lamb. 

The secret sauce is all in the attack/release:
you can change both the length and the shape of their curve.  
The shapes look like [this](https://www.desmos.com/calculator/iuvx0mrsyi); _t_ in Desmos corresponds to the _shape_ parameter in the plugin.  
When it has the middle value, the curve is a slice of pure sine.  

## user preferences

The start of the dsp file has a few user changeable values:
- SampleRate  
  **ATTENTION** Make sure you set this correctly for proper functioning of the plugin.
- NrChannels  
  Speaks for itself.
- testingFeatures  
  0 for a simple plugin.  
  1 for gain reduction outputs, an A/B comparison system and a comparison to a 4-pole smoother.

## Building

After installing [Rust](https://rustup.rs/) and [Faust](https://faust.grame.fs), you can compile lamb as follows:

```shell
./build.sh
sudo ./install.sh
```

🐑
