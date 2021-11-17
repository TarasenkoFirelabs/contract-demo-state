import React from 'react';
import { transitions, positions, Provider as AlertProvider } from 'react-alert'
import MyApp from './_app';

const AlertTemplate = ({ style, message, close }) => (
    <div style={style}>
        <button onClick={close}>X</button>
        <div>View this transaction in <a href={message} target="_blank" rel="noreferrer">explorer</a></div>
    </div>
)

// optional configuration
const options = {
    position: positions.TOP_RIGHT,
    timeout: 5000,
    offset: '30px',
    transition: transitions.FADE
}

export default function Home() {
    return (
        // <AlertProvider template={AlertTemplate} {...options}>
            <MyApp/>
        // </AlertProvider>
    )
}
