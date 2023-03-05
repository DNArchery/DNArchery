import * as React from 'react';
import { IoMdSend } from 'react-icons/io';
import FilePicker from '../FilePicker/FilePicker';
import defaults from './defaults'
import './styles.css'

const Input = ({ onSubmit }) => {

    const [input, setInput] = React.useState('');

    return <div className='input' >
        <div style={{ width: '80%', display: 'flex', justifyContent: 'center', flexDirection: 'column' }}>
            <div style={{ width: '120%', display: 'flex', alignItems: 'center', flexDirection: 'row', height: '8%' }}>
                <div className='search'>
                    <input type={'text'} placeholder='Enter DNA sequence' value={input} onInput={(e) => setInput(e.target.value)} />
                    <IoMdSend onClick={() => onSubmit(input)} style={{ color: 'blue' }} />
                </div>
                <h4 onClick={() => setInput(defaults.dna)} style={{ marginLeft: '10px', color: 'blue' }}>GET RANDOM</h4>
            </div>
            <div style={{ width: '100%', height: '80%', display: 'flex', justifyContent: 'center' }}>
                <FilePicker />
            </div>
        </div>
    </div>
}

export default Input;