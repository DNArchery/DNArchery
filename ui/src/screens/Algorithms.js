import * as React from 'react'
import MultipleInput from '../components/Input/MultipleInput';
import PlaceHolder from '../components/PlaceHolder/PlaceHolder'
import './Algorithms.css';

const Algorithms = (props) => {

    const { state, onChange } = props;
    const [results, setResults] = React.useState(state.results || [])
    const [needleman, setN] = React.useState('');
    const [smith, setSmith] = React.useState('');
    const [sparse, setS] = React.useState('');

    const Result = ({ label, result }) => {
        return <div style={{ width: '100%', flex: 1 }} >
            <p>{label}</p>
            <textarea>{result}</textarea>
        </div>
    }

    const submit = ({a, b}) => {
        fetch('http://127.0.0.1:1337/dna/sparse_alignment', {
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
        <MultipleInput />
        <div className='output'>
            {/* <PlaceHolder /> */}
            <div style={{ flex: 8, display: 'flex', width: '90%', flexDirection: 'column', justifyContent: 'center' }}>
                <Result />
            </div>
        </div>
        <div style={{ flex: 1 }} />
    </div>
}

export default Algorithms;
