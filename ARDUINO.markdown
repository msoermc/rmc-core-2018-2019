# Arduino Communications Standard
## Interface
Data transmissions occur via serial (UART) with a Baud rate of 115200.

Checksums are calculated by taking the base 256 modulus of the sum of
all bytes in the message following the checksum, including spacers.

10 is not a valid command ID. This decision was made to prevent
ambiguity with the header series.

## Format
All numbers are given as hex.

_      |Header   | Checksum | Spacer | Command | Spacer | Part 1 | Spacer | Part 2
-------|---------|----------|--------|---------|--------|--------|--------|------
Format |0a:0a:0a | X        | 0d     | X       | 0d     | X:X    | 0d     | X:X
Bytes  |3        | 1        | 1      | 1       | 1      | 2      | 1      | 2

Command | Name   | Part 1     | Part 2
--------|--------|------------|-------
1       | Drive  | {LEFT}     | {Right}
2       | Height | {VELOCITY} | Unused
3       | Dumper | {VELOCITY} | Unused
4       | Digger | {VELOCITY) | Unused

The velocities (including for the drive command) are provided as signed
16 bit integers with the domain \[-1000..1000], with -1000 being full
reverse and 1000 being full forward. Forwards indicates exactly what you
would expect for the drive train, a motion towards dumping and a motion
towards digging.

## Rationale
This protocol was designed to carry a high volume of fixed-size data
and have unambiguous message start signals. The presence of the spacers
means that the header series will always indicate the beginning of a
message, unless a hardware fault results in the transmission of a faulty
message, which the checksum will serve to mitigate.