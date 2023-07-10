# solana cli deployment elf verification issue

```
Error: ELF error: ELF error: Unresolved symbol (sol_alt_bn128_group_op) at instruction #16736 (ELF file offset 0x20a18
```

### To replicate the issue run:

```solana-test-validator --reset```


```anchor deploy```


```solana program deploy target/deploy/exec_test.so```
