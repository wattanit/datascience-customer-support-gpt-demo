import './App.css';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import MainWindow from './MainWindow';

function ChatPage() {
  return (
    <Container fluid>
      <Row>
        <Col>
        Header
        </Col>
      </Row>
      <Row>
        <Col xs={12} md={8} className='mainWindow'>
          <MainWindow/>
        </Col>
        <Col xs={0} md={4}>
          Log window
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

export default ChatPage;