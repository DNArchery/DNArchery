import React from 'react'

const AminoAcidCells = ({ label }) => {
    return <div className='card'>
        <p>{label}</p>
    </div>
}

const AminoAcid = ({ aminoAcid }) => {
    return (
        <div style={{ width: '80%', display: 'flex', flexDirection: 'column', marginTop: '50px' }}>
            <div className='amino-acid'>
                {aminoAcid.map((item, index) => {
                    return <AminoAcidCells key={index} label={item} />
                })}
            </div>
        </div>
    )
}

export default AminoAcid;