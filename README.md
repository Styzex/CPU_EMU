# P1831 CPU Emulator

- This emulator is just a fun project I've made for learning purposes
- It runs at roughly 30% speed of your actual CPU
- The P1831 is an 8-bit CPU that I designed. It is intentionally very simple.

## Versions

1. **release** - Maximum performance, zero safety checks.
2. **debug** - Performs runtime validation and reports invalid operations.

## Code examples

```ASM
MOV R1 0xFF
MOV R2 0x01
SUB R1 R2
JNZ R1 0x16
HLT
```

## Instruction Set Architecture (ISA)

### Constraints

- **Clock speed**: Variable
- **Emulator performance**: Varies by hardware. Approximately 1 billion simulated instructions per second in release mode on the developer's machine.
- **8-bit values only**: All values are unsigned integers (0-255)
- **2 registers**: R1 and R2 are general purpose registers and are the only available registers
- **Memory layout**:
  - Addresses 0-15 contain your data. This section works as the memory for your code to use.
  - Addresses 16-63 contain program instructions. Writing to this region modifies the running program.

  > Programs may write to any address (0-63) at runtime.
  > Writing to addresses 16-63 modifies the program itself, allowing self-modifying code.

- **64 bytes total memory**: Addresses 0-63
- **Program start address**: Execution begins at address 16 (0x10)

### Instructions

#### Jump Instructions

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

#### Memory Instructions

- **LOD** - Load from memory into register
  - Syntax: `LOD register address`
  - Example: `LOD R1 0x00` (load memory[0] into R1)

- **STR** - Store register value to memory
  - Syntax: `STR register address`
  - Example: `STR R1 0x00` (store R1 into memory[0])

#### Data Instructions

- **MOV** - Move immediate value into register
  - Syntax: `MOV register value`
  - Example: `MOV R1 0x05` (set R1 = 5)

#### Arithmetic Instructions

- **ADD** - Add registers
  - Syntax: `ADD register_dest register_src`
  - Operation: `register_dest = register_dest + register_src`
  - Example: `ADD R1 R2` (R1 = R1 + R2)
  - Note: Overflow (result>255) causes an error

- **SUB** - Subtract registers
  - Syntax: `SUB register_dest register_src`
  - Operation: `register_dest = register_dest - register_src`
  - Example: `SUB R1 R2` (R1 = R1 - R2)
  - Note: Underflow (result<0) causes an error

#### System Instructions

- **HLT** - Halt execution
  - Syntax: `HLT`
  - Stops the processor

### Syntax Rules

1. Tokens must be separated by a single space.
2. Instructions and registers must be in **UPPERCASE**
3. Values are hexadecimal `0x05`

```
operator operand operand
```

### Instruction Encoding

Each instruction component (opcode, register, address, or immediate value) occupies 1 byte.

Example:

```
ADD R1 R2

Memory layout:
[ADD][R1][R2]

Total size: 3 bytes
```

#### Opcodes and Encodings

| Instruction | Encoding |
| ----------- | -------- |
| JMP         | 0x01     |
| JZ          | 0x02     |
| JNZ         | 0x03     |
| LOD         | 0x04     |
| STR         | 0x05     |
| MOV         | 0x06     |
| ADD         | 0x07     |
| SUB         | 0x08     |
| HLT         | 0xFF     |

| Register | Encoding |
| -------- | -------- |
| R1       | 0xF0     |
| R2       | 0xF1     |
