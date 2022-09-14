# TentHash's Design Rationale

This document explains the rationale behind TentHash's design.

(Pre-1.0 this document serves not only to explain the rationale behind TentHash's design, but also the claims I think I can make about TentHash's properties, so that others can more easily rip them apart and make me look appropriately foolish.)


## Goals

TentHash is designed with the following goals, in order of priority:

1. **Strong collision resistance (without attackers).**  For all practical purposes, it should be safe to assume that different pieces of (legitimate) data will never have colliding hashes.
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

This loop accomplishes three important things:

1. It gives every input bit a good chance of affecting every output bit in the digest.  Meaning that flipping any single bit in the input will, with overwhelming probability, significantly change the output digest.  (This is also called "avalanche".)
2. It gives the relationship between all input bits a *high complexity*.  Meaning that, with overwhelming probability, the effect of flipping a single input bit can only be "cancelled out" by a complex random change elsewhere in the input, not a simple change.
3. It *strongly orders* the input chunks.  Meaning that, with overwhelming probability, swapping any two input chunks will significantly change the output digest.

Points 2 and 3 are important.  For example, you could design a hash that first xors all the input chunks, and only *afterwards* does any mixing.  And if well mixed, that would still have the first property, but it would lose both the second and third property: any bit flip can be easily cancelled out with a single additional bit flip in a different chunk, and shuffling the order of the chunks won't make any difference to the output hash.

Similarly, you could design a hash that mixes each chunk individually *before* xoring.  Such a hash would meet point 1 and—with the notable exception of chunks that happen to be identical—it would also meet point 2.  But it would still fail point 3: shuffling chunks wouldn't affect the output hash.

A xor-and-mix loop that mixes the hash state, on the other hand, accomplishes all three points.  And that provides strong collision resistance.

At least, if the mixing function is good.


## The mixing function.

TentHash's mixing function is identical to Skein's MIX & permute approach, except that it arranges the four `ABCD` blocks differently (but equivalently) to make SIMD implementation easier, and it uses different rotation constants.

I chose Skein's approach for a few reasons:

1. It (of course) satisfies the basic requirements of a good mixing function, such as being bijective, increasing entropy, and actually mixing the *whole* state together, not just siloed parts individually.
2. It's simple and easy to implement, even on platforms that aren't ideal for it.
3. It's reasonably fast: most of the operations can be done in parallel each round due to having few data dependencies, and it's also easy to vectorize.

The basis of this approach is what the Skein paper calls the "MIX" function.  It takes two unsigned 64-bit integers and mixes them together like this:

```sh
X += Y
Y = (Y <<< rotation_constant) ^ X
```

Iterating the MIX function for many rounds does a good job of mixing all the bits up both within and between `X` and `Y`.

However, on its own this only works for 128 bits of state.  To extend it to larger state sizes we still mix integers in pairs, but permute which integers are paired together each round.  The 256-bit variant of Skein (with four integers `A`, `B`, `C`, and `D`) always mixes `A` with `B` and `C` with `D`, but it swaps `B` and `D` every round which effectively changes which integer `A` and `C` are paired with.

TentHash does something different but equivalent: we always mix `A` with `C` and `B` with `D`, but swap `C` and `D` every round.  This has no effect on the mixing quality compared to Skein, but makes it easier to vectorize.

The entire TentHash mixing function, then, looks like this:

```sh
for round in number_of_rounds:
    A += C
    B += D
    C = (C <<< rotation_constant_1) ^ A
    D = (D <<< rotation_constant_2) ^ B
    swap(C, D)
```

But there are still those pesky rotation constants left to define.


## The mixing function's rotation constants.

Skein uses the following eight pairs of rotation constants for its 256-bit variant:

```
14 16
52 57
23 40
 5 37
25 33
46 12
58 22
32 32
```

It cycles through these constants, wrapping back to the start after the last pair.  They were optimized to maximize average diffusion, and they achieve full diffusion in 9 rounds.

However, TentHash doesn't use Skein's constants.  Instead, TentHash uses the following six pairs of rotation constants:

```
31 25
 5 48
20 34
21 57
11 41
18 33
```

These still achieve full diffusion in 9 rounds, but are optimized for a different metric.  Instead of optimizing to maximize *average* diffusion, these constants are optimized to maximize *minimum* diffusion.  That's a mouthful, and also a little ambiguous, so let's walk through exactly what I mean by that.

TODO


## The number of mixing rounds.

TODO


## Xoring the message length.

TODO


## The initial hash state.

