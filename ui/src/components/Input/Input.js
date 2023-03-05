import * as React from 'react';
import { IoMdSend } from 'react-icons/io';
import { FaRandom } from 'react-icons/fa';
import FilePicker from '../FilePicker/FilePicker';
import './styles.css'

const Input = ({ onSubmit }) => {

    const [input, setInput] = React.useState('');

    const getRandom = () => {
        fetch('http://127.0.0.1:1337/sequence/random', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ length: 1337 })
            })
                .then(response => response.text())
                .then(data => setInput(data))
                .catch(error => console.error(error));
    }

    return <div className='input' >
        <div style={{ width: '80%', display: 'flex', justifyContent: 'center', flexDirection: 'column' }}>
            <div style={{ width: '120%', display: 'flex', alignItems: 'center', flexDirection: 'row', height: '8%' }}>
                <div className='search'>
                    <input type={'text'} placeholder='Enter DNA sequence' value={input} onInput={(e) => setInput(e.target.value)} />
                    <IoMdSend onClick={() => onSubmit(input)} style={{ color: 'blue', fontSize: '20px' }} />
                </div>
                <div onClick={getRandom} className='button'>
                    <FaRandom style={{ color: 'white', fontSize: '20px' }} />
                </div>
            </div>
            <div style={{ width: '100%', height: '80%', display: 'flex', justifyContent: 'center' }}>
                <FilePicker />
            </div>
        </div>
    </div>
}

export default Input;