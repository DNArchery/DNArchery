import React from 'react'

const CodonCells = ({ label }) => {
    return <div className='card'>
        <p>{label}</p>
    </div>
}

const Codon = ({ codon }) => {
    return (
        <div>
                <table>
                    {
                        codon.forEach(group => {
                            <tr>
                                {
                                    group.forEach(codon => {
                                        return <td> { codon } </td>
                                    })}
                            </tr>
                        })
                    }
                </table>
        </div>
    )
}

export default Codon;