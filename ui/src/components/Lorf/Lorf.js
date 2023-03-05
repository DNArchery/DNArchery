import React from 'react'

const Lorf = ({ lorf }) => {
    console.log(lorf);
    return (
        <div style={{ flex: 1, display: 'flex', flexDirection: 'column' }}>
            <div style={{ flex: 3 }} />
            <div className='amono-acid'>
            <b>Lorf (Longest Open Reading Frame)</b><br/>
                <div className="lorf">
                    {lorf}
                </div>
            </div>
        </div>
    )
}

export default Lorf;