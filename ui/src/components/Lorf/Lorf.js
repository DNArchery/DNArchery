import React from 'react'

const Lorf = ({ lorf }) => {
    return (
        <div style={{ flex: 1, display: 'flex', flexDirection: 'column' }}>
            <div style={{ flex: 3 }} />
            <b>Lorf (Longest Open Reading Frame)</b><br/>
                <div className="lorf">
                    {lorf}
                </div>
        </div>
    )
}

export default Lorf;