# Simple Proof of Work

Finds a suitable 4-byte prefix so that, a SHA256 hash of the prefix combined with the original string of bytes, has two
last bytes as 0xCA, 0xFE. The original content of the string is expected to be passed in hexadecimal format. The result
consists of two lines, first being the SHA256 string found and second 4-byte prefix used (in hexadecimal format)
.

----

## Usage

_pow -p &lt;payload&gt;_