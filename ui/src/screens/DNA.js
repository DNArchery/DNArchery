import * as React from 'react'
import Input from '../components/Input/Input';
import './DNA.css';
import AminoAcid from '../components/AminoAcid/AminoAcid';
import Codon from '../components/Codon/Codon';

const DNA = (props) => {

    const { state, onChange } = props;
    const [aminoAcid, setAminoAcid] = React.useState(state.aminoAcid || [])
    const [codon, setCodon] = React.useState(state.codon || [])

    React.useEffect(() => {
        onChange({ aminoAcid });
    }, [aminoAcid]);

    const submit = (query) => {
        fetch('http://127.0.0.1:1337/dna/to_amino_acids', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ dna: query })
        })
            .then(response => response.json())
            .then(data => setAminoAcid(data.amino_acids))
            .catch(error => console.error(error));

        fetch('http://127.0.0.1:1337/sequence/codon_frames', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                sequence: query
            })
            })
            .then(response => {
                return response.json()
            })
            .then(data => setCodon(data))
            .catch(error => console.error(error));
    }

    return <div style={{ flex: 1, display: 'flex' }}>
        <div style={{ flex: 1 }} />
        <Input onSubmit={query => submit(query)} />
        <div className='output'>
            {/* {aminoAcid.length > 0 ? <AminoAcid aminoAcid={aminoAcid} /> : null} */}
            {codon.length > 0 ? <Codon codon={codon} /> : null}
        </div>
        <div style={{ flex: 1 }} />
    </div>
}

export default DNA;
