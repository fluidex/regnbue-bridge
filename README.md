# Regnbue Bridge

Regnbue Bridge is the L1 <-> L2 fast bridge for FluiDex, helps teleport fund for users.

When a user needs a fast withdrawal, they can withdraw to a trusted operator in L2, and the operator will finish the withdrawal for the user in a faster way than a normal withdrawal.

"Regnbue" means "rainbow" in Danish.

## Configuration

You can find sample config in `config` folder. Block-submitter needs an eth account to operate.
Thus, a `geth` styled keystore is need. You can found it in `block_submitter.yaml.template`.