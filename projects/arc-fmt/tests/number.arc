{Number}
int1 =+99
int2 = 42
int3 = 0
int4 =-17
int5 = 1_000
int6 = 5_349_221
int7 = 1_2_3_4_5     % VALID but discouraged
% fractional
dec1 =+1.0
dec2 =-0.0_2
dec3 =+5_0.
dec4 = 3.1415
dec5 =-0.01
dec6 = 224_617.445_991_228
% infinity
sf1 = infinity % positive infinity
sf2 =+Infinity % positive infinity
sf3 =-INFINITY % negative infinity
% not a number
sf4 = nan % actual sNaN/qNaN encoding is implementation specific
sf5 =+NaN % same as `nan`
sf6 =-NAN % valid, actual encoding is implementation specific