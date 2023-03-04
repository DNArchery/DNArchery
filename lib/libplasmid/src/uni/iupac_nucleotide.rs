use crate::{
    err::{PlasmidError, PlasmidNucleotideType},
    traits::*,
};

/// IUPAC Nucleotide
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum IupacNucleotide {
    /// Adenine
    A,
    /// Cytosine
    C,
    /// Guanine
    G,
    /// Thymine / Uracil
    T,
    /// Weak (A/T)
    W,
    /// Strong (G/C)
    S,
    /// Amino (A/C)
    M,
    /// Ketone (G/T)
    K,
    /// Purine (A/G)
    R,
    /// Pyrimidine (C/T)
    Y,
    /// Not A (CGT)
    B,
    /// Not C (AGT)
    D,
    /// Not G (ACT)
    H,
    /// Not T (ACG) / Not U (ACG)
    V,
    /// Any nucleotide (A/C/G/T)
    N,
    /// Gap (None)
    Gap,
}

impl IupacNucleotide {
    pub fn matches(&self, n: &IupacNucleotide) -> bool {
        use self::IupacNucleotide::*;
        match self {
            A => [A, W, M, R, D, H, V, N].contains(n),
            C => [C, S, M, Y, B, H, V, N].contains(n),
            G => [G, S, K, R, B, D, V, N].contains(n),
            T => [T, W, K, Y, B, D, H, N].contains(n),
            W => [A, T, W, N].contains(n),
            S => [C, G, S, N].contains(n),
            M => [A, C, M, N].contains(n),
            K => [G, T, K, N].contains(n),
            R => [A, G, R, N].contains(n),
            Y => [C, T, Y, N].contains(n),
            B => [C, G, T, B, N].contains(n),
            V => [A, C, G, V, N].contains(n),
            D => [A, G, T, D, N].contains(n),
            H => [A, C, T, H, N].contains(n),
            N => true,
            Gap => n == &Gap,
        }
    }

    pub(crate) fn all_as_str() -> &'static str {
        "ACGTWSMKRYBVDHN-"
    }

    pub(crate) fn nucleotide_type() -> PlasmidNucleotideType {
        PlasmidNucleotideType::IUPAC
    }

    pub(crate) fn is_iupac(c: &char) -> bool {
        Self::all_as_str().contains(*c)
    }
}

impl ToIupac for IupacNucleotide {
    fn to_iupac(&self) -> IupacNucleotide {
        *self
    }
}

impl Nucleotide for IupacNucleotide {
    fn complement(&self) -> Self {
        use self::IupacNucleotide::*;
        match self {
            A => T,
            T => A,
            C => G,
            G => C,
            W => W,
            S => S,
            M => K,
            K => M,
            R => Y,
            Y => R,
            B => V,
            D => H,
            H => D,
            V => B,
            N => N,
            Gap => Gap,
        }
    }
}

impl TryFromLetter for IupacNucleotide {
    fn try_from_letter(c: char) -> anyhow::Result<Self> {
        use self::IupacNucleotide::*;
        match c.to_ascii_uppercase() {
            'A' => Ok(A),
            'C' => Ok(C),
            'G' => Ok(G),
            'T' => Ok(T),
            'U' => Ok(T),
            'W' => Ok(W),
            'S' => Ok(S),
            'M' => Ok(M),
            'K' => Ok(K),
            'R' => Ok(R),
            'Y' => Ok(Y),
            'B' => Ok(B),
            'D' => Ok(D),
            'H' => Ok(H),
            'V' => Ok(V),
            'N' => Ok(N),
            '-' => Ok(Gap),
            _ => bail!(PlasmidError::InvalidNucleotide {
                nucleotide_type: Self::nucleotide_type(),
                char: c
            }),
        }
    }
}

impl ToLetter for IupacNucleotide {
    fn to_letter(&self) -> char {
        use self::IupacNucleotide::*;
        match self {
            A => 'A',
            C => 'C',
            G => 'G',
            T => 'T',
            W => 'W',
            S => 'S',
            M => 'M',
            K => 'K',
            R => 'R',
            Y => 'Y',
            B => 'B',
            D => 'D',
            H => 'H',
            V => 'V',
            N => 'N',
            Gap => '-',
        }
    }
}
