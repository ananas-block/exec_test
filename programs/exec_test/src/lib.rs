use anchor_lang::prelude::*;

declare_id!("DUUot4WvRmSkRgKmpm7VWgQMkNbqcNa9QLkNr7ueTio4");

#[program]
pub mod exec_test {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {
    // /// CHECK:` doc comment explaining why no checks through types are necessary.
    // signer: AccountInfo<'info>,
    // /// CHECK:` doc comment explaining why no checks through types are necessary.
    // payer: AccountInfo<'info>,
}
