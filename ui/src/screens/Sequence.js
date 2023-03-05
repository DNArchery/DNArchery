import * as React from 'react'
import MultipleInput from '../components/Input/MultipleInput';
import PlaceHolder from '../components/PlaceHolder/PlaceHolder'
import './Algorithms.css';

const Sequence = (props) => {

    const { state, onChange } = props;
    const [active, setActive] = React.useState(state.active || false);
    const [kmer, setKmer] = React.useState(state.kmer|| '');
    const [ndiffs, setNdiffs] = React.useState(state.ndiffs|| '');
    const [hamming, setHamming] = React.useState(state.hamming|| '');
    const [levenshtein, setLevenshtein] = React.useState(state.levenshtein|| '');

    React.useEffect(() => {
        if (kmer || ndiffs || hamming || levenshtein) setActive(true);
        else setActive(false);
    }, [kmer, ndiffs, hamming, levenshtein])

    React.useEffect(() => {
        onChange({ active, kmer, ndiffs, hamming, levenshtein });
    }, [active, kmer, ndiffs, hamming, levenshtein])

    const Result = ({ label, a}) => {
        return <div style={{ width: '100%', flex: 1 }} >
            <div style={{ display: 'flex' }}>
                <div style={{ flex: 1, display: 'flex' }}>
                    <p>{label}</p>
                </div>
            </div>
            <textarea value={a}></textarea>
        </div>
    }

    const submit = ({ a, b }) => {
        fetch('http://127.0.0.1:1337/dna/kmer_substring', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ dna: a, dna_b: b })
        })
            .then(response => response.json())
            .then(data => setKmer(data.dna))
            .catch(error => console.error(error));

            console.log(kmer);
        fetch('http://127.0.0.1:1337/dna/ndiffs', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ dna_a: a, dna_b: b })
        })
            .then(response => response.json())
            .then(data => setNdiffs(data.ndiff))
            .catch(error => console.error(error));
        fetch('http://127.0.0.1:1337/dna/hamming_distance', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ dna_a: a, dna_b: b })
        }).then(response => response.json())
            .then(data => setHamming(data.distance))
            .catch(error => console.error(error));
        fetch('http://127.0.0.1:1337/dna/levenshtein_distance', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ dna_a: a, dna_b: b })
        }).then(response => response.json())
            .then(data => setLevenshtein(data.distance))
            .catch(error => console.error(error));
    }

    return <div style={{ flex: 1, display: 'flex' }}>
        <div style={{ flex: 1 }} />
        <MultipleInput onSubmit={result => submit({ a: result.a, b: result.b })} />
        <div className='output'>
            {active ? <div style={{ flex: 8, display: 'flex', width: '90%', flexDirection: 'column', justifyContent: 'center' }}>
                <Result label={'K-Mer Sequencing'} a={kmer} />
                <Result label={'Base Position Difference'} a={ndiffs} />
                <Result label={'Hamming Distance'} a={hamming}/>
                <Result label={'Levenshtein Distance'} a={levenshtein} />
            </div>
                : <PlaceHolder />}

        </div>
        <div style={{ flex: 1 }} />
    </div>
}

export default Sequence;
