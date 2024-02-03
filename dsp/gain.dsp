declare name "lamb";
declare version "0.1";
declare author "Bart Brouns";
declare license "AGPLv3";

import("stdfaust.lib");

///////////////////////////////////////////////////////////////////////////////
//                          compile time variables                           //
///////////////////////////////////////////////////////////////////////////////


SampleRate = 48000;
// Make sure you set this correctly for proper functioning of the plugin

NrChannels = 2;
// Speaks for itself.

testingFeatures = 0;
// 0 for a simple plugin
// 1 for gain reduction outputs, an A/B comparison system
// and a comparison to a 4-pole smoother.

///////////////////////////////////////////////////////////////////////////////
//                                  process                                     //
///////////////////////////////////////////////////////////////////////////////


process =
  // SIN_tester;
  par(i, NrChannels, _*ba.db2linear(inputGain)):
  lookahead_compressor_N_chan(strength,thresh,attack,release,knee,link,meter,NrChannels)
  :postProc(testingFeatures)
;

///////////////////////////////////////////////////////////////////////////////
//                         SIN  smoother                                     //
///////////////////////////////////////////////////////////////////////////////

attackSamples = ba.sec2samp(attack);
maxAttackSamples =
  maxAttack*SampleRate
;

SIN(attack,release) = loop~(_,_)
with {
  loop(prevRamp,prevGain,x) =
    ramp
  , gain
  with {
  duration =
    (attack*attacking)+(release*releasing);
  gain = prevGain+gainStep ;
  gainStep =
    select2(releasing
           , rawGainStep :max(dif)
           , rawGainStep :min(dif)
           ) with {
    rawGainStep =
      shapeDif(shapeSlider,ramp,duration,ma.SR)*fullDif;
    fullDif =dif/(1-warpedSine(shapeSlider,ramp));
  };
  shapeDifFormula(shapeSlider,phase,len) =
    warpedSineFormula(shapeSlider,phase+len)
    - warpedSineFormula(shapeSlider,phase);

  shapeDif(shape,phase,duration,sr) =
    warpedSine(shapeSlider,phase+(1 / sr / duration))
    - warpedSine(shapeSlider,phase);

  dif = x-prevGain;
  releasing =
    dif>0;
  attacking =
    dif<0;

  ramp =
    (start,end)
  , shapeDif(shapeSlider,prevRamp+rampStep,duration',ma.SR)
    * ((dif'/dif)/(1-warpedSine(shapeSlider',prevRamp)))
    :seq(i, 16, compare)
    : ((+:_*.5),!) // average start and end, throw away the rest
    :max(0):min(1)
  with {
    start = 0;
    end = 1;
    rampStep = 1 / ma.SR / duration;

    compare(start,end,compSlope) =
      (
        select2(bigger , start , middle)
      , select2(bigger , middle , end)
      , compSlope
      )
    with {
      bigger = compSlope>slope(middle);
      slope(x) =
        shapeDif(shapeSlider,x,duration,ma.SR)
        *(1/(1-warpedSine(shapeSlider,x)));
      middle = (start+end)*.5;
    };
  };
  // ******************************************** the curves: ******************************
  kneeCurve(shape,knee,x) =
    select3( (x>shape-(knee*.5)) + (x>shape+(knee*.5))
           , 0
           , (x-shape + (knee*.5)):pow(2)/(knee*2)
           , x-shape);
  warp(shape,knee,x) =
    (x-factor*kneeCurve(shape,knee,x))/(2*shape) with {
    factor = (1/shape-2)/(1/shape-1);
  };
  sineShaper(x) = (sin((x*0.5 + 0.75)*2*ma.PI)+1)*0.5;
  warpedSine(shapeSlider,x) =
    ba.tabulateNd(0, warpedSineFormula,(nrShapes, 1<<16,0, 0,nrShapes, 1, shapeSlider,x)).lin;

  warpedSineFormula(shapeSlider,x) =
    sineShaper(warp(shape,knee,x:max(0):min(1))):pow(power)
  with {
    power = (4*shape/3)+(1/3);
    knee = min(2*shape,2-(2*shape));
    shape = shapeSliderVal(shapeSlider);
  };
  shapeSlider =
    select2(releasing
           , attackShape
           , releaseShape);


  shapeSliderVal(shapeSlider) =
    shapeSlider
    / (nrShapes-1)
    * range
    + start
    // : hbargraph("shapeBG", 0.3, 0.7)
  with {
    range = 2* (.5-start);
    start = 0.3;
  };
};
};


///////////////////////////////////////////////////////////////////////////////
//                                compressor                             //
///////////////////////////////////////////////////////////////////////////////
lookahead_compressor_N_chan(strength,thresh,att,rel,knee,link,meter,N) =
  si.bus(N) <: si.bus(N*2):
  (
    par(i, N, _@attackSamples)
   ,((par(i,N,abs) : lookahead_compression_gain_N_chan(strength,thresh,att,rel,knee,link,N))
     : par(i, N, (meter(i)))
       <: si.bus(N*2)
    )
  )
  :((ro.interleave(N,2)
     : par(i,N, *))
   , si.bus(N)
   );

lookahead_compression_gain_N_chan(strength,thresh,att,rel,knee,link,1) =
  lookahead_compression_gain_mono(strength,thresh,att,rel,knee);

lookahead_compression_gain_N_chan(strength,thresh,att,rel,knee,link,N) =
  si.bus(N)
  <: (si.bus(N),(ba.parallelMax(N) <: si.bus(N))) : ro.interleave(N,2) : par(i,N,(it.interpolate_linear(link)))
  : par(i,N,lookahead_compression_gain_mono(strength,thresh,att,rel,knee)) ;

