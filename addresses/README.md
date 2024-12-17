# Addresses

Examples of using addresses in **`MultiTest`**. There are following address categories:

- [user address](#user-address),
- [contract address](#contract-address),
- [validator address](#validator-address).

## User address

#### Blockchain behavior

On blockchain, user addresses are derived from the public key of the user as follows:
- bytes of the public key are hashed using SHA-256,
- resulting bytes of SHA-256 hash are hashed again using RIPEMD-160,
- resulting bytes are converted to Bech32 format with blockchain's prefix.

Example:

1. Public key encoded in Base64, as displayed by the command `chaind keys --list`, where `chaind` is the CLI daemon of the chain:

    ```text
    - address: cosmos10w323vd8hm8adp3kus6qp99yrs86mj4kh2ruhg
      name: alice
      pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"AqAWno6jFXEqE3MC5v9BObuBkHIyrRORXDtNReuPHo/s"}'
      type: local
    ```
    
    Public key = `AqAWno6jFXEqE3MC5v9BObuBkHIyrRORXDtNReuPHo/s`.

2. Conversion to SHA-256 gives the following bytes (hex):

    ```text
    e4ce72c758f2296e557800b4db6470eca3cdb6459aa7cfcd6d38a7a95c8db1d5
    ```
   
3. Conversion using RIPEMD-160 of the SHA-256 hash (hex):

    ```text
    7ba2a8b1a7becfd68636e4340094a41c0fadcab6
    ```
   
4. Conversion to Bech32 format with `cosmos` prefix:

    ```text
    cosmos10w323vd8hm8adp3kus6qp99yrs86mj4kh2ruhg
    ```

#### MultiTest behaviour 

Generating a pair of keys for each user address used in tests in **`MultiTest`** could be little cumbersome.  
User addresses in **`MultiTest`** are created based on any string (user alias) provided by the test writer.
Provided string is hashed using SHA-256 and then converted to Bech32 with blockchain's prefix.  


Example:

1. User alias:

    ```text
    alice
    ```
2. Hashing with SHA-256 (hex):

    ```text
    2bd806c97f0e00af1a1fc3328fa763a9269723c8db8fac4f93af71db186d6e90
    ``` 
3. Conversion to Bech32 format with `cosmos` prefix:

    ```text
    cosmos190vqdjtlpcq27xslcveglfmr4ynfwg7gmw86cnun4acakxrdd6gq7g6jwn
    ``` 

### Legend

- CW_PREFIX = `"cosmwasm"`;
- CW_BECH32 = `"cosmwasm1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqs2g053y"`;
- CW_BECH32M = `"cosmwasm1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqsl5lc5x"`;
- NB_PREFIX = `"nebula"`;
- NB_BECH32 = `"nebula1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqsvsqrvp"`;
- NB_BECH32M = `"nebula1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqsevs0fr"`;

### Full matrix

```text
┌───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┐
│(1)│      (2)       │          (3)          │       (4)      │    (5)    │    (6)     │(7)│
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_app_api.rs                                                                          │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤
│ 1 │ app.api()      │ App::default()        │       -        │     -     │ CW_BECH32  │ T │
│ 2 │ app.api()      │ App::default()        │       -        │     -     │ CW_BECH32M │ N │
│ 3 │ app.api()      │ AppBuilder.with_api() │ MockApiBech32  │ CW_PREFIX │ CW_BECH32  │ T │
│ 4 │ app.api()      │ AppBuilder.with_api() │ MockApiBech32m │ CW_PREFIX │ CW_BECH32M │ T │
│ 5 │ app.api()      │ AppBuilder.with_api() │ MockApiBech32  │ NB_PREFIX │ NB_BECH32  │ T │
│ 6 │ app.api()      │ AppBuilder.with_api() │ MockApiBech32m │ NB_PREFIX │ NB_BECH32M │ T │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_into_addr.rs                                                                        │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤
│ 1 │ into_addr()    │          -            │       -        │     -     │ CW_BECH32  │ T │
│ 2 │ into_addr()    │          -            │       -        │     -     │ CW_BECH32M │ N │
│ 3 │ into_addr()    │          -            │       -        │ CW_PREFIX │ CW_BECH32  │ T │
│ 4 │ into_addr()    │          -            │       -        │ CW_PREFIX │ CW_BECH32M │ N │
│ 5 │ into_addr()    │          -            │       -        │ NB_PREFIX │ NB_BECH32  │ T │
│ 6 │ into_addr()    │          -            │       -        │ NB_PREFIX │ NB_BECH32M │ N │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_into_bech32.rs                                                                      │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤ 
│ 1 │ into_bech32()  │          -            │       -        │     -     │ CW_BECH32  │ T │
│ 2 │ into_bech32()  │          -            │       -        │     -     │ CW_BECH32M │ N │
│ 3 │ into_bech32()  │          -            │       -        │ CW_PREFIX │ CW_BECH32  │ T │
│ 4 │ into_bech32()  │          -            │       -        │ CW_PREFIX │ CW_BECH32M │ N │
│ 5 │ into_bech32()  │          -            │       -        │ NB_PREFIX │ NB_BECH32  │ T │
│ 6 │ into_bech32()  │          -            │       -        │ NB_PREFIX │ NB_BECH32M │ N │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_into_bech32m.rs                                                                     │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤ 
│ 1 │ into_bech32m() │          -            │       -        │     -     │ CW_BECH32  │ N │
│ 2 │ into_bech32m() │          -            │       -        │     -     │ CW_BECH32M │ T │
│ 3 │ into_bech32m() │          -            │       -        │ CW_PREFIX │ CW_BECH32  │ N │
│ 4 │ into_bech32m() │          -            │       -        │ CW_PREFIX │ CW_BECH32M │ T │
│ 5 │ into_bech32m() │          -            │       -        │ NB_PREFIX │ NB_BECH32  │ N │
│ 6 │ into_bech32m() │          -            │       -        │ NB_PREFIX │ NB_BECH32M │ T │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_mock_api.rs                                                                         │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤ 
│ 1 │ MockApi        │ MockApi::default      │       -        │     -     │ CW_BECH32  │ T │
│ 2 │ MockApi        │ MockApi::default      │       -        │     -     │ CW_BECH32M │ N │
│ 3 │ MockApi        │ MockApi::with_prefix  │       -        │ CW_PREFIX │ CW_BECH32  │ T │
│ 4 │ MockApi        │ MockApi::with_prefix  │       -        │ CW_PREFIX │ CW_BECH32M │ N │
│ 5 │ MockApi        │ MockApi::with_prefix  │       -        │ NB_PREFIX │ NB_BECH32  │ T │
│ 6 │ MockApi        │ MockApi::with_prefix  │       -        │ NB_PREFIX │ NB_BECH32M │ N │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_mock_api_bech32.rs                                                                  │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤ 
│ 1 │ MockApiBech32  │          -            │       -        │     -     │ CW_BECH32  │ N │
│ 2 │ MockApiBech32  │          -            │       -        │     -     │ CW_BECH32M │ N │
│ 3 │ MockApiBech32  │          -            │       -        │ CW_PREFIX │ CW_BECH32  │ T │
│ 4 │ MockApiBech32  │          -            │       -        │ CW_PREFIX │ CW_BECH32M │ N │
│ 5 │ MockApiBech32  │          -            │       -        │ NB_PREFIX │ NB_BECH32  │ T │
│ 6 │ MockApiBech32  │          -            │       -        │ NB_PREFIX │ NB_BECH32M │ N │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_mock_api_bech32m.rs                                                                 │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤ 
│ 1 │ MockApiBech32m │          -            │       -        │     -     │ CW_BECH32  │ N │
│ 2 │ MockApiBech32m │          -            │       -        │     -     │ CW_BECH32M │ N │
│ 3 │ MockApiBech32m │          -            │       -        │ CW_PREFIX │ CW_BECH32  │ N │
│ 4 │ MockApiBech32m │          -            │       -        │ CW_PREFIX │ CW_BECH32M │ T │
│ 5 │ MockApiBech32m │          -            │       -        │ NB_PREFIX │ NB_BECH32  │ N │
│ 6 │ MockApiBech32m │          -            │       -        │ NB_PREFIX │ NB_BECH32M │ T │
└───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┘
```

### Matrix reduced to existing tests cases 

```text
┌───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┐
│(1)│      (2)       │          (3)          │       (4)      │    (5)    │    (6)     │(7)│
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_app_api.rs                                                                          │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤
│ 1 │ app.api()      │ App::default()        │       -        │     -     │ CW_BECH32  │ T │
│ 3 │ app.api()      │ AppBuilder.with_api() │ MockApiBech32  │ CW_PREFIX │ CW_BECH32  │ T │
│ 4 │ app.api()      │ AppBuilder.with_api() │ MockApiBech32m │ CW_PREFIX │ CW_BECH32M │ T │
│ 5 │ app.api()      │ AppBuilder.with_api() │ MockApiBech32  │ NB_PREFIX │ NB_BECH32  │ T │
│ 6 │ app.api()      │ AppBuilder.with_api() │ MockApiBech32m │ NB_PREFIX │ NB_BECH32M │ T │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_into_addr.rs                                                                        │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤
│ 1 │ into_addr()    │          -            │       -        │     -     │ CW_BECH32  │ T │
│ 3 │ into_addr()    │          -            │       -        │ CW_PREFIX │ CW_BECH32  │ T │
│ 5 │ into_addr()    │          -            │       -        │ NB_PREFIX │ NB_BECH32  │ T │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_into_bech32.rs                                                                      │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤ 
│ 1 │ into_bech32()  │          -            │       -        │     -     │ CW_BECH32  │ T │
│ 3 │ into_bech32()  │          -            │       -        │ CW_PREFIX │ CW_BECH32  │ T │
│ 5 │ into_bech32()  │          -            │       -        │ NB_PREFIX │ NB_BECH32  │ T │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_into_bech32m.rs                                                                     │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤ 
│ 2 │ into_bech32m() │          -            │       -        │     -     │ CW_BECH32M │ T │
│ 4 │ into_bech32m() │          -            │       -        │ CW_PREFIX │ CW_BECH32M │ T │
│ 6 │ into_bech32m() │          -            │       -        │ NB_PREFIX │ NB_BECH32M │ T │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_mock_api.rs                                                                         │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤ 
│ 1 │ MockApi        │ MockApi::default      │       -        │     -     │ CW_BECH32  │ T │
│ 3 │ MockApi        │ MockApi::with_prefix  │       -        │ CW_PREFIX │ CW_BECH32  │ T │
│ 5 │ MockApi        │ MockApi::with_prefix  │       -        │ NB_PREFIX │ NB_BECH32  │ T │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_mock_api_bech32.rs                                                                  │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤ 
│ 3 │ MockApiBech32  │          -            │       -        │ CW_PREFIX │ CW_BECH32  │ T │
│ 5 │ MockApiBech32  │          -            │       -        │ NB_PREFIX │ NB_BECH32  │ T │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_mock_api_bech32m.rs                                                                 │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤ 
│ 4 │ MockApiBech32m │          -            │       -        │ CW_PREFIX │ CW_BECH32M │ T │
│ 6 │ MockApiBech32m │          -            │       -        │ NB_PREFIX │ NB_BECH32M │ T │
└───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┘
```

### Matrix reduced to tests cases included in documentation 

```text
┌───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┐
│(1)│      (2)       │          (3)          │       (4)      │    (5)    │    (6)     │(7)│
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_app_api.rs                                                                          │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤
│ 1 │ app.api()      │ App::default()        │       -        │     -     │ CW_BECH32  │ T │
│ 4 │ app.api()      │ AppBuilder.with_api() │ MockApiBech32m │ CW_PREFIX │ CW_BECH32M │ T │
│ 5 │ app.api()      │ AppBuilder.with_api() │ MockApiBech32  │ NB_PREFIX │ NB_BECH32  │ T │
│ 6 │ app.api()      │ AppBuilder.with_api() │ MockApiBech32m │ NB_PREFIX │ NB_BECH32M │ T │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_into_addr.rs                                                                        │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤
│ 1 │ into_addr()    │          -            │       -        │     -     │ CW_BECH32  │ T │
│ 5 │ into_addr()    │          -            │       -        │ NB_PREFIX │ NB_BECH32  │ T │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_into_bech32.rs                                                                      │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤ 
│ 1 │ into_bech32()  │          -            │       -        │     -     │ CW_BECH32  │ T │
│ 5 │ into_bech32()  │          -            │       -        │ NB_PREFIX │ NB_BECH32  │ T │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_into_bech32m.rs                                                                     │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤ 
│ 2 │ into_bech32m() │          -            │       -        │     -     │ CW_BECH32M │ T │
│ 6 │ into_bech32m() │          -            │       -        │ NB_PREFIX │ NB_BECH32M │ T │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_mock_api.rs                                                                         │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤ 
│ 1 │ MockApi        │ MockApi::default      │       -        │     -     │ CW_BECH32  │ T │
│ 5 │ MockApi        │ MockApi::with_prefix  │       -        │ NB_PREFIX │ NB_BECH32  │ T │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_mock_api_bech32.rs                                                                  │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤ 
│ 3 │ MockApiBech32  │          -            │       -        │ CW_PREFIX │ CW_BECH32  │ T │
│ 5 │ MockApiBech32  │          -            │       -        │ NB_PREFIX │ NB_BECH32  │ T │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_mock_api_bech32m.rs                                                                 │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤ 
│ 4 │ MockApiBech32m │          -            │       -        │ CW_PREFIX │ CW_BECH32M │ T │
│ 6 │ MockApiBech32m │          -            │       -        │ NB_PREFIX │ NB_BECH32M │ T │
└───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┘
```

## Contract address

(WIP)

## Validator address

(WIP)
