import("stdfaust.lib");
fq = hslider("CutOff", 1, 0, 1, 0.001) : si.smoo : denormalize_fq(40, ma.SR / 2);

// TODO: Is denormalization the correct term?
denormalize_fq(offset, max_fq, fq) = (fq * fq)  : * (max_fq - offset) + offset;

process = _,_  : par(i, 2, fi.lowpassLR4(fq));
