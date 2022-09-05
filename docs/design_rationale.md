# TentHash's Design Rationale

This document explains the rationale behind TentHash's design decisions.  It is organized as a Q&A for fun and clarity.


## Why the 160-bit digest size?

TentHash is not intended for use with hash maps, but is instead intended as a message digest that can uniquely identify data.  When attackers aren't a consideration, as with TentHash, 128 bits should be enough for that, and 128 bits was the original target for TentHash.  But other aspects of TentHash's design (expanded on later in this document) resulted in a reliable level of collision resistance consistent with an 160-bit digest, and there didn't seem to be any reason to not expose that.  People can always truncate further if desired.

It is actually a complete coincidence that 160 bits is also the digest size of hashes like SHA1.


## Why the 256-bit internal state size?

In short, for speed.  A 256-bit state allows us to process input data in 256-bit chunks, which is around twice as fast as processing it in 128-bit chunks.  (This is discussed in more detail in the section about the mixing function design.)

An even larger state could have been chosen, such as 512 bits, for further potential gains.  And I did experiment with that.  But in practice the speed gains were minimal without employing explicit SIMD, and it also increased the complexity of the mix function to an extent that didn't seem appropriate given TentHash's goals.

256-bits strikes a nice sweet spot between good performance and a simple implementation.


## Why the simplistic xor-and-mix construction for mixing the input data?

Because for a non-cryptographic hash you really don't need anything more complex.  It provides no security from attackers, of course: the mixing function is trivially reversible, so you can just walk the hashing process backwards to get any output digest you like.  But for a non-cryptographic hash function that doesn't matter.

What *does* matter is that the approach:

1. Allows every bit in the input data to affect the output digest with a (very high) probability consistent with the digest size.
2. That it strongly orders the input chunks.  I.e. swapping two chunks also affects the output digest with a (very high) probability consistent with the output digest size.

And as long as the mixing function is good, the xor-and-mix approach accomplishes this just fine.

Another benefit is that, because of its simplicity, the properties of the xor-and-mix approach are reasonably easy to analyze and reason about.


## Why not process the data in multiple independent lanes for faster SIMD execution?

Some fast hashes do indeed take this approach.  And it works really well for speed: since the lanes never interact until the end, you can run them perfectly in parellel.

However, doing that reduces the collision resistance of the whole hash to that of a single lane.  For example, if your lane size is only 64-bits wide, then using the independent-lane approach results in a hash with a collision resistance equivalent to a 64-bit hash, even if the lanes are mixed into e.g. a 256-bit digest at the end.

So for smaller lane sizes that obviously wouldn't be appropriate for a message digest hash like TentHash.  And although 128-bit lanes would have been fine, in my experiments that didn't give much speed improvement because the mixing function I chose already effectively parellelizes two 128-bit lanes while still doing full 256-bit mixing.

Moreover, to be able to safely truncate a hash digest, all the bits need to be mixed well in the final digest.  And that means that using independent lanes effectively splits mixing into two separate functions: one for processing the input data, and a separate different mixing function to do the final digest mixing.  And that was added complexity I didn't want for TentHash: I wanted just one mixing function used for everything.

In other words, independent lanes harms both collision resistance *and* simplicity for only minor speed gains.

(Having said that, if performance needs to be absolutely break-neck fast for some application, there's nothing stopping anyone from running TentHash in a "striped" mode: e.g. passing every other chunk to a separate TentHash hasher, and then appending the digests and throwing those through another TentHash hasher to get the final hash.  Such an approach could presumabl be e.g. AVX512 accellerated.  It would result in a different digest than plain TentHash, of course, but as long as that fact is properly documented that's not a problem.)


## Why doesn't TentHash use multiple threads for faster hashing?

Because it adds more complexity than I wanted for TentHash.  And because (as with any hash function) multi-threading can be layered *on top of* TentHash instead of being built-in:

* If you have multiple items of data to process, you can simply dispatch each item to be hashed in a different thread.  That will generally be both faster and simpler than trying to use multi-threading to process a single data item.
* If you do need to process a single data item with multiple threads, either striping (as discussed in the previous section about SIMD) or a Merkle-tree construction can still be used, just with TentHash as the base hash function.  The resulting digest will be different from plain TentHash, of course, but as long as that fact is properly documented that's not a problem.


## How did you design TentHash's mixing function?

It wasn't from scratch.  It's actually identical to Skein's MIX & permute approach, just with different rotation constants and with the chunks ordered a little differently.

I chose Skein's approach for a few reasons:

1. It's simple and easy to understand.
2. It (of course) satisfies the basic requirements of a mixing function, such as being bijective.
3. The most complex operation it uses is addition, so all operations are easy and reasonably efficient to perform in software even on 32-bit or lower platforms.  This makes portable implementations straightforward.
4. It is easily "vectorized" by CPUs without any explicit vectorization in the code.

To explain point 4, let's revisit a single round of mixing:

```python
    A += C
    B += D
    C = (C <<< rotation_constant) ^ A
    D = (D <<< rotation_constant) ^ B
    swap(C, D)
```

The only dependent operations are the xors with `A` and `B` when computing the new `C` and `D`.  Everything else is an independent operation that can be dispatched simultaneously by the CPU.  This means that—assuming each operation takes one CPU cycle, as is the case on x86-64—a single round of mixing can be done in just 2 CPU cycles.  And at least on x86-64, this is indeed what happens.

And this is all with a very simple, straightforward implementation that requires no special hardware instructions or coding gymnastics to make efficient.

The main drawbacks of this mixing function are:

1. It is zero-sensitive: if the hash state is all zeros, then mixing doesn't do anything.
2. It takes quite a few rounds to achieve full diffusion.  The Skein paper suggests that it takes 9 rounds, which is consistent with my own testing.

The first drawback isn't an issue due to other aspects of TentHash's design: even though the mix function is zero-sensitive, the overall hash is not.

The second drawback is certainly a little unfortunate: even though each individual round is very fast (2 cycles), achieving full diffusion requires 9 rounds (18 cycles total).  However, in testing other constructions (such as multiplication-based ones) that achieve full diffusion, I couldn't actually make anything faster.  Someone a little more skilled than I at optimization could probably figure something out.  But I don't think it would be *much* faster outside of employing specialized hardware instructions.

So for 256 bits of state I believe the Skein MIX function has at least *competitive* performance--all while being simple and easy to implement on a variety of platforms.


## Can you explain in more detail why TentHash's mixing function being zero-sensitive isn't a problem?

Sure!  As mentioned already, even though the mixing function is zero-sensitive, TentHash itself isn't.  And this is for two reasons:

1. We incorperate the input data length into the hash.
2. We initialize the hash state to be non-zero.

Let's start with point 1.  Imagine that we actually initialized the hash state to zero.  Even with that being the case, and with input data composed entirely of zeros, we would still get different hashes for different input data lengths.  This is also why it's fine for us to just pad the final input chunk with zeros: actual zeros in the data are distinguished from padding zeros by the message length.  (There's some deeper analysis of this that addresses some other "what if" scenarios, but this explanation should at least give you an intuition.)

Now on to point 2.  By intializing the state to a gibberish non-zero value, it is vanishingly unlikely that the state will ever become zero in the first place.  A piece of input data would have to *exactly match* that gibberish to xor it to all zeros.  That would be trivial for an attacker, of course.  But TentHash isn't meant to protect against attacks.  For real data, the chance of a match is roughly 1 in 2<sup>256</sup>.  Or in other words, it can be safely assumed to never happen.


## If full diffusion takes 9 rounds of mixing, why does TentHash only use 6 rounds between input chunks?

TODO
