import * as React from 'react'
import Input from '../components/Input/Input';
import PlaceHolder from '../components/PlaceHolder/PlaceHolder';
import './DNA.css';

const DNA = (props) => {

    const { state, onChange } = props;
    const [sequence, setSequence] = React.useState(state.sequence || [])

    React.useEffect(() => {
        onChange({ sequence });
      }, [sequence]);

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
            .then(data => setSequence(data.amino_acids))
            .catch(error => console.error(error));

    }

    return <div style={{ flex: 1, display: 'flex' }}>
        <div style={{ flex: 1 }} />
        <Input onSubmit={query => submit(query)} />
        <div className='output'>
            {sequence.length <= 0 ? <PlaceHolder />
                : <div style={{ flex: 1, display: 'flex', flexDirection: 'column' }}>
                    <div style={{ flex: 3 }} />
                    <div className='sequence'>
                        {sequence.map((item, index) => {
                            return <SequenceCard key={index} label={item} />
                        })}
                    </div>
                </div>}
        </div>
        <div style={{ flex: 1 }} />
    </div>
}

export default DNA;
