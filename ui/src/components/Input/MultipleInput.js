import * as React from 'react';
import { IoMdSend } from 'react-icons/io';
import { FaRandom } from 'react-icons/fa';
import FilePicker from '../FilePicker/FilePicker';
import './styles.css'

const MultipleInput = ({ onSubmit }) => {

    const [inputA, setInputA] = React.useState('');
    const [inputB, setInputB] = React.useState('');

    const getRandom = async () => {
        var result;
        try {
            result = await fetch('http://127.0.0.1:1337/sequence/random', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ length: 1337 })
            })
            result = await result.text()
            return result;
        } catch(e) {
            console.error(e)
         }
    }

    return <div className='input' >
        <div style={{ width: '80%', display: 'flex', justifyContent: 'center', flexDirection: 'column' }}>
            <div style={{ width: '120%', display: 'flex', alignItems: 'center', flexDirection: 'row', height: '8%' }}>
                <div className='search'>
                    <input type={'text'} placeholder='Enter DNA sequence A' value={inputA} onInput={(e) => setInputA(e.target.value)} />
                </div>
                <div onClick={() => onSubmit({ a: inputA, b: inputB })} className='button'>
                    <IoMdSend style={{ color: 'white', fontSize: '20px' }} />
                </div>
            </div>
            <div style={{ width: '120%', display: 'flex', marginTop: '10px', alignItems: 'center', flexDirection: 'row', height: '8%' }}>
                <div className='search'>
                    <input type={'text'} placeholder='Enter DNA sequence B' value={inputB} onInput={(e) => setInputB(e.target.value)} />
                </div>
                <div onClick={async() => {
                    setInputA(await getRandom())
                    setInputB(await getRandom())
                }} className='button'>
                    <FaRandom style={{ color: 'white', fontSize: '20px' }} />
                </div>
            </div>
            <div style={{ width: '100%', height: '80%', display: 'flex', justifyContent: 'center' }}>
                <FilePicker />
            </div>
        </div>
    </div>
}

export default MultipleInput;
