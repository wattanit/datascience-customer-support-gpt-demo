import './App.css';
import {useState} from 'react';
import ChatPage from './ChatPage';
import LoginPage from './LoginPage';
import DemoPageNewUser from './DemoPageNewUser';

function App() {
  let [pageName, setPageName] = useState('loginPage');

  let nextPage = () => {
    if (pageName === 'loginPage') {
      setPageName('chatPage');
    } else if (pageName === 'chatPage') {
      setPageName('loginPage');
    }else{
      setPageName('loginPage');
    }
  }

  let demoPage = (nextPageName) => {
    setPageName(nextPageName);
  }


  let page;
  if (pageName === 'loginPage') {
    page = <LoginPage nextPage={nextPage} demoPage={demoPage}/>;
  } else if (pageName === 'chatPage') {
    page = <ChatPage nextPage={nextPage}/>;
  } else if (pageName === 'demoPageNewUser') {
    page = <DemoPageNewUser nextPage={nextPage}/>;
  } else {
    page = <div>Not found</div>;
  }

  return (
    <>
      {page}
    </>
  );
}

export default App;
