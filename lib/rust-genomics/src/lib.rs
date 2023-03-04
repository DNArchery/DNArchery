use rand::Rng;use std::io::{Error, ErrorKind};

// Reading fasta/gff3 files
use std::fs;
use std::fmt;
use std::cmp;
use std::io::{Write, BufReader, BufRead};

use rayon::prelude::*;
use std::sync::{Arc, Mutex};

extern crate strsim;
use strsim::{hamming, levenshtein, normalized_levenshtein, osa_distance,
             damerau_levenshtein, normalized_damerau_levenshtein, jaro,
             jaro_winkler, sorensen_dice};


pub const NUCLEOTIDE: [char;4] = ['A', 'T', 'C', 'G'];

type Index = usize;

// Maybe good for future stuff 
enum Nucleotide {
    A,
    T,
    C,
    G,
}
impl Nucleotide {
    fn to_char(&self) -> char {
        match self {
            Nucleotide::A => 'A',
            Nucleotide::T => 'T',
            Nucleotide::C => 'C',
            Nucleotide::G => 'G',
        }
    }
    fn index_to_nt(index: usize) -> Result<Nucleotide, Error> {
        match index {
            0 => Ok(Nucleotide::A),
            1 => Ok(Nucleotide::T),
            2 => Ok(Nucleotide::C),
            3 => Ok(Nucleotide::G),
            _ => Err(Error::new(ErrorKind::Other, format!["Nucleotide not found at {}.", &index])),
        }
    }
}

/// Longest open reading frame [start, stop]. Can have one or multiple.
#[derive(Debug, PartialEq, Clone)]
pub enum LORF {
    One([Index; 2]),
    Many(Vec<[Index; 2]>),
}

