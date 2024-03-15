# Instruction guessing thing

Tool to help me extend decompilers for architectures that aren't supported by any decompilers.

Example:
```
$ cargo run 0x8118e4b3 constraints
jal [31,27]:00011 [26]:0 [25,21]:00100 [20,16]:10000 [15,0]:0010011111001101
```
Program prints out which instructions defined in the csv file called "constraints" match the address

Constraints is based on the [mips16e specification](https://s3-eu-west-1.amazonaws.com/downloads-mips/documents/MD01172-2B-MIPS16e2-AFP-01.00.pdf)

For now it only supports 32 bit but later implement 16 and 64 bit

```
program <hex> <file with shorthand of constraints>
```
