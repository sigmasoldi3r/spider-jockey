# Spider-Jockey

A simple program that emits **Typescript** classes
based on provided JSON files, tested with _truffle_
compilation output.

## What does it generate?

It generates a class file that wraps around an
`ethers.Contract` instance, and based on the generated
ABI, it wraps each contract method in a corresponding
typed method of that class, so you don't have to
go blind when calling smart contract methods.

## Example

Let's say that you have a contract called `Some.sol`:

```sol
contract Some {
  function getMagicNumberOf(address _addr) public view returns (uint256) {
    // Some illusory code here that does the magic
  }
}
```

This package will generate this class:

```ts
import ethers from "ethers";

class Some {
  constructor(private readonly contract: ethers.Contract) {}

  async getMagicNumberOf(_addr: string): number {
    // Some binding code here
  }
}
```

So you only have to do the magic of:

```ts
const contract = ...;// bind your contract
const some = new Some(contract);

const result = await some.getMagicNumberOf('some valid ethers address');
```

This makes the instance of `some` at least somewhat
type-safe, and gives you intellisense hints
when developing.

## Contributing

~~If you know about a good Typescript code emitter that
works with rust, I am looking forward to implement that also.~~

**Edit:** Since I couldn't find anything, and maintaining a code
that does copy-and-paste of strings is tedious, I've built my
own, but does only support the minimal for this program.
During the following weeks I'll abstract that to an individual
crate, so it can be reused.

There's a lot of things to enhance, like adding more
parameters to the generator, here are some:

- Where to output files
- If output them in a single file with multiple export
- Specify if the library is `ethers` or use a generic wrapper
  - That would make it more decoupled and customizable
  - Default implementation of that wrapper can be provided
- Specify if the types are wrapped around DTOs
  - Addresses for instance could be wrapped, instead of using `string`
- Specify if runtime checks are validated with 3rd party libraries
  - Like validating DTOs with `class-validator`
- Specify further error handling
- Use project files of some kind to configure the generator
  - Like a `.spiderjockeyrc` or `spider-jockey.config.ron`...

To name some. Any help is welcome.
