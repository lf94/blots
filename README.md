# Blots - A simple paper-based storage

![Blots sample](./sample.png)

Because digital storage has a hard time lasting past 10 years, and paper is proven to last centuries, I have created Blots. It takes digital data and arranges it on paper for long term archives or sensitive data that needs to be put into "cold storage".

There are several other non-magnetic based storage formats (technically, CD/DVD/Blu-rays go into this category too. Unfortunately they are just as bad - M-DISC's Blu-rays being the exception) such as the well known QR Code, DataMatrix, Optar by Twibright Labs, and a few others you will find with some research.

Note: **I am not an expert with this type of technology**. If you know of some issues this current format might have, please let me know in a pull request, but explain yourself, not just for me, but others who wish to learn.

## Goals and technicalities
The Blots format aims to be simple for self-preservation. Implementation can be done in a night. It aims to incorporate simple error correction (anti-decay), alignment, anti-dirt and to be somewhat dense.

It achieves these with simple XOR parity bits, an "alignment component" located at the top left, no use of black, and storing 2-bits per dot using 4 colors: red, green, blue and white (or absence of ink).

The format is also semi "anti-fold", meaning folding should not cause too many problems. This is because of how data blocks are arranged:

```
[a1][a2][p2]
[p1][b1][b2]
```

Each byte is broken into 4 pieces, and then those 4 pieces divided into two groups: a and b.

Each group has its own parity bits, for reconstruction.

Because of the way they are arranged, folding may destroy some data on one line, but not the other.

# Caveat
While I have implemented an encoder, **I have had no time implementing a decoder**.

To challenge the goal of simplicity, I encourage the users of Blots to implement their own.

I will pull your decoder if it is simple and a clean implementation!

## Building
`cargo build --release`

## Usage
`./blots [--encode|--decode] filename`

It will output a new file, `filename.blots.png`.

