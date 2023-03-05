import React from 'react'

const Protein = ({ protein }) => {
    return (
        <div style={{ flex: 1, display: 'flex', flexDirection: 'column'}}>
            <div className='amino-acid'>
            <b>Protein</b><br/>
                <div className="lorf">
                    {protein}
                </div>
            </div>
        </div>
    )
}

export default Protein;