import React from 'react'

const Protein = ({ protein }) => {
    return (
        <div style={{ width: '80%', display: 'flex', flexDirection: 'column' }}>
            <h4>Protein</h4>
            <div className="protein">
                {protein}
            </div>
        </div>
    )
}

export default Protein;