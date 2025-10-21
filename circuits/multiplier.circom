// pragma circom 2.0.0;

/*
  multiplier.circom
  Proves that private inputs a and b multiply to public output c.
*/
 
template Multiplier() {
    // private signals (witness)
    signal input a;
    signal input b;

    // public signal
    signal output c;

    // enforce a * b == c
    signal mul;
    mul <== a * b;
    c <== mul;
}

component main = Multiplier();
