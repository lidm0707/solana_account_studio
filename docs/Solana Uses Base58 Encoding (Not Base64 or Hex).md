Solana uses Base58 to represent public keys, wallet addresses, and signatures because it’s:

✅ Human-Friendly

Base58 removes confusing characters like 0 (zero), O (capital o), l (lower L), and I (capital i).

Avoids reading/mistyping errors when copying an address.

✅ Shorter Than Hex

Base58 is more compact.
For example:

Hex: 64 characters

Base58: ~44 characters
→ Easier to share, display, or print.

✅ Compatible with Bitcoin Standards

Bitcoin also uses Base58 for addresses.

Solana inherits this tradition for interoperability and familiarity.

🧠 Why Exactly “58”?

Because Base58 = Base64 − 6 characters removed:

Removed characters	Reason
0, O, I, l	Look too similar
+, /	Not URL-friendly

So 64 – 6 = 58 characters → Base58

🪙 Solana Key Example (Base58)
6Bq2vQ3JkFq...RrA5UEYJrVt


Easy to read, no confusing characters.
