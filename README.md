[![Automated Builds](https://github.com/magnetophon/lamb-rs/actions/workflows/build.yml/badge.svg)](https://github.com/magnetophon/lamb-rs/actions/workflows/build.yml)

# lamb 🐑

A lookahead compressor/limiter that's soft as a lamb. 

<p align=”center”>
    <img src="images/lamb.png" alt="screenshot">
</p>

Lamb was made with these goals in mind:

- Sound as clean as possible and as dirty as desired.
- Give the user full control over the character with the minimum amount of knobs.

## Features

The following features set it apart from other compressor/limiters:
#### Use it as a brickwall limiter, a compressor, a leveler, a waveshaper/clipper or anything in between.  
  As long as the ratio is inf:1, the lookahead is 100% and the output gain is 0dB, the output will not exceed the threshold[^1].  
  When the attack, release, release hold and adaptive release are all at their minimum value, you get a waveshaper/clipper.  
  When adaptive release is at 100%, you get a leveler.
#### Adjust the shape of the attack and release.  
  The inspiration for this came from [a video by Dan Worrall](https://youtu.be/7Yit769SN64?t=1115).  
  When shape is at 0, the curve is a slice of pure sine.  
  ![lamb_shape](https://github.com/magnetophon/lamb-rs/assets/7645711/bfb42317-0dfb-451e-84f0-a6af50eed433)
#### No discontinuities in the derivative of the gain reduction.  
  The gain never abruptly changes direction, resulting in a smoother sound, even at short attack and release times.
#### Release hold eliminates distortion while keeping most of the level.  
  Release hold prevents the gain from coming up if it needs to go back down again soon.  
  You control how soon is soon.  
  Here's a gain reduction graph with and without it:
  ![hold](https://github.com/magnetophon/lamb-rs/assets/7645711/6b11f866-2684-41a4-beb7-f83ea2964246)
#### Adaptive release (optionally) prevents pumping.  
  With adaptive release, the gain won't rush up too much when there is a quiet part after a big peak, but it will still react quickly to transients.  
  The first couple of dB of release have the speed you set with the release knob, after that it slows down.  
  The adaptive release slider controls how many dBs will release fast and how much it will slow down afterwards.  
  If you DO want obvious pumping, just turn adaptive release off!
#### Exact attack and release times allow you to easily match any breathing to the tempo of your track.   
  In most compressors and limiters, the times describe how long it takes to do "most" of the gain change.  
  In lamb, 500 ms corresponds to exactly 1/4 note at 120 BPM.
#### Adjust the amount of stereo linkage.  
  Most limiters are fully stereo linked.
  This makes sense, since you don't want the stereo image to shift.  
  However, if the asymmetry in gain reduction is small and short enough, you won't notice any shift in stereo image.  
  (Partially) unlinking left and right can sometimes sound more natural, because a loud sound on one side won't make the other side duck in level.  
  Slower changes in gain reduction, caused by  adaptive release, are always fully linked.  
  You can set the amount of linkage for transients as needed.
#### Choose between fixed or minimum latency.  
  In most case, you can leave this at fixed, but if you want to use lamb live or for tracking, you can set it to minimum.  
  The latency is always reported to the host.
  
## Downsides
There are two main downsides to this plugin:
- Heavy on the CPU.  
  Because of the advanced algorithm, this plugin is quite heavy.  
  I have done everything in my power to make it lighter, from writing a highly optimized [sliding minimum algorithm](https://github.com/grame-cncm/faustlibraries/blob/d28c51f6c667e00f521a8cb2232786795c558aa4/basics.lib#L2258-L2618)  
  to writing an [N dimensional memoization function](https://github.com/grame-cncm/faustlibraries/blob/d28c51f6c667e00f521a8cb2232786795c558aa4/basics.lib#L956-L1495).  
  If you are good at math or computer science and want to help me optimize it more, open an issue, or better yet a PR!
- Long latency.  
  The clean sound of this plugin is partially made possible by a copious amount of lookahead.  
  In fixed latency mode, the latency is 100ms.  
  In minimum latency mode, the latency depends on the attack, release hold and lookahead parameters.
  

## Usage

Apart from regular dragging and using the mouse-wheel, you can interact with the sliders in the following ways:
- Shift + drag or mouse-wheel: fine adjustments
- Alt + click on a slider: type a value
- Double-click or right-click on a slider: back to the default value.


## Building and installing

After installing [Rust](https://rustup.rs/), you can build and install lamb as follows:

```shell
git submodule update --init --recursive
cargo xtask bundle lamb --release
cp -r target/bundled/lamb.vst3 ~/.vst3
```

## Rebuilding the Faust dsp

NOTE:  this is only needed if you want to change the dsp, not if you just want to compile the plugin.

The faust dsp code in ``dsp/lamb.dsp`` is only transpiled to rust if you build with the ``faust-rebuild`` feature activated, like so: 

``` shell
cargo xtask bundle lamb --release  --features faust-rebuild
```

The smoothing algorithm in lamb needs double precision to work.  
This only recently [got supported](https://github.com/grame-cncm/faust/commit/9f2eb5766605f9f8235a45965c69ff33b4274685), so you need to use faust version [2.72.14](https://github.com/grame-cncm/faust/releases/tag/2.72.14) or newer.  
This faust version has a bug for Windows, that got fixed [here](https://github.com/grame-cncm/faust/commit/bde0c9e3168a6da9e953367856099100e9537490).  
Therefore, Windows users currently need to build faust from source to be able to rebuild the dsp of lamb.  
[Here's](https://github.com/grame-cncm/faust/wiki/BuildingSimple) a quick tutorial on how to do that.


## Thanks

This plugin would not have been possible without the following projects:
- [Faust](http://faust.grame.fr)
- [NIH-plug](https://github.com/robbert-vdh/nih-plug)
- [lowpass-lr4-faust-nih-plug](https://codeberg.org/obsoleszenz/lowpass-lr4-faust-nih-plug)
- [Vizia](https://github.com/vizia/vizia)
- [Cyma](https://github.com/exa04/cyma)

I would like to thank [Stéphane Letz](https://github.com/sletz), [Robbert van der Helm](https://github.com/robbert-vdh), [obsoleszenz](https://github.com/obsoleszenz), [exa](https://github.com/exa04), [Dr George Atkinson](https://github.com/geom3trik) and [Dario Sanfilippo](https://github.com/dariosanfilippo) for their fantastic support and feedback!   

🐑  

[^1]: Lamb does not feature True Peak limiting yet.
This is in the pipeline though: https://github.com/magnetophon/lamb-rs/milestone/1  
For most applications, this is less of an issue then you might think, see: https://www.izotope.com/en/learn/true-peak-limiter.html
