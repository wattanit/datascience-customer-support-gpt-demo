import './App.css';
import {useState} from 'react';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import Form from 'react-bootstrap/Form';
import Button from 'react-bootstrap/esm/Button';

function LoginPanel(props) {
    let [phoneNumber, setPhoneNumber] = useState('');

    let phoneNumberChangeHandler = (e) => {
        // only update state if the input is a number
        if (isNaN(e.target.value)) {
            return;
        }
        setPhoneNumber(e.target.value);
    }

    let loginHandler = (e) => {
        e.preventDefault();
        console.log("Login with phone number: " + phoneNumber);
        //TODO: call Login API, wait for response
        props.nextPage();
    }

    return (
        <Container className='loginPage'>
            <h2>เข้าสู่ระบบ</h2>
            <Form.Group>
                <Form.Label>หมายเลขโทรศัพท์</Form.Label>
                <Form.Control type='text' 
                    id='phone_number' 
                    name='phone_number'
                    value={phoneNumber}
                    onChange={phoneNumberChangeHandler}/>
                <Form.Text className='text-muted'>หมายเลขที่สมัครใช้บริการ MyMo เช่น 0812345678</Form.Text>
            </Form.Group>
            <Button variant='primary' type='submit' onClick={loginHandler}>เข้าสู่ระบบ</Button>
        </Container>
    )
}

function LoginPage(props) {
  return (
    <Container fluid>
      <Row>
        <Col>
        Header
        </Col>
      </Row>
      <Row>
        <Col>
          <LoginPanel nextPage={props.nextPage}/>
        </Col>
      </Row>
      <Row>
        <Col style={{"textAlign": "center"}}>
            <a href="#" onClick={()=>props.demoPage('demoPageNewUser')}>Create new demo user</a>
        </Col>
      </Row>
      <Row>
        <Col>
          Footer
        </Col>
      </Row>
    </Container>
  );
}

export default LoginPage;