lookahead_compression_gain_mono(strength,thresh,att,rel,knee) =
  ba.linear2db : gain_computer(strength,thresh,knee)
  : ba.slidingMin(attackSamples+1,maxAttackSamples)
  : ba.db2linear
    <:
    select2(SINsmoo(testingFeatures)
           , SIN(attack,release)
             :(!,_)
           ,smootherCascade(4, release, attack ))
with {
  gain_computer(strength,thresh,knee,level) =
    select3((level>(thresh-(knee/2)))+(level>(thresh+(knee/2))),
            0,
            ((level-thresh+(knee/2)) : pow(2)/(2*max(ma.EPSILON,knee))),
            (level-thresh))
    : max(0)*-strength;
};

///////////////////////////////////////////////////////////////////////////////
//                                    GUI                                   //
///////////////////////////////////////////////////////////////////////////////

AB(0,p) = p;
AB(1,p) = ab:hgroup("[2]",sel(aG(p),bG(p)));
sel(a,b,x) = select2(x,a,b);
aG(x) = vgroup("[0]a", x);
bG(x) = vgroup("[1]b", x);

SINsmoo(0) = 0;
SINsmoo(1) =
  AB(testingFeatures,checkbox("SIN / 4-pole smoother"));

ab = checkbox("[1]a/b");
inputGain = AB(testingFeatures,hslider("[00]input_gain", 0, -24, 24, 1)):si.smoo;
strength = AB(testingFeatures,strengthP);
strengthP = hslider("[02]strength", 100, 0, 100, 1) * 0.01;
thresh = AB(testingFeatures,threshP);
threshP = hslider("[03]thresh",-1,-30,0,1);
attack = AB(testingFeatures,attackP);
attackP = hslider("[04]attack[unit:ms] [scale:log]",30, 0, maxAttack*1000,1)*0.001;
attackShape = AB(testingFeatures,attackShapeP);
attackShapeP = half+hslider("[05]attack_shape" , 2, 0-half, half, 0.1);
release = AB(testingFeatures,releaseP);
releaseP = hslider("[06]release[unit:ms] [scale:log]",42,1,1000,1)*0.001;
releaseShape = AB(testingFeatures,releaseShapeP);
releaseShapeP = half+hslider("[07]release_shape" , -3, 0-half, half, 0.1);
knee = AB(testingFeatures,kneeP);
kneeP = hslider("[08]knee",2,0,30,1);
link = AB(testingFeatures,linkP);
linkP = hslider("[09]link", 100, 0, 100, 1) *0.01;

nrShapes = 9;
half = (nrShapes-1)*.5;

maxAttack = 0.1;

SINtest = toggle(soft,loud) with {
  toggle(a,b) = select2(block,b,a);
  block = os.lf_sawpos(0.5)>0.5;
  soft = sine*0.1;
  loud = sine;
  sine = os.osc(5000);
};


meter(i) =
  _<: attach(_, (ba.linear2db:max(-24):min(0):hbargraph(
                   "v:[10]meters/%i[unit:dB]", -24, 0)
                ));

///////////////////////////////////////////////////////////////////////////////
//                                    test                                   //
///////////////////////////////////////////////////////////////////////////////

postProc(0) = si.bus(NrChannels),par(i, NrChannels, !);
postProc(1) = si.bus(NrChannels*2);

SIN_tester =
  hgroup("",
         vgroup("[2]test", test)
         <:vgroup("[1]SIN",
                  (ba.slidingMin(attackSamples+1,maxAttackSamples):SIN(attack,release))
                  ,_@attackSamples
                   // ,ba.slidingMin(attackSamples,maxAttackSamples)
                  ,(((ba.slidingMin(attackSamples+1,maxAttackSamples):smootherCascade(4, release, attack )),_@attackSamples):min)
                 ));
test = (select3(hslider("test", 2, 0, 2, 1)
               , test0
               , test1
               , test2
               )

       , no.lfnoise(hslider("rate", 100, 0.1, 20000, 0.1))
       )
       :it.interpolate_linear(hslider("Xfade", 0, 0, 1, 0.01))
;

test0 = select2(os.lf_sawpos(0.5)>0.5, -1,1);
test1 = select3(
          ((os.lf_sawpos(1)>hslider("POS1", 0.25, 0, 1 , 0.01))+(os.lf_sawpos(1)>hslider("POS2", 0.5, 0, 1 , 0.01))),
          1, -1, 0);
test2 =
  (loop~_)
with {
  loop(prev,x) = no.lfnoise0(abs(prev*69)%9:pow(2)+1);
};
N=4;
T = ma.T;
PI = ma.PI;
TWOPI = 2.0 * PI;
TWOPIT = TWOPI * T;
/* Cascaded one-pole smoothers with attack and release times. */
smoother(N, att, rel, x) = loop ~ _
with {
  loop(fb) = coeff * fb + (1.0 - coeff) * x
  with {
  cutoffCorrection = 1.0 / sqrt(pow(2.0, 1.0 / N) - 1.0);
  coeff = ba.if(x > fb, attCoeff, relCoeff);
  TWOPITC = TWOPIT * cutoffCorrection;
  attCoeff = exp(-TWOPITC / att);
  relCoeff = exp(-TWOPITC / rel);
};
};
smootherCascade(N, att, rel, x) = x : seq(i, N, smoother(N, att, rel));