TODO


## Q&A

The above sections have broadly covered the rationale behind TentHash's design.  This section answers related questions that people might have.

### Q. Why yet another hash?  Why not just use \[hash name here\] instead?

Because all the cryptographic hashes I'm aware of are slow and/or are complex to implement, and all the non-cryptographic hashes I'm aware of with sufficiently large digest sizes are at least one of:

- Slow (e.g the famous FNV hash).
- Complex and/or inscrutable.
- Reliant on special hardware instructions (kind of a sub-category of complex, in how it impacts porting).
- *Undocumented,* which in turn means that:
    - Independent implementations have to infer how the hash works from source code.
    - The rationale and tradeoffs of the design are unknown.

I wanted a hash that was useful as a message digest, simple to implement, reasonably fast, and *documented*.  And as far as I was able to find, no such hash existed.

It's possible, of course, that I missed one!  In which case: oops, we have an extra hash function now.


### Q. Does TentHash pass the SMHasher test suite?

Yes, trivially.  Including all power-of-two truncations down to 32 bits (the smallest SMHasher will test).  But with two qualifications:

1. TentHash isn't seedable, so it of course doesn't pass the seeding tests.  But if you simply prepend (or append, either way) the seed to the input data, then TentHash passes the seeding tests as well.
2. The SMHasher suite can only test power-of-two digest sizes, and TentHash is 160 bits.  To get around this, in place of testing 160 bits I tested 256 bits by outputting the entire internal state as a digest.

(Aside: qualification 2 is interesting because we know from the analysis earlier in this document that TentHash isn't strictly 256 bits strong.  And yet it still cleanly passes SMHasher as a 256-bit hash.  This isn't because SMHasher is bad—it's not!  Rather, this is because even at just moderately sized hashes it's impossible to fully test a hash function for issues.  In other words, SMHasher is useful and hashes ought to pass it, but don't design your hashes against it.  Something something Goodhart's law.)


### Q. Why the 160-bit digest size?

160 bits actually exceeds the design target I had for TentHash, which was 128 bits.

The reason for the 160-bit digest is simply that, after optimizing the mixing function, that's (conservatively) how many bits of reliable collision resistance the finalized hash state has.  And since people can always truncate down to 128 bits if desired, there isn't much reason to *not* provide the full 160 bits.

(It's honestly just a weird coincidence that 160 bits is also the digest size of e.g. SHA1.)


### Q. Why the 256-bit internal state size?

In my testing, 256 bits struck a good balance between having good performance and having a simple implementation.

128 bits resulted in a slightly simpler implementation, but also significantly reduced performance.

512 bits, on the other hand, notably increased the implementation complexity.  And although it certainly has the potential for performance gains with a wide SIMD implementation, a straightforward scalar implementation decreased performance compared to 256 bits.

So given TentHash's goals, 256 bits felt like it hit a good sweet spot.


### Q. Why not process the data in multiple independent lanes for faster SIMD execution?

Some fast hashes do indeed take this approach.  And it works really well for improving performance: since the lanes never interact until the end, you can run them perfectly in parallel.

However, doing that reduces the collision resistance of the whole hash to that of a single lane.  For example, if your lane size is only 64-bits wide, then using the independent-lane approach results in a hash with a collision resistance that's equivalent to a 64-bit hash.  And this is true even if the lanes are mixed into e.g. a 256-bit digest at the end.  So with small lane sizes the independent-lane approach isn't appropriate for a message digest hash like TentHash.

Using 128-bit lanes would be fine.  But in my experiments that didn't give much speed improvement over the Skein approach, because the Skein approach is already pretty good at parallelizing the work.  And it does full 256-bit mixing.

It's also a little simpler implementation-wise to just have one way of mixing the hash state, rather than having both the parallel-way while mixing the input data and the serial way when mixing the lanes together at the end.


### Q. Why doesn't TentHash use multiple threads for faster hashing?

Because it adds more complexity than I wanted for TentHash.  And because (as with any hash function) multi-threading can be layered *on top of* TentHash in applications that need it:

* If you have multiple items of data to process, you can simply dispatch each item to be hashed in a different thread.  That's a lot simpler than trying to use multi-threading to process a single data item, and as long as you have enough data items to keep the threads busy it should be about as fast.
* If you do need to process a single data item with multiple threads, either using striping or a Merkle-tree construction with TentHash as the base hash function should work just fine.  The resulting digest will be different from plain TentHash, of course.  But as long as that fact is properly documented that's not a problem.
