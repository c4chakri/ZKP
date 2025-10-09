pragma circom 2.0.0;
template Pack8() {
  signal input b[8];      // private bits (0/1)
  signal output packed;   // public packed value

  var sum = 0;
  for (var i = 0; i < 8; i++) {
    // assert bitness: b[i] * (b[i] - 1) == 0 is a boolean check
    b[i] * (b[i] - 1) === 0;
    sum += b[i] * (1 << i);
  }
  packed <== sum;
}
component main = Pack8();
