runestone
=========

![runestone_github_banner](https://github.com/YuurinBee/runestone/assets/32799031/c395194d-f253-483e-8904-4d4352eb7a58)

Warning
-------

`runestone` implements runes, a fungible token protocol for Bitcoin.

Fungible tokens are, without exaggeration and nearly without exception, a vile
abyss of hopium, scams, and incompetence.

Runes are no different.

If you want to make money, buy bitcoin.

To Do
-----

- Decide whether or not I should do this at all. Fungible tokens are almost
  entirely meritless. However, alternative fungible token standards, like
  BRC-20, have a large on-chain footprint and have lead to a proliferation of
  UTXOs, and standards with no on-chain footprint have UX challenges and have
  been slow to see adoption.

- Finalize the varint encoding should be used. LEB128, VLQ, and a prefix varint
  are all contenders.

- Decide whether or not duplicate symbols are allowed.

- Decide if symbols should be restricted to be no longer than the most recently
  mined sat name.

- Decide whether or not the ID of an asset is a sequentally assigned ID, or
  based on the block height and transaction index of the issuance transaction.
  The former is more compact, but the latter is less ambiguous and allows for
  compact SPV proofs.

- Decide whether or not ID should be a delta, which would be more compact,
  but would require that transfers be encoded in increasing order of asset ID.

- Decide whether or not we should encode transfer outputs as
  `ceil(log2(output_count))`-length bit strings instead of varints, since we
  know the number of transaction outputs. Saves space but somewhat tricky.

- Decide whether or not an amount of 0 in a transfer should be a shorthand for
  all remaining runes.

- Write an index.

- Write a plain-english specification.

- Finalize the payload signature. In order to discourage the proliferation of
  incompatible issuance transactions, this implementation uses a push of the
  bytes `"RUNE_TEST"` after an `OP_RETURN` to indicate a rune payload. This is
  intended to be temporary, and should be changed to a final.

- Decide on an activation height. The degens will surely REEEEEE loudly if
  there are any shenanigans around launch of the protocol. The best way to
  avoid this is to define an activation height well into the future.
