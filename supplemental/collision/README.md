This program demonstrates how straightforward it is to intentionally create hash collisions in TentHash.  It generates multiple pieces of data that all hash to a specified hash output.  TentHash should *not* be used in security-sensitive code.

Despite this, TentHash is nevertheless very robust against collisions for any data that is not intentionally engineered to collide, and is well suited to non-security-sensitive contexts.  See TentHash's design rationale document for further explanation.
