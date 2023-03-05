import * as React from 'react'
import Input from '../components/Input/Input';
import './DNA.css';
import AminoAcid from '../components/AminoAcid/AminoAcid';

const DNA = (props) => {

    const { state, onChange } = props;
    const [aminoAcid, setAminoAcid] = React.useState(state.aminoAcid|| [])

    React.useEffect(() => {
        onChange({ aminoAcid });
      }, [aminoAcid]);

    const SequenceCard = ({ label }) => {
        return <div className='card'>
            <p>{label}</p>
        </div>
    }

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

    }

    return <div style={{ flex: 1, display: 'flex' }}>
        <div style={{ flex: 1 }} />
        <Input onSubmit={query => submit(query)} />
        <div className='output'>
            {aminoAcid.length > 0 ? <AminoAcid aminoAcid={aminoAcid}/> : null}
        </div>
        <div style={{ flex: 1 }} />
    </div>
}

export default DNA;
