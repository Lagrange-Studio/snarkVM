// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use super::*;

#[cfg(feature = "private_key")]
impl<N: Network> TryFrom<PrivateKey<N>> for GraphKey<N> {
    type Error = Error;

    /// Derives the account graph key from an account private key.
    fn try_from(private_key: PrivateKey<N>) -> Result<Self, Self::Error> {
        Self::try_from(&private_key)
    }
}

#[cfg(feature = "private_key")]
impl<N: Network> TryFrom<&PrivateKey<N>> for GraphKey<N> {
    type Error = Error;

    /// Derives the account graph key from an account private key.
    fn try_from(private_key: &PrivateKey<N>) -> Result<Self, Self::Error> {
        // Compute sk_tag := T^sk_sig.
        let sk_tag = N::t_scalar_multiply(&private_key.sk_sig());
        // Output the graph key.
        Self::try_from(sk_tag)
    }
}

impl<N: Network> TryFrom<Group<N>> for GraphKey<N> {
    type Error = Error;

    /// Derives the account graph key from `sk_tag`.
    fn try_from(sk_tag: Group<N>) -> Result<Self> {
        // Output the graph key.
        Ok(Self { sk_tag })
    }
}

impl<N: Network> TryFrom<&Group<N>> for GraphKey<N> {
    type Error = Error;

    /// Derives the account graph key from `sk_tag`.
    fn try_from(sk_tag: &Group<N>) -> Result<Self> {
        Self::try_from(*sk_tag)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use snarkvm_console_network::Testnet3;

    type CurrentNetwork = Testnet3;

    const ITERATIONS: u64 = 1000;

    #[test]
    fn test_try_from() -> Result<()> {
        for _ in 0..ITERATIONS {
            // Sample a new graph key.
            let private_key = PrivateKey::<CurrentNetwork>::new(&mut test_crypto_rng())?;
            let candidate = GraphKey::try_from(private_key)?;

            // Check that graph key is derived correctly from `sk_tag`.
            assert_eq!(candidate, GraphKey::try_from(candidate.sk_tag())?);
        }
        Ok(())
    }
}