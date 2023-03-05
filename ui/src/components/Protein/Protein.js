import React from 'react'

const Protein = ({ protein }) => {
    return (
        <div style={{ flex: 1, display: 'flex', flexDirection: 'column' }}>
            <div style={{ flex: 3 }} />
            <b>Protein</b><br/>
                <div className="protein">
                    {protein}
            </div>
        </div>
    )
}

export default Protein;