/// A genomic sequence is represented here.
#[derive(Debug, PartialEq, Clone)]
pub struct Sequence {
    // The actual sequence, with no whitespaces and all upperspace
    pub seq: String,
    pub lorf: Option<LORF>,
}
impl fmt::Display for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.seq)
    }
}
impl Sequence {
    /// Returns a Sequence with the seq given to them 
    /// 
    /// # Arguments
    /// 
    /// * `seq` - A String that holds the sequence itself. Must have no white space or special characters. 
    /// 
    /// # Examples
    /// ```
    /// use rust_genomics::Sequence;
    /// let epic_seq = Sequence::new("ATGATGATG".to_string());
    /// ```
    pub fn new(seq: String) -> Sequence {
        Sequence{ seq: seq.to_uppercase(), lorf: None }
    }
    // TODO: check sequence validity (e.g. is ATCG)
    fn check(&self) -> bool {
        true
    }
    /// Returns a Result type containing the nucleotide at a given index 
    /// 
    /// # Arguments
    /// 
    /// * `index` - should be within the length of the Sequence, or an Error is returned
    pub fn find_at_index(&self, index: usize) -> Result<&str, Error> {
        match self.seq.get(index..index+1) {
            Some(n) => {
                return Ok(n);
            },
            None => {
                return Err(Error::new(ErrorKind::Other, format!["Nucleotide not found at {}.", &index]));
            }
        }
    }
    /// Takes the current sequence and returns a vector of codons at 3 reading frames.
    /// 
    /// # Examples
    /// 
    /// For sequence ATCAGGCAT, there are 3 frames. By calling this function on that sequence, it returns 
    /// 
    /// 
    /// ```rust,ignore
    ///     [["ATC", "AGG", "CAT"],
    /// 
    ///     ["A", "TCA", "GGC", "AT"],
    /// 
    ///     ["AT", "CAG", "GCA", "T"]]
    /// ```
    /// 
    pub fn return_reading_frames(&self) -> Vec<Vec<&str>> {
        let mut reading_frame = vec![Vec::new(), Vec::new(), Vec::new()];
        for i in 0..3 {
            if i > 0 { 
                reading_frame[i].push(&self.seq[0..i]); 
            }
            let mut cut_seq = &self.seq[i..];
            while !cut_seq.is_empty() {
                let (codon, remaining_seq) = cut_seq.split_at(cmp::min(3, cut_seq.len()));
                reading_frame[i].push(codon);
                cut_seq = remaining_seq;
            }
        }
        reading_frame
    }
    fn return_start_stop_positions(codon_list: Vec<&str>) -> (Vec<usize>, Vec<usize>){
        let mut start_positons = Vec::new();
        let mut stop_positions = Vec::new();

        for (index, codon) in codon_list.into_iter().enumerate() {
            match codon {
                "ATG" => start_positons.push(index),
                "TAA" | "TAG" | "TGA" => stop_positions.push(index),
                _ => continue
            }
        }
        (start_positons, stop_positions)
    }
    /// Given a sequence, return the longest open reading frame (section of sequence beginning with
    ///  start codon and ending with stop codon)
    #[inline]
    pub fn find_lorf(&mut self) -> LORF {
        let reading_frames = self.return_reading_frames();
        let mut lorf_list = vec![[0, 0]];
        for frame in reading_frames {
            let (start_positons, stop_positions) = Sequence::return_start_stop_positions(frame);
            for start_index in &start_positons {
                for (i, stop_index) in stop_positions.iter().enumerate() {
                    // Condition 1: start before stop
                    if start_index >= stop_index { continue }
                    // Condition 2: no stops between start and stop index
                    if i > 0 {
                        let prev_stop_index = &stop_positions[i-1];
                        if start_index < prev_stop_index && prev_stop_index < stop_index { continue }
                    }
                    // Condition 3: Length Condition
                    let orf_length = stop_index - start_index;
                    let lorf_length = lorf_list[0][1] - lorf_list[0][0];
                    if orf_length < lorf_length { continue }
                    if orf_length > lorf_length {
                        lorf_list = vec![[*start_index, *stop_index]];
                        continue;
                    }
                    if orf_length == lorf_length {
                        lorf_list.push([*start_index, *stop_index]);
                        continue;
                    }
                }
            }
        }
        if lorf_list.len() == 1 {
            return LORF::One(lorf_list[0]);
        }
        else {
            return LORF::Many(lorf_list.to_vec());
        }
    }
    /// Given a sequence, return the longest open reading frame (section of sequence beginning with
    ///  start codon and ending with stop codon). Uses threads and concurrency.
    #[inline]
    pub fn concurrent_find_lorf(&mut self) -> LORF {
        let reading_frames = self.return_reading_frames();
        let lorf_mutex = Arc::new(Mutex::new(vec![[0, 0]]));
        let mut handles = Vec::new();

        for frame in reading_frames {
            let (start_positons, stop_positions) = Sequence::return_start_stop_positions(frame);
            let stop_positions = Arc::new(stop_positions);
            let lorf_mutex = Arc::clone(&lorf_mutex);
            let handle = std::thread::spawn(move || {
                for start_index in start_positons {
                    for (i, stop_index) in (*stop_positions).iter().enumerate() {
                        // Condition 1: start before stop
                        if start_index >= *stop_index { continue }
                        // Condition 2: no stops between start and stop index
                        if i > 0 {
                            let prev_stop_index = (*stop_positions)[i-1];
                            if start_index < prev_stop_index && prev_stop_index < *stop_index { continue }
                        }
                        // Condition 3: Length Condition
                        let orf_length = stop_index - start_index;
                        let mut lorf_list = lorf_mutex.lock().unwrap();
                        let lorf_length = lorf_list[0][1] - lorf_list[0][0];
                        if orf_length < lorf_length { continue }
                        if orf_length > lorf_length {
                            *lorf_list = vec![[start_index, *stop_index]];
                            drop(lorf_list);
                            continue;
                        }
                        if orf_length == lorf_length {
                            (*lorf_list).push([start_index, *stop_index]);
                            drop(lorf_list);
                            continue;
                        }
                    }
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        let lorf_list = &*lorf_mutex.lock().unwrap();
        if lorf_list.len() == 1 {
            return LORF::One(lorf_list[0]);
        }
        else {
            return LORF::Many(lorf_list.to_vec());
        }
    }
    /// Returns a randomly generated Sequence with the given length attribute
    /// 
    /// # Arguments
    /// 
    /// * `len` - the length of that sequence 
    /// 
    /// # Examples
    /// ```
    /// use rust_genomics::Sequence;
    /// let epic_seq = Sequence::gen_random_seq(500);
    /// ```
    pub fn gen_random_seq(len: i64) -> Sequence {    
        let mut rng = rand::thread_rng();
        let mut seq: String = String::new();
        for _ in 0..len {
            let i = rng.gen_range(0, 4);
            seq.push(NUCLEOTIDE[i]);
        }
        Sequence{seq, lorf: None}
    }
    //TODO
    pub fn return_levenshtein(string1: &String, string2: &String) -> usize {
        let mut diff = 0;
        if string1 == string2 {
            return diff
        }
        if string1.len() == 0 {
            return string2.len()
        }
        if string2.len() == 0 {
            return string1.len()
        }
        1
    }
    pub fn compare(seq1: Sequence, seq2: Sequence) -> f64 {
        normalized_levenshtein(&seq1.seq[..], &seq2.seq[..])
    }
}

#[derive(Debug, PartialEq, Clone)]
/// An entry in a fasta file is represented here
pub struct FastaRecord {
    /// The entry's header
    pub header: String,
    /// The entry's sequence
    pub sequence: Sequence,
}
impl fmt::Display for FastaRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Header: {}\nSequence: {}", self.header, self.sequence)
    }
}
impl FastaRecord {
    /// Returns a FastaRecord with the header and sequence given to them 
    pub fn new(header: String, sequence: Sequence) -> FastaRecord {
        FastaRecord{header, sequence}
    }
    pub fn compare(fr1: &FastaRecord, fr2: &FastaRecord) -> f64 {
        normalized_levenshtein(&fr1.sequence.seq[..], &fr2.sequence.seq[..])
    }
}

#[derive(Debug, PartialEq)]
/// A FASTA file is represented here
pub struct FASTA {
    /// name/path to file
    pub name: String,
    /// file content, containing a vector of records
    pub content: Vec<FastaRecord>,
}
impl fmt::Display for FASTA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "File Name: {}\n", self.name);
        for record in &self.content {
            write!(f, "Header: {}\nSequence: {}\n\n", record.header, record.sequence);
        }
        Ok(())
    }
}
impl FASTA {
    fn new(name: String, content: Vec<FastaRecord>) -> FASTA {
        FASTA{name, content}
    }
    pub fn find_lorfs(&mut self, concurrent: bool) -> Vec<LORF> {
        let mut lorfs = Vec::new();
        for record in &mut self.content {
            if record.sequence.seq.is_empty() { continue }
            let value = match concurrent {
                true => record.sequence.concurrent_find_lorf(),
                false => record.sequence.find_lorf()
            };
            lorfs.push(value);
        }
        lorfs
    }
    // TODO: make better
    pub fn compare(f1: &FASTA, f2: &FASTA) -> f64 {
        if f1 == f2 { return 1.0 }
        if f1.content.len() == 0 || f2.content.len() == 0 { return 0.0 }

        let f1_str: String = f1.content.clone().into_iter().map(|record| record.sequence.to_string()).collect::<String>();
        let f2_str: String = f2.content.clone().into_iter().map(|record| record.sequence.to_string()).collect::<String>();
        
        normalized_levenshtein(&f1_str[..], &f2_str[..])
    }
    /// Returns and generates a FASTA given a path to a .fasta file
    pub fn read_fasta(path: &str) -> FASTA {    
        let data = fs::read_to_string(path).unwrap();
        let data: Vec<&str> = data.split('>').collect();
        let mut records: Vec<FastaRecord> = Vec::new();
    
        for entry in data {
            if entry.is_empty() {continue}
            let mut entry: Vec<&str> = entry.split("\n").collect();
            let header = entry.remove(0);
            let mut sequence: String = entry.into_iter().collect();
            sequence = sequence.replace("\n", "").replace("\r", "");
    
            let sequence = Sequence::new(sequence);
            records.push(FastaRecord::new(header.to_string(), sequence));
        }
        
        FASTA::new(path.to_string(), records)
    }
    /// Returns and generates a FASTA given a path to a .fasta file (slow version)
    pub fn slow_read_fasta(path: &str) -> FASTA {    
        let file = fs::File::open(path).expect("path to file not found");
        let reader = BufReader::new(file);
        let mut records = Vec::new();
        let mut temp_header = "".to_string();
        let mut temp_seq = "".to_string();
        for (index, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            if line.is_empty() {continue}
            if line.contains('>') {
                // push all prev record, don't push for 1st line
                if index > 0 {
                    records.push(FastaRecord::new(temp_header, Sequence::new(temp_seq.to_owned())));
                }
                // start new record
                temp_header = line;
                continue;
            }
            temp_seq.push_str(&line);
        }
        // push final record
        records.push(FastaRecord::new(temp_header, Sequence::new(temp_seq.to_owned())));
        FASTA::new(path.to_string(), records)
    }
    /// Returns and generates a FASTA given a path to a .fasta file (using rayon)
    pub fn rayon_read_fasta(path: &str) -> FASTA {
        let data = fs::read_to_string(path).unwrap();
        let data: Vec<&str> = data.split('>').collect();
    
        let mut records: Vec<FastaRecord> = Vec::new();
    
        for entry in data {
            if entry.is_empty() {continue}
            let mut entry: Vec<&str> = entry.par_lines().collect();
            let header = entry.remove(0);
            let mut sequence: String = entry.into_iter().collect();
            sequence = sequence.replace("\n", "").replace("\r", "");
    
            let sequence = Sequence::new(sequence);
            records.push(FastaRecord::new(header.to_string(), sequence));
        }
        FASTA::new(path.to_string(), records)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_struct() {
        // gen correct length
        let sequence = Sequence::gen_random_seq(1000);
        assert_eq!(sequence.seq.len(), 1000);
        // find correct index
        assert![sequence.find_at_index(10).is_ok()];
        assert![sequence.find_at_index(2000).is_err()];
        // codon packing
        sequence.return_reading_frames();
        // comparison
        let my_seq = Sequence::new("ATG".to_string());
        let target_seq = Sequence::new("ATGA".to_string());
        assert_eq!(Sequence::compare(my_seq, target_seq), 0.75);
    }

    #[test]
    fn test_fasta_record_struct() {
        // gen correct length
        let record = FastaRecord::new("epic record name".to_string(), Sequence::new("ATGATGATCCGG".to_string()));
        assert_eq!(record.header, "epic record name".to_string()); 
        assert_eq!(record.sequence.seq, Sequence::new("ATGATGATCCGG".to_string()).seq); 
        
        let target_record = FastaRecord::new("epic record uwu".to_string(), Sequence::new("AGGATTATCCGG".to_string()));
        println!("{}", FastaRecord::compare(&record, &target_record));
    }

    #[test]
    fn test_fasta_struct() {
        // gen correct length
        let fasta = FASTA::read_fasta("data/haha-1.fasta");
        let record = FastaRecord::new("ENST00000555133.5 ENSG00000100591:ENST00000555133.5 cdna:protein_coding".to_string(),
         Sequence::new("TTCTTTGTTTTCCTATGGGTGAGGAATGGTTGTATGAGCAGTTGGGTTTCGGGACGCTTTTTGGGGAGAACCCGATGGAGTCTGAAGGATCTCTAAATCA\
         GGCGGAACCCACGGACGGAGAGAGATGCTTCAAATTGGTCCACGGATAAGCTGAAAACACTGTTCCTGGCAGTGCAGGTTCAAAATGAAGAAGGCAAGTGTGAGGTGACGGAAGT\
         GAGTAAGCTTGATGGAGAGGCATCCATTAACAATCGCAAAGGGAAACTTATCTTCTTTTATGAATGGAGCGTCAAACTAAACTGGACAGGTACTTCTAAGTCAGGAGTACAATAC\
         AAAGGACATGTGGAGATCCCCAATTTGTCTGATGAAAACAGCGTGGATGAAGTGGAGATTAGTGTGAGCCTTGCCAAAGATGAGCCTGACACAAATCTCGTGGCCTTAATGAAGG\
         AAGAAGGGGTGAAACTTCTAAGAGAAGCAATGGGAATTTACATCAGCACCCTCAAAACAGAGTTCACCCAGGGCATGATCTTACCTACAATGAATGGAGAGTCAGTAGACCCAGT\
         GGGGCAGCCAGCACTGAAAACTGAGGAGCGCAAGGCTAAGCCTGCTCCTTCAAAAACCCAGGCCAGACCTGTTGGAGTCAAAATCCCCACTTGTAAGATCACTCTTAAGGAAACC\
         TTCCTGACGTCACCAGAGGAGCTCTATAGAGTGTTTACCACCCAAGAGCTGGTGCAGGCCTTTACCCATGCTCCTGCAACATTAGAAGCAGACAGAGGTGGAAAGTTCCACATGG\
         TAGATGGCAACGTCTCTGGGGAATTTACTGATCTGGTCCCTGAGAAACATATTGTGATGAAGTGGAGGTTTAAATCTTGGCCAGAGGGACACTTTGCCACCATCACCTTGACCTT\
         CATCGACAAGAACGGAGAGACTGAGCTGTGCATGGAAGGTCGAGGCATCCCTGCTCCTGAGGAAGAGCGGACGCGACAGGGCTGGCAGCGGTACTACTTTGAGGGCATTAAACAG\
         ACCTTTGGCTATGGCGCACGCTTATTTT".to_string()));
        assert_eq!(fasta.name, "data/haha-1.fasta");
        assert_eq!(&fasta.content[0], &record);

        assert_eq!(FastaRecord::compare(&fasta.content[0], &record), 1.0);
        assert_eq!(FASTA::compare(&fasta, &fasta), 1.0);

    }

    #[test]
    #[ignore]
    fn lorf() {
        let mut sequence = Sequence::new("ATGGGAATGTGA".to_string());
        let lorf = sequence.find_lorf();
        match lorf {
            LORF::One(value) => assert!(value == [0, 3]),
            _ => panic!("at the disco"),
        }
        let lorf_concurrent = sequence.concurrent_find_lorf();
        match lorf_concurrent {
            LORF::One(value) => assert!(value == [0, 3]),
            _ => panic!("at the disco"),
        }
    }
    #[test]
    #[ignore]
    fn compare_lorf_methods() {
        let mut long_sequence = Sequence::gen_random_seq(10000);
        let lorf = long_sequence.find_lorf();
        println!("{:?}", lorf);
        let lorf_concurrent = long_sequence.concurrent_find_lorf();
        println!("{:?}", lorf_concurrent);
    }
    #[test]
    #[ignore]
    fn test_rayon_fasta() {
        // Yes, there's actually a gene called haha-1. It's in charge of humor.
        let fasta = FASTA::rayon_read_fasta("data/haha-1.fasta");
        println!("{}", fasta);
    }
}



