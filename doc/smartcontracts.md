# Smart Contracts

BitGreen blochain does support smart contracts written in an embedded domain specific RUST language (EDSL).  
Basically you use Rust language with some limits on the usable libraries ("crates" in Rust).  
You can use only the "no-std" crates, many crates can be configured with such feature "no-std".    
There is a good reason for this limitation: you cannot access the underlying operating system from a Smart Contract that is compiled in WASM (Web Assembly).  
The EDSL name "!Ink", in the following documentation we will refer to the language as "!Ink".  

Our smart contract platform allows users to publish additional logic on top of BitGreen blockchain logic.  
Since smart contract logic can be published by anyone, including malicious actors and inexperienced developers, 
there are a number of intentional safe guards built around these public smart contract platform:  

- Fees: Ensuring that contract developers are charged for the computation and storage they force on the computers running their contract, and not allowed to abuse the block creators.  
- Sandbox: A contract is not able to modify core blockchain storage or the storage of other contracts directly. It's power is limited to only modifying it's own state, and the ability to 
make outside calls to other contracts or runtime functions.  
- State Rent: A contract takes up space on the blockchain, and thus is charged for simply existing. This ensures that people don't take advantage of "free, unlimited storage".  
- Reversion: A contract can be prone to have situations which lead to logical errors. The expectations of a contract developer are low, so extra overhead is added to support reverting transactions when they fail so no state is updated when things go wrong.  

Smart contracts allows the BitGreen community to extend and develop on top of our blockchain and lead to further evolution for the core runtime.  


# !Ink Language
Ink! is not a new programming language, but rather adapt a subset of Rust to serve our purpose.

- Rust is an ideal smart contract language: It is type safe, memory safe, and free of undefined behaviors. It generates small binaries because it doesn’t include extra bloat, like a garbage collector, and advanced optimizations and tree shaking remove dead code. Through compiler flags, Rust can automatically protect against integer overflow.  
- Rust ecosystem: You gain from all of support available to the Rust ecosystem for free. As the language develops, ink! will automatically gain access to new features and functionality, improving how you can write smart contracts in the future.  
- Tooling: Because ink! follows Rust standards, tools like rustfmt, clippy and rust-analyzer already work out of the box. Same goes for code formatting and syntax highlighting in most modern text editors. Also Rust has an integrated test and benchmark runner.  
- No overhead: Minimal runtime.
- Safe & Efficient: Zero-cost & safe abstractions.  
- Productive: Cargo + crates.io Ecosystem. 
- 1st class Wasm: Rust provides the first class support for the WebAssembly.  
- Small Size: In the space-constrained blockchain world size is important. The Rust compiler is a great help for that, since it reorders struct fields in order to make each type as small as possible. Thus Rust data structures are very compact, in many cases even more compact than in C. 

## WebAssembly for Smart Contracts

There are good reasons to use Web Assembly for Smart Contracts:

- High performance: Wasm is high performance — it’s built to be as close to native machine code as possible while still being platform independent.  
- Small size: It facilitates small binaries to ship over the internet to devices with potentially slow internet connection. This is a great fit for the space-constrainted blockchain world.  
- General VM & bytecode: It was developed so that code can be deployed in any browser with the same result. Contrary to the EVM it was not developed towards a very specific use case, this has the benefit of a lot of tooling being available and large companies putting a lot of resources into furthering Wasm development.  
- Efficient JIT execution:64 and 32-bit integer operation support that maps one-to-one with CPU instructions.  
- Minimalistic: Formal spec that fits on a single page.  
- Deterministic execution: Wasm is easily made deterministic by removing floating point operations, which is necessary for consensus algorithms.
- Open Standards > Custom Solutions: Wasm is a standard for web browsers developed by W3C workgroup that includes Google, Mozilla, and others. There’s been many years of work put into Wasm, both by compiler and standardisation teams.  
- Many languages available: Wasm expands the family of languages available to smart contract developers to include Rust, C/C++, C#, Typescript, Haxe, and Kotlin. This means you can write smart contracts in whichever language you’re familiar with, though we’re partial to Rust due to its lack of runtime overhead and inherent security properties.  
- Memory-safe, sandboxed, and platform-independent.  
- LLVM supportSupported by the LLVM compiler infrastructure project, meaning that Wasm benefits from over a decade of LLVM’s compiler optimisation.  
- Large companies involved: Continually developed by major companies such as Google, Apple, Microsoft, Mozilla, and Facebook.  

## Reference:
- The main language documentation is kept updated at this address: [https://paritytech.github.io/ink-docs/](https://paritytech.github.io/ink-docs/).  
- A good tutorial is available here: [https://substrate.dev/substrate-contracts-workshop](https://substrate.dev/substrate-contracts-workshop).  
- You can learn by examples: [https://github.com/paritytech/ink/tree/master/examples](https://github.com/paritytech/ink/tree/master/examples).  
- Another guide from Substrate is here: [https://substrate.dev/docs/en/knowledgebase/smart-contracts/overview](https://substrate.dev/docs/en/knowledgebase/smart-contracts/overview).  
- The official repository of the !Ink Language: [https://github.com/paritytech/ink](https://github.com/paritytech/ink).  







