# Arduino Communications Standard
## Interface
Data transmissions occur via serial (UART) with a Baud rate of 115200.

Checksums are calculated by taking the base 256 modulus of the sum of
all bytes in the message following the checksum, including spacers.

10 (0x0a) is not a valid command ID. This decision was made to prevent
ambiguity with the start signal.

## Format
All numbers are given as hex.

_      | Start   | Spacer | Checksum | Command | Spacer | Part 1 | Spacer | Part 2 |
-------|---------|--------|----------|---------|--------|--------|--------|--------|
Format |0a:0a:0a | 0d     | X        | X       | 0d     | X:X    | 0d     | X:X    |
Bytes  |3        | 1      | 1        | 1       | 1      | 2      | 1      | 2      |

Command | Name   | Part 1     | Part 2
--------|--------|------------|-------
1       | Left   | {VELOCITY} | Unused
2       | Right  | {VELOCITY} | Unused
3       | Height | {VELOCITY} | Unused
4       | Dumper | {VELOCITY} | Unused
5       | Digger | {VELOCITY} | Unused
6       | FR     | {VELOCITY} | {DIRECTION}
7       | FL     | {VELOCITY} | {DIRECTION}
8       | RR     | {VELOCITY} | {DIRECTION}
9       | RL     | {VELOCITY} | {DIRECTION}


The velocities (including for the drive command) are provided as signed
16 bit integers with the domain \[-1000..1000], with -1000 being full
reverse and 1000 being full forward. Forwards indicates a forwards
motion for the drive train, a motion towards dumping for the dumper and
a motion towards digging for the digger.

## Rationale
This protocol was designed to carry a high volume of fixed-size data
and have unambiguous message start signals. The presence of the spacers
means that the start signal will always indicate the beginning of a
message, unless a hardware fault results in the transmission of a faulty
message, which the checksum will serve to mitigate.