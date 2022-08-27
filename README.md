# Base

Simple tool to conver from a give **base** into any other **base**. **_You can even create your own base_**

1.  From base 2 to/from base 10
2.  From base 2 to/from base 16
3.  From base 16 to base base 10

<p align="center">
    <img src="base.png" title="Base" alt="Base"/>
</p>

```bash
[base 2] Base { base: 2, sys: ["0", "1"] }
[base 10] Base { base: 10, sys: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"] }
[base 16] Base { base: 16, sys: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F"] }
bin(10) -> 1010
b10(["1", "0", "1", "0"]) -> 10
b16(175) -> 0xAF
b10(["A", "F"]) -> 175
b16(["1", "0", "1", "0"]) -> 0xA
b2(["A"]) -> 1010
```
