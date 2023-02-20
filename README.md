# solana-test-validator --bpf-program flag seems to be broken

### To replicate the issue run:



```solana-test-validator --reset --bpf-program DUUot4WvRmSkRgKmpm7VWgQMkNbqcNa9QLkNr7ueTio4 ./target/deploy/exec_test.so```

then run:

```anchor test --skip-deploy --skip-local-validator --skip-build```

Results in:

```Error: failed to send transaction: Transaction simulation failed: This program may not be used for executing instructions```

In the explorer it looks like that --bpf-program deploys to the old account format, see below for screenshots.


### It works when deploying manually:


```solana-test-validator --reset```


```anchor test --skip-local-validator```




# Deployed with bpf-program flag
![Deployed with bpf-program flag](/screenshots/deploy_with_bpf_program_flag.png)

# Deployed Manually

![Deployed Manually](/screenshots/deployed_manually.png)
