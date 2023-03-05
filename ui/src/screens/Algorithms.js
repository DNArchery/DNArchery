import * as React from 'react'
import MultipleInput from '../components/Input/MultipleInput';
import PlaceHolder from '../components/PlaceHolder/PlaceHolder'
import './Algorithms.css';

const Algorithms = (props) => {

    const { state, onChange } = props;
    const [active, setActive] = React.useState(state.active || false);
    const [needleman, setN] = React.useState(state.needleman || '');
    const [smith, setSmith] = React.useState(state.smith || '');
    const [sparse, setS] = React.useState(state.sparse || '');

    React.useEffect(() => {
        if (needleman || smith || sparse) setActive(true);
        else setActive(false);
    }, [needleman, smith, sparse])

    React.useEffect(() => {
        onChange({ active, needleman, smith, sparse });
    }, [active, needleman, smith, sparse])

    const Result = ({ label, score, a, b, match }) => {
        return <div style={{ width: '100%', flex: 1 }} >
            <div style={{ display: 'flex' }}>
                <div style={{ flex: 1, display: 'flex' }}>
                    <p>{label}</p>
                </div>
                <div style={{ flex: 1, display: 'flex', justifyContent: 'flex-end' }}>
                    <p>Score: {score}</p>
                </div>
            </div>
            {match ? <textarea readOnly style={{fontSize: '20px'}} value={`𝗠𝗮𝘁𝗰𝗵 𝗣𝗮𝘁𝗵: ${match}`}></textarea> 
            :<textarea readOnly value={a ? `𝗔𝗹𝗶𝗴𝗻𝗺𝗲𝗻𝘁 𝗔 :: ${a} \n\n𝗔𝗹𝗶𝗴𝗻𝗺𝗲𝗻𝘁 𝗕 :: ${b}` : ''}></textarea>}
        </div>
    }

    const submit = ({ a, b }) => {
        fetch('http://127.0.0.1:1337/dna/needleman_wunsch', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ dna_a: a, dna_b: b })
        })
            .then(response => response.json())
            .then(data => setN(data))
            .catch(error => console.error(error));
        fetch('http://127.0.0.1:1337/dna/smith_waterman', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ dna_a: a, dna_b: b })
        })
            .then(response => response.json())
            .then(data => setSmith(data))
            .catch(error => console.error(error));
        fetch('http://127.0.0.1:1337/dna/sparse_alignment', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ dna_a: a, dna_b: b })
        }).then(response => response.json())
            .then(data => setS(data))
            .catch(error => console.error(error));
    }

    return <div style={{ flex: 1, display: 'flex' }}>
        <div style={{ flex: 1 }} />
        <MultipleInput onSubmit={result => submit({ a: result.a, b: result.b })} />
        <div className='output'>
            {active ? <div style={{ flex: 8, display: 'flex', width: '90%', flexDirection: 'column', justifyContent: 'center' }}>
                <Result label={'Needleman-Wunsch'} score={needleman.score} a={needleman.alignment_a} b={needleman.alignment_b} />
                <Result label={'Smith-Waterman'} score={smith.score} a={smith.alignment_a} b={smith.alignment_b} />
                <Result label={'Sparse Alignment'} score={sparse.score} match={JSON.stringify(sparse.match_path)} />
            </div>
                : <PlaceHolder />}

        </div>
        <div style={{ flex: 1 }} />
    </div>
}

export default Algorithms;
