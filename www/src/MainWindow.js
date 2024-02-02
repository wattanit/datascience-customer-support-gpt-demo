import { useEffect, useState } from 'react';
import Container from 'react-bootstrap/Container';
import Button from 'react-bootstrap/Button';
import Form from 'react-bootstrap/Form';
import './App.css';

function ChatWindow(props) {
    let displayed_messages = props.messages.map((message, index) => {
        return (
            <div key={index} className='chatMessage'>
                <b>{message.user}</b>:<br/>
                {message.text}
            </div>
        )
    });

    return (
        <>
            <div className="chatWindow">
                {displayed_messages}
            </div>
            <div style={{"display": "flex"}}>
                <Form.Control type="text" 
                 value={props.messageText}
                 onChange={(e) => props.setMessageText(e.target.value)}
                 style={{"display": "inline"}}/>
                <Button variant='primary' onClick={()=>{
                    console.log("Add message: " + props.messageText);
                    props.sendMessageHandler();
                    props.setMessageText('');
                }}>Send</Button>
            </div>
        </>
    )
}

function MainWindow() {
    let [messageText, setMessageText] = useState('');
    let [messages, setMessages] = useState([]);

    let newCharHandler = async () => {
        console.log("New char");
        setMessageText('');
        setMessages([]);
    }

    let sendMessageHandler = () => {
        console.log("Add message: " + messageText)
        let message = {
            user: "User",
            text: messageText
        }
        setMessages([...messages, message])
    }

    return (
        <Container>
            <Button variant="outline-success" onClick={newCharHandler}>New chat</Button>
            <ChatWindow sendMessageHandler={sendMessageHandler} 
                messageText={messageText}
                setMessageText={setMessageText}
                messages={messages}
                setMessages={setMessages}
                />
        </Container>
    )
}

export default MainWindow;