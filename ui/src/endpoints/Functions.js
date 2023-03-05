const fasta_lorf = (fasta_file_path) => {
    var response = fetch('http://127.0.0.1:1337/fasta/lorf', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            path: fasta_file_path
        })
    })
        .then(response => {
            return response.json()
        })
        .then(data => setSequence(data.amino_acids))
        .catch(error => console.error(error));

    return response;
}

const dna_random_sequence = (length) => {
    var response = fetch('http://127.0.0.1:1337/sequence/random', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            length: length
        })
    })
        .then(response => {
            return response.json()
        })
        .then(data => setSequence(data.amino_acids))
        .catch(error => console.error(error));

    return response;
}

const dna_to_amino_acids = (dna_string) => {
    var response = fetch('http://127.0.0.1:1337/sequence/random', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            dna: dna_string
        })
    })
        .then(response => {
            return response.json()
        })
        .then(data => setSequence(data.amino_acids))
        .catch(error => console.error(error));

    return response;
}

const nucleotide_at_index = (sequence, index) => {
    var response = fetch('http://127.0.0.1:1337/sequence/nucleotide_at_index', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            sequence: sequence,
            index: index
        })
    })
        .then(response => {
            return response.json()
        })
        .then(data => setSequence(data.amino_acids))
        .catch(error => console.error(error));

    return response;
}

const codon_frames = (sequence) => {
    var response = fetch('http://127.0.0.1:1337sequence/codon_frames', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            sequence: sequence
        })
    })
        .then(response => {
            return response.json()
        })
        .then(data => setSequence(data.amino_acids))
        .catch(error => console.error(error));

    return response;
}

const sequence_lorf = (sequence) => {
    var response = fetch('http://127.0.0.1:1337sequence/lorf', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            sequence: sequence
        })
    })
        .then(response => {
            return response.json()
        })
        .then(data => setSequence(data.amino_acids))
        .catch(error => console.error(error));

    return response;
}

const dna_to_protein = (dna_string) => {
    var response = fetch('http://127.0.0.1:1337/dna/to_protein', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            dna: dna_string
        })
    })
        .then(response => {
            return response.json()
        })
        .then(data => setSequence(data.amino_acids))
        .catch(error => console.error(error));

    return response;
}

// returns Blob object
const dna_to_svg = (dna_string) => {
    var response = fetch('http://127.0.0.1:1337/dna/circular_svg', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            dna: dna_string
        })
    })
        .then(response => {
            return response.blob()
        })
        .then(data => setSequence(data.amino_acids))
        .catch(error => console.error(error));

    return response;
}

const dna_needleman_wunsch = (dna_a, dna_b) => {
    var response = fetch('http://127.0.0.1:1337/dna/needleman_wunsch', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            dna_a: dna_a,
            dna_b: dna_b
        })
    })
        .then(response => {
            return response.json()
        })
        .then(data => setSequence(data.amino_acids))
        .catch(error => console.error(error));

    return response;
}

const dna_kmer_substring = (dna_a, dna_b) => {
    var response = fetch('http://127.0.0.1:1337/dna/kmer_substring', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            dna_a: dna_a,
            dna_b: dna_b
        })
    })
        .then(response => {
            return response.json()
        })
        .then(data => setSequence(data.amino_acids))
        .catch(error => console.error(error));

    return response;
}

const dna_ndiffs = (dna_a, dna_b) => {
    var response = fetch('http://127.0.0.1:1337/dna/ndiffs', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            dna_a: dna_a,
            dna_b: dna_b
        })
    })
        .then(response => {
            return response.json()
        })
        .then(data => setSequence(data.amino_acids))
        .catch(error => console.error(error));

    return response;
}

const dna_hamming_distance = (dna_a, dna_b) => {
    var response = fetch('http://127.0.0.1:1337/dna/hamming_distance', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            dna_a: dna_a,
            dna_b: dna_b
        })
    })
        .then(response => {
            return response.json()
        })
        .then(data => setSequence(data.amino_acids))
        .catch(error => console.error(error));

    return response;
}

const dna_levenshtein_distance = (dna_a, dna_b) => {
    var response = fetch('http://127.0.0.1:1337/dna/levenshtein_distance', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            dna_a: dna_a,
            dna_b: dna_b
        })
    })
        .then(response => {
            return response.json()
        })
        .then(data => setSequence(data.amino_acids))
        .catch(error => console.error(error));

    return response;
}

const dna_sparse_alignment = (dna_a, dna_b) => {
    var response = fetch('http://127.0.0.1:1337/dna/sparse_alignment', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            dna_a: dna_a,
            dna_b: dna_b
        })
    })
        .then(response => {
            return response.json()
        })
        .then(data => setSequence(data.amino_acids))
        .catch(error => console.error(error));

    return response;
}

const dna_smith_waterman = (dna_a, dna_b) => {
    var response = fetch('http://127.0.0.1:1337/dna/smith_waterman', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            dna_a: dna_a,
            dna_b: dna_b
        })
    })
        .then(response => {
            return response.json()
        })
        .then(data => setSequence(data.amino_acids))
        .catch(error => console.error(error));

    return response;
}