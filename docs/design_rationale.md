# TentHash's Design Rationale

This document explains the rationale behind TentHash's design.


## Goals

TentHash is designed with the following goals, in order of priority:

1. **Strong collision resistance.**  For all practical purposes, it should be safe to assume that different pieces of (legitimate) data will never have colliding hashes.
2. **Simplicity.**  It shouldn't require diagrams to understand, and it should be straightforward to write conforming implementations for just about any platform.  Its design should also be *documented* and *easy to audit*.
3. **Reasonably fast.**  It doesn't need to win any benchmark competitions, but it should be able to process 100 GB of data in seconds, not minutes, on a reasonably modern computer without resorting to multi-threading.

Some **non-goals** are:

* TentHash is **not** trying to be secure against attacks.  It is explicitly a *non-cryptographic* hash function.
* TentHash is **not** trying to be especially fast for very small inputs: its target use case is message digests, not hash maps.


## The xor & mix loop.

TentHash mixes the input data into the hash state with a simple xor-and-mix loop, like so:

```sh
for chunk in input_data:
    hash_state ^= chunk
    mix(hash_state)
```

This approach was chosen because of its simplicity and because a more sophisticated approach simply isn't needed when attackers aren't a consideration.

As long as the mixing function diffuses the hash state well, this simple loop accomplishes two things:

1. It gives every bit of input a good chance of affecting every output bit in the digest.  Meaning that flipping any single bit in the input will, with overwhelming probability, completely change the output digest.
2. It *strongly orders* the input data.  Meaning that (for example) swapping two input chunks will, with overwhelming probability, completely change the output digest.

That second point is important.  For example, you could instead design a hash that first xors all the input data, and only *afterwards* does any mixing.  And if well mixed at the end, that would still have the first property, but it would lose the second one: swapping any input chunks would have no effect on the hash output.

Another way to look at this is: if there is a single bit flip in the input, what is the expected complexity of the additional bit flips necessary to cancel it out and result in the same output digest?  If you just xor all the input chunks before doing any mixing, then it trivially only takes a single additional bit flip: just flip the same bit location in any other input chunk.  Which is *extremely low complexity*—just a 1/256 "chance" in a very informal sense.

But if the hash state is fully mixed between xoring input chunks, that complexity becomes equal to the size of the hash state, making accidental cancellations about as astronomically unlikely as the hash state is large.

In other words: if point 1 is about making the relationship between the input and output bits complex, then point 2 is about making the relationship of the input bits *amongst themselves* complex.

Absent attackers, these two points are all you need for strong collision resistance.  And the xor-and-mix loop is perfectly capable of providing both.  But it does, of course, depend on having a good mixing function, discussed next.


## The mixing function.

TentHash's mixing function is identical to Skein's MIX & permute approach, except that it arranges the four `ABCD` blocks differently (but equivalently) to make SIMD implementation easier, and it uses different rotation constants.

I chose Skein's approach for a few reasons:

1. It's simple and easy to understand.
2. It (of course) satisfies the basic requirements of a good mixing function, such as being bijective and increasing entropy.
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

So for 256 bits of state I believe the Skein MIX function has at least *competitive* performance—all while being simple and easy to implement on a variety of platforms.


## The mixing function's rotation constants.

TODO


## The number of mixing rounds.

TODO


## Xoring the message length.

TODO


## The initial hash state.

TODO


## Q&A

The above sections have broadly covered the rationale behind TentHash's design.  This section answers related questions that people might have.

### Q. Why create yet another hash?  Why not just use \[hash name here\] instead?

Because all the cryptographic hashes I'm aware of are slow and/or are complex to implement, and all the non-cryptographic hashes I'm aware of with sufficiently large digest sizes for TentHash's target use cases are at least one of:

- Slow (e.g the famous FNV hash).
- Complex and/or inscrutable.
- Reliant on special hardware instructions (kind of a sub-category of complex, in the way it impacts porting).
- *Undocumented,* which in turn means that:
    1. Independent implementations have to infer how the hash works from source code.
    2. The rationale and tradeoffs of the design are unknown.

