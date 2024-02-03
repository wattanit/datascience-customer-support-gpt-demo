import './App.css';
import {useState} from 'react';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import Form from 'react-bootstrap/Form';
import Button from 'react-bootstrap/esm/Button';

function DemoPageNewUser(props) {
    let [newUser, setNewUser] = useState({phone_number: '', name: ''});

    let createNewUserHandler = ()=>{
        console.log('Create new user: ', newUser);
        //TODO: call create new user API
    }

    return (
        <Container fluid>
            <Row>
                <Col>
                Header
                </Col>
            </Row>
            <Row>
                <Col>
                    <h2>Demo backend panel</h2>
                    <hr/>
                    <h3>Create new user</h3>
                    <Form.Group>
                        <Form.Label>Phone number</Form.Label>
                        <Form.Control type='text' 
                            id='phone_number' 
                            name='phone_number'
                            value={newUser.phone_number} 
                            onChange={(e)=>{
                                if (isNaN(e.target.value)) {
                                    return;
                                }
                                setNewUser({...newUser, phone_number: e.target.value});
                            }}/>   
                    </Form.Group>
                    <Form.Group>
                        <Form.Label>Name</Form.Label>
                        <Form.Control type='text' 
                            id='name' 
                            name='name'
                            value={newUser.name}
                            onChange={(e)=>{
                                setNewUser({...newUser, name: e.target.value});
                            }}/>
                    </Form.Group>
                    <Button variant='primary' type='submit' onClick={createNewUserHandler}>Create user</Button>
                </Col>
            </Row>
            <Row>
                <Col>
                Footer
                </Col>
            </Row>
        </Container>
    )
}

export default DemoPageNewUser;