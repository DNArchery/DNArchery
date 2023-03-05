import React from 'react'

const Lorf = ({ lorf }) => {
    return (
        <div style={{ width: '80%', display: 'flex', flexDirection: 'column' }}>
            <b>Lorf (Longest Open Reading Frame)</b><br/>
                <div className="lorf">
                    {lorf}
                </div>
        </div>
    )
}

export default Lorf;