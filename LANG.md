# Documentation of the Instruction Set Architecture for the P1831 Processor
## Constraints
- **1GHz Clock speed**: It runs roughly at a 3:1 ratio
- **8-bit values only**: All values are unsigned integers (0-255)
- **2 registers**: R1 and R2 are the only available registers
- **Memory layout**:
  - Addresses 0-15: Safe data storage
  - Addresses 16-63: Code section (writing here overwrites your program!)
- **64 bytes total memory**: Addresses 0-63

## Instructions
### Jump Instructions
- **JMP** - Unconditional jump
  - Syntax: `JMP address`
  - Example: `JMP 0x10` (jump to address 16)

- **JZ** - Jump if Zero
  - Syntax: `JZ register address`
  - Jumps to the address if the specified registers value is 0
  - Example: `JZ R1 0x20`

- **JNZ** - Jump if Not Zero
  - Syntax: `JNZ register address`
  - Jumps to the address if the specified registers value is not 0
  - Example: `JNZ R1 0x20`

### Memory Instructions
- **LOD** - Load from memory into register
  - Syntax: `LOD register address`
  - Example: `LOD R1 0x00` (load memory[0] into R1)

- **STR** - Store register value to memory
  - Syntax: `STR register address`
  - Example: `STR R1 0x00` (store R1 into memory[0])

### Data Instructions
- **MOV** - Move immediate value into register
  - Syntax: `MOV register value`
  - Example: `MOV R1 0x05` (set R1 = 5)

### Arithmetic Instructions
- **ADD** - Add registers
  - Syntax: `ADD register_dest register_src`
  - Operation: `register_dest = register_dest + register_src`
  - Example: `ADD R1 R2` (R1 = R1 + R2)

- **SUB** - Subtract registers
  - Syntax: `SUB register_dest register_src`
  - Operation: `register_dest = register_dest - register_src`
  - Example: `SUB R1 R2` (R1 = R1 - R2)
  - Note: Underflow (negative results) causes an error

## Registers
### General purpose registers
- **R1**
- **R2**

## Syntax Rules
1. Every instruction, register, and value must be followed by a space
2. Instructions and registers must be in **UPPERCASE**
3. Values are hexadecimal `0x05`
4. Each operand and operator occupies 1 byte in memory
```
operator operand operand
```
