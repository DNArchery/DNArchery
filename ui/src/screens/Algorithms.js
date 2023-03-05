import * as React from 'react'
import Input from '../components/Input/Input';

const Algorithms = () => {
    return <div style={{ flex: 7, display: 'flex' }}>
        <div style={{ flex: 1 }} />
        <Input />
        <div className='output'>
            <p>Algorithms</p>
        </div>
        <div style={{ flex: 1 }} />
    </div>
}

export default Algorithms;
