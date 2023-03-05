import * as React from 'react'
import { useDropzone } from 'react-dropzone'
import { CgFileAdd, CgFile } from 'react-icons/cg';
import './styles.css'

const FilePicker = () => {

    const onDrop = React.useCallback(acceptedFiles => {
        console.log('Got some files')
    }, [])
    const { getRootProps, getInputProps, isDragActive } = useDropzone({ onDrop })

    return <div {...getRootProps()} style={{ flex: 1, display: 'flex', justifyContent: 'center', alignItems: 'center' }}>
        <div {...getInputProps()} className='picker' style={{ borderWidth: isDragActive ? '0px' : '1px', backgroundColor: isDragActive ? 'blue' : 'transparent' }} >
            {isDragActive ? <CgFile color='white' /> : <CgFileAdd color='grey' />}
            <h4 style={{ color: isDragActive ? 'white' : 'grey' }} >{isDragActive ? 'Drop it!' : 'or drop your DNA file here'}</h4>
        </div>
    </div>
}

export default FilePicker;