I wanted a hash that was useful as message digest, simple to implement, reasonably fast, and *documented*.  And as far as I could tell from the searching I did, no such hash existed.

It is, of course, possible that I missed one!  In which case: oops, we have an extra hash function now.


### Q. Does TentHash pass the SMHasher test suite?

Yes, trivially.  Including all truncations down to 32 bits (the smallest SMHasher will test).

With the addendum that since TentHash isn't seedable, you need to prepend (or append, either way) the seed to the input data for TentHash to pass the seeding tests.

### Q. Why the 160-bit digest size?

128 bits should generally be enough for TentHash's intended use cases.

So the reason for the 160-bit digest is simply that, after optimizing the mixing function, that's (conservatively) how many bits of reliable collision resistance the hash state has.  And I couldn't think of any reason to *not* expose that, since people can always truncate down to 128 bits if desired.

(It's a complete coincidence that 160 bits is also the digest size of famous cryptographic hashes like SHA1.  Really, it actually is a complete coincidence—it surprised me too.)


## Q. Why the 256-bit internal state size?

In short, for speed.  A 256-bit state allows us to process input data in 256-bit chunks, which is around twice as fast as processing it in 128-bit chunks.  (This is discussed in more detail in the section about the mixing function design.)

An even larger state could have been chosen, such as 512 bits, for further potential gains.  And I did experiment with that.  But in practice the speed gains were minimal without employing explicit SIMD, and it also increased the complexity of the mix function to an extent that didn't seem appropriate given TentHash's goals.

256-bits strikes a nice sweet spot between good performance and a simple implementation.


## Q. Why not process the data in multiple independent lanes for faster SIMD execution?

Some fast hashes do indeed take this approach.  And it works really well for speed: since the lanes never interact until the end, you can run them perfectly in parallel.

However, doing that reduces the collision resistance of the whole hash to that of a single lane.  For example, if your lane size is only 64-bits wide, then using the independent-lane approach results in a hash with a collision resistance equivalent to a 64-bit hash, even if the lanes are mixed into e.g. a 256-bit digest at the end.

So for smaller lane sizes that obviously wouldn't be appropriate for a message digest hash like TentHash.  And although 128-bit lanes would have been fine, in my experiments that didn't give much speed improvement because the mixing function I chose already effectively parallelizes two 128-bit lanes while still doing full 256-bit mixing.

Moreover, to be able to safely truncate a hash digest, all the bits need to be mixed well in the final digest.  And that means that using independent lanes effectively splits mixing into two separate functions: one for processing the input data, and a separate different mixing function to do the final digest mixing.  And that was added complexity I didn't want for TentHash: I wanted just one mixing function used for everything.

In other words, independent lanes harms both collision resistance *and* simplicity for only minor speed gains.

(Having said that, if performance needs to be absolutely break-neck fast for some application, there's nothing stopping anyone from running TentHash in a "striped" mode: e.g. passing every other chunk to a separate TentHash hasher, and then appending the digests and throwing those through another TentHash hasher to get the final hash.  Such an approach could presumably be e.g. AVX512 accelerated.  It would result in a different digest than plain TentHash, of course, but as long as that fact is properly documented that's not a problem.)


## Q. Why doesn't TentHash use multiple threads for faster hashing?

Because it adds more complexity than I wanted for TentHash.  And because (as with any hash function) multi-threading can be layered *on top of* TentHash instead of being built-in:

* If you have multiple items of data to process, you can simply dispatch each item to be hashed in a different thread.  That will generally be both faster and simpler than trying to use multi-threading to process a single data item.
* If you do need to process a single data item with multiple threads, either striping (as discussed in the previous section about SIMD) or a Merkle-tree construction can still be used, just with TentHash as the base hash function.  The resulting digest will be different from plain TentHash, of course, but as long as that fact is properly documented that's not a problem.
