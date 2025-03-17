### Anchor CPI
_invoke_signed()_

#### Setup
```bash
1. anchor init program-a  cd program-b

2. anchor new program-b

3. Cargo.toml (program-a) add dependencies to program-b
-> program-b = {path = "../program-b", features= ["cpi"]}

4. Anchor.toml - set resolution to false
```

#### Run tests
```bash
anchor build && anchor test
```

#### Start Local Validator
```rust
cd .anchor/
solana-test-validator
```

#### Explorer
open explorer.solana  
switch to localhost  
find test output transaction signature


#### Expected Output
```bash
> Program logged: "Instruction: Initialize"
> Program logged: "Greetings from: Program A"
> Program invoked: System Program
> Program returned success
> Program invoked: Unknown Program (6grzSV1SpnQjt9Jf7cMtqQDUqaC7SZDXvKE5GCGrVHxZ)
> Program logged: "Instruction: Initialize"
> Program logged: "Greetings from: Progrma B"
> Program consumed: 676 of 190087 compute units
> Program returned success
> Program consumed: 10862 of 200000 compute units
> Program returned success
```
