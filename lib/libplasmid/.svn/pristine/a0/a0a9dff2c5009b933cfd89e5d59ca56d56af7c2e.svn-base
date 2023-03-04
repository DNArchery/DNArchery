#[macro_export]
macro_rules! rna_partial_codon {
    ($a:ident) => {
        crate::rna::RnaPartialCodon::new(Some(RnaNucleoBase::$a), None, None)
    };
    ($a:ident $b:ident) => {
        crate::rna::RnaPartialCodon::new(Some(RnaNucleoBase::$a), Some(RnaNucleoBase::$b), None)
    };
    ($a:ident $b:ident $c:ident) => {
        crate::rna::RnaPartialCodon::new(
            Some(RnaNucleoBase::$a),
            Some(RnaNucleoBase::$b),
            Some(RnaNucleoBase::$c),
        )
    };
}

#[macro_export]
macro_rules! rna_codon {
    ($a:ident $b:ident $c:ident) => {
        crate::rna::RnaCodon($a, $b, $c)
    };
}
