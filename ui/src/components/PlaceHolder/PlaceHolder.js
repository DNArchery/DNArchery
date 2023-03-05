import * as React from 'react';
import { GiDna2 } from 'react-icons/gi';

const PlaceHolder = () => {
    return <>
        <GiDna2 style={{ fontSize: '15vh', color: 'grey' }} />
        <p style={{ color: 'grey' }}>Nothing to visualize</p>
    </>
}

export default PlaceHolder;
