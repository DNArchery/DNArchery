pub trait Codon<B> {
    fn from_triplet_arr(triplet: [B; 3]) -> Self;
    fn to_triplet_arr(&self) -> [B; 3];

    fn anticodon(&self) -> Self;
}

macro_rules! impl_codon_traits {
    ($base: ty => $codon: ty) => {
        impl crate::traits::Codon<$base> for $codon {
            fn from_triplet_arr(triplet: [$base; 3]) -> $codon {
                Self { triplet }
            }

            fn to_triplet_arr(&self) -> [$base; 3] {
                self.triplet
            }

            fn anticodon(&self) -> Self {
                Self::from_triplet_arr(self.to_triplet_arr().map(|base| base.complement()))
            }
        }

        impl<T> crate::traits::TryFromStr<'_, T> for $codon
        where
            T: AsRef<str>,
        {
            fn try_from_str(value: T) -> anyhow::Result<Self> {
                use crate::err::PlasmidError;
                use crate::traits::{Codon, TryFromLetter};
                let chars = value.as_ref().chars().collect::<Vec<_>>();
                if chars.len() != 3 {
                    bail!(PlasmidError::InvalidNucleotideSequence {
                        nucleotide_type: <$base>::nucleotide_type(),
                        seq: value.as_ref().to_string()
                    })
                }
                let mut nucleobase_codes: Vec<$base> = Vec::with_capacity(3);
                for char in chars {
                    nucleobase_codes.push(TryFromLetter::try_from_letter(char)?)
                }
                let arr = nucleobase_codes.as_slice().try_into().unwrap();
                Ok(Codon::from_triplet_arr(arr))
            }
        }

        impl ToString for $codon {
            fn to_string(&self) -> String {
                use crate::traits::ToLetter;
                self.to_triplet_arr()
                    .iter()
                    .map(|base| base.to_letter())
                    .collect()
            }
        }

        impl Into<$codon> for [$base; 3] {
            fn into(self) -> $codon {
                <$codon>::from_triplet_arr(self)
            }
        }
    };
